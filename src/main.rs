use axum::{Router, routing::get, routing::post};
use axum::extract::{DefaultBodyLimit, Multipart, Path, State};
use axum::response::Html;

mod db;
mod file_storage;

#[tokio::main]
async fn main() -> Result<(), std::fmt::Error> {
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

// which calls one of these handlers
async fn root() -> Html<&'static str>{
    Html("<H1>Hello to the site</H1>")
}

async fn post_foo(Path(path): Path<String>, State(database): State<db::Database>, mut multipart: Multipart) {
    // println!("Received post request for {}", &path.as_str());
    // println!("Entire form: {:?}", multipart);
    // Error handling required here to cope with the payload being too large
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
        let path = file_storage::add_file(&path.as_str(), data.to_vec())?;

        // these are wrong, not sure what the right way to do things
        // if there is an error then we need to delete the file as it's not valid
        let _ = match database.add_entry_vec8(&path.as_str(), "data".to_string()).await {
            Ok(_) => println!("Success"),
            Err(error) => {
                file_storage::delete_file("");
                println!("Error {:?}", error)
            }
        };
    }
}