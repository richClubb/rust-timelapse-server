use chrono::{DateTime, Utc};

use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::Context;
use opentelemetry::{global, trace::Tracer};
use opentelemetry::trace::{SpanKind, Status};

pub fn initialise_file_storage() -> Result<(), Box<dyn std::error::Error>>{

    let base_path = std::env::var("FILE_STORE_PATH")?;

    std::fs::create_dir_all(base_path)?;

    Ok(())
}

pub fn add_file(camera : &str, data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>>{
    
    let parent_cx = Context::current();
    let tracer = global::tracer("rust-timelapse-server");

    let mut span = tracer
        .span_builder(format!("add file to disk"))
        .with_kind(SpanKind::Server)
        .start_with_context(&tracer, &parent_cx);

    let utc: DateTime<Utc> = Utc::now();

    let base_path = std::env::var("FILE_STORE_PATH")?;

    let path = format!("{}/{}/{}-{}.jpg", base_path, camera, camera, utc);

    std::fs::write(&path, data)?;

    span.set_status(Status::Ok);
    Ok(path)
}

pub fn delete_file(_: &str) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}