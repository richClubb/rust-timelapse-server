use axum::{Router, routing::get, routing::post, extract::Path};

#[tokio::main]
async fn main() {
    // build our application with a single route
    // our router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo/{*filename}", post(post_foo))
        .route("/foo/bar", get(foo_bar));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// which calls one of these handlers
async fn root() {println!("root")}
async fn post_foo(Path(path): Path<String>, test: String) {
    println!("{:?} {:?}", path, test)
}
async fn foo_bar() {}