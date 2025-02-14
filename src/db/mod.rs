use influxdb2::Client;
use influxdb2::models::DataPoint;

use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::Context;
use opentelemetry::{global, trace::Tracer};
use opentelemetry::trace::{SpanKind, Status};

use futures::prelude::*;

#[derive(Clone, Debug)]
pub struct Database {
    client: Client
}

impl Database {

    #[allow(dead_code)]
    #[tracing::instrument]
    pub async fn add_entry_base64(&self, camera: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {

        let data = vec![DataPoint::builder("pictures")
                                .tag("camera", camera)
                                .field("image", data)
                                .build()?];

        self.client.write("pictures", stream::iter(data)).await?;

        Ok(())
    }

    #[allow(dead_code)]
    #[tracing::instrument]
    pub async fn add_entry_vec8(&self, parent_cx: &opentelemetry::Context, camera: &str, path: String) -> Result<(), Box<dyn std::error::Error>> {

        let tracer = global::tracer("rust-timelapse-server");

        let mut span = tracer
            .span_builder(format!("add to db vec8"))
            .with_kind(SpanKind::Server)
            .start_with_context(&tracer, &parent_cx);

        let data = vec![DataPoint::builder("pictures")
                                .tag("camera", camera)
                                .field("image", path)
                                .build()?];

        self.client.write("pictures", stream::iter(data)).await?;
        
        span.set_status(Status::Ok);

        Ok(())
    }
}

#[tracing::instrument]
pub async fn initalise_database() -> Result<Database, &'static str>{

    let client = Client::new("http://localhost:8086", "org", "MyInitialAdminToken0==");

    let database = Database{client: client};

    return Ok(database);
}