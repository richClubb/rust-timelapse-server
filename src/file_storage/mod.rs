use chrono::{DateTime, Utc};

use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::Context;
use opentelemetry::{global, trace::Tracer};
use opentelemetry::trace::{SpanKind, Status};

use tracing::{info, error};

#[tracing::instrument]
pub fn initialise_file_storage() -> Result<(), Box<dyn std::error::Error>>{

    let base_path = std::env::var("FILE_STORE_PATH")?;

    std::fs::create_dir_all(base_path)?;

    Ok(())
}

#[tracing::instrument]
pub fn add_file(parent_cx: &opentelemetry::Context, camera : &str, data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>>{
            
    let tracer = global::tracer("rust-timelapse-server");

    let mut span = tracer
        .span_builder(format!("add file to disk"))
        .with_kind(SpanKind::Server)
        .start_with_context(&tracer, &parent_cx);

    info!("inside add_file!");

    let utc: DateTime<Utc> = Utc::now();

    let base_path = std::env::var("FILE_STORE_PATH")?;

    let path = format!("{}/{}/{}-{}.jpg", base_path, camera, camera, utc);

    match std::fs::write(&path, data) {
        Ok(_) => (),
        Err(error) => {
            error!("Failed to add file to disk");
            span.set_status(Status::Error{ description: error.to_string().into()});
            return Err(error.to_string().into());
        }
    };

    info!("Successfully added file");
    span.set_status(Status::Ok);
    Ok(path)
}

#[tracing::instrument]
pub fn delete_file(_: &str) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}