use chrono::{DateTime, Utc};


pub fn initialise_file_storage() -> Result<(), Box<dyn std::error::Error>>{

    let base_path = std::env::var("FILE_STORE_PATH")?;

    std::fs::create_dir_all(base_path)?;

    Ok(())
}

pub fn add_file(camera : &str, data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>>{
    
    let utc: DateTime<Utc> = Utc::now();

    let base_path = std::env::var("FILE_STORE_PATH")?;

    let path = format!("{}/{}/{}-{}.jpg", base_path, camera, camera, utc);

    println!("{}", &path);
    std::fs::write(&path, data)?;

    println!("Added files");
    Ok(path)
}

pub fn delete_file(_: &str) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}