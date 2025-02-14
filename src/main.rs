use axum::{Router, routing::get, routing::post};
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::response::Html;
use axum::http::StatusCode;

use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{FutureExt, SpanKind, Status, TraceContextExt, Tracer};
use opentelemetry::{global, Context};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_stdout::SpanExporter;

use tracing;

mod db;
mod file_storage;

fn init_tracer() -> Result<(), Box<dyn std::error::Error>> {  

    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder().with_tonic().build()?;

    //global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .with_batch_exporter(otlp_exporter)
        .build();
    global::set_tracer_provider(provider);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::fmt::Error> {

    init_tracer();

    let tracer = global::tracer("rust-timelapse-server");

    let database = match db::initalise_database().await {
        Ok(db) => db,
        Err(_) => panic!("Error initialising database")
    };

    // would prefer to bubble up the error, but not sure how
    match file_storage::initialise_file_storage() {
        Ok(_) => (),
        Err(_) => panic!("Could not initialise file storage")
    }
    
    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo/{*camera}", post(post_foo)).with_state(database).layer(DefaultBodyLimit::max(20000000));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[tracing::instrument]
// which calls one of these handlers
async fn root() -> Html<&'static str>{

    let tracer = global::tracer("rust-timelapse-server");

    let mut span = tracer
        .span_builder(format!("Home request"))
        .with_kind(SpanKind::Server)
        .start(&tracer);

    span.set_status(Status::Ok);

    Html("<H1>Hello to the site</H1>")
}

#[tracing::instrument]
async fn post_foo(Path(path): Path<String>, State(database): State<db::Database>, mut multipart: Multipart) -> StatusCode{
    
    // Error handling required here to cope with the payload being too large
    let tracer = global::tracer("rust-timelapse-server");
    global::set_text_map_propagator(TraceContextPropagator::new());

    let mut span = tracer
        .span_builder(format!("post image request"))
        .with_kind(SpanKind::Server)
        .start(&tracer);

    //let cx = Context::current();
    let cx = Context::current_with_span(span);
    let span = cx.span();

    while let Some(field) = multipart.next_field().await.unwrap() {
        //let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        // this was for storing the image in base64 in the database
        // use base64::prelude::*;
        // let data = BASE64_STANDARD.encode(data);
        // let _ = match database.add_entry_base64(&path.as_str(), data).await {
        //     Ok(_) => println!("Success"),
        //     Err(error) => println!("Error {:?}", error)
        // };

        // these are wrong, not sure what the right way to do things
        let path = match file_storage::add_file(&cx, &path.as_str(), data.to_vec())
        {
            Ok(path) => path,
            Err(_) => {
                span.set_status(Status::error("error adding file"));
                return StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        // these are wrong, not sure what the right way to do things
        // if there is an error then we need to delete the file as it's not valid
        let _ = match database.add_entry_vec8(&cx, &path.as_str(), "data".to_string()).await {
            Ok(_) => true,
            Err(_) => {
                span.set_status(Status::error("Error storing entry in db"));
                return StatusCode::INTERNAL_SERVER_ERROR
            }
        };
    }

    StatusCode::OK
}