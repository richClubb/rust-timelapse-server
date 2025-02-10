use influxdb2::Client;
use influxdb2::models::DataPoint;

use futures::prelude::*;

#[derive(Clone)]
pub struct Database {
    client: Client
}

impl Database {

    #[allow(dead_code)]
    pub async fn add_entry_base64(&self, camera: &str, data: String) -> Result<(), Box<dyn std::error::Error>> {

        let data = vec![DataPoint::builder("pictures")
                                .tag("camera", camera)
                                .field("image", data)
                                .build()?];

        self.client.write("pictures", stream::iter(data)).await?;

        Ok(())
    }

    pub async fn add_entry_vec8(&self, camera: &str, path: String) -> Result<(), Box<dyn std::error::Error>> {

        let data = vec![DataPoint::builder("pictures")
                                .tag("camera", camera)
                                .field("image", path)
                                .build()?];

        self.client.write("pictures", stream::iter(data)).await?;

        Ok(())
    }
}


pub async fn initalise_database() -> Result<Database, &'static str>{

    let client = Client::new("http://localhost:8086", "org", "MyInitialAdminToken0==");

    let database = Database{client: client};

    return Ok(database);
}