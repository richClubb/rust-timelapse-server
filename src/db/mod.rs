use std::fmt::Error;
use influxdb::Client;
use chrono;

pub fn initalise_database() -> Result<Client, Error>{

    let client = Client::new("http://localhost:8086", "test");

    // add in a query to check the db and return an error if it can't get anything

    return Ok(client);
}

pub fn add_entry() -> Result<(), &'static str>{

    return Err("Not implemented yet");
}