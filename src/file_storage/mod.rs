use chrono::{DateTime, Utc};

use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::{global, trace::Tracer, propagation::Extractor};
use opentelemetry::trace::{SpanKind, Status};

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl Extractor for MetadataMap<'_> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

pub fn initialise_file_storage() -> Result<(), Box<dyn std::error::Error>>{

    let base_path = std::env::var("FILE_STORE_PATH")?;

    std::fs::create_dir_all(base_path)?;

    Ok(())
}

pub fn add_file(camera : &str, data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>>{
    
    let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
    let tracer = global::tracer("rust-timelapse-server");

    let mut span = tracer
        .span_builder(format!("add file to disk"))
        .with_kind(SpanKind::Server)
        .start_with_context(&tracer, &parent_cx);

    let utc: DateTime<Utc> = Utc::now();

    let base_path = std::env::var("FILE_STORE_PATH")?;

    let path = format!("{}/{}/{}-{}.jpg", base_path, camera, camera, utc);

    match std::fs::write(&path, data) {
        Ok(_) => (),
        Err(error) => {
            span.set_status(Status::Error{ description: error.to_string().into()});
            return Err(error.to_string().into());
        }
    };

    span.set_status(Status::Ok);
    Ok(path)
}

pub fn delete_file(_: &str) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}