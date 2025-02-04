use axum::{Router, routing::get, routing::post};
use axum::extract::{DefaultBodyLimit, Multipart, Path};

#[tokio::main]
async fn main() {
    // build our application with a single route
    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo/{*filename}", post(post_foo)).layer(DefaultBodyLimit::max(2000000))
        .route("/foo/bar", get(foo_bar));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// which calls one of these handlers
async fn root() {println!("root")}
async fn post_foo(Path(path): Path<String>, mut multipart: Multipart) {
    println!("Received post request for {}", path);
    println!("Entire form: {:?}", multipart);
    // Error handling required here to cope with the payload being too large
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {:?} bytes", name, data.len());
    }
}
async fn foo_bar() {}