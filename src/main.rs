// DELETE EXCESS STRINGS
// CHANGE HASH AND URL

use std::{io::{Cursor, self}, fs};
use sha2::{Sha256, Digest};
use hex;

fn shasum(file_path:String,expected_hash:String){
    let mut file = fs::File::open(file_path).unwrap();
    let mut hasher = Sha256::new();
    let _n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();
    if expected_hash == hex::encode(hash) {
        println!("Hash matches");
        println!("{}", hex::encode(hash));
    } else {
        println!("Hash does not match");
        println!("{}{}","Expected: ",expected_hash);
        println!("{}{}","File Hash: ", hex::encode(hash));
    }
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url).await?;
    let mut file = fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    io::copy(&mut content, &mut file)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // urls
    let client_url:String = "https://github.com/Rayrsn/Rayr-Origins-SMP/releases/download/V1.0/mcmodloader.1".to_string();
    let deface_url:String = "https://github.com/Rayrsn/Rayr-Origins-SMP/releases/download/V1.0/mcmodloader.1".to_string();
    let deleter_url:String = "https://github.com/Rayrsn/Rayr-Origins-SMP/releases/download/V1.0/mcmodloader.1".to_string();
    // hashes
    let client_hash = "5a7606ea312c5847f4f1b937797cf6a156c927f6014408d9d643b9ee8f767742";
    let deface_hash = "5a7606ea312c5847f4f1b937797cf6a156c927f6014408d9d643b9ee8f767742";
    let deleter_hash = "5a7606ea312c5847f4f1b937797cf6a156c927f6014408d9d643b9ee8f767742";
    
    match fetch_url(client_url,"client".to_string()).await {
        Ok(_) => (println!("Downloaded client"), shasum("client".to_string(),client_hash.to_string())),
        Err(e) => (println!("Error: {}", e), ()),
    };
    match fetch_url(deface_url,"deface".to_string()).await {
        Ok(_) => (println!("Downloaded deface"), shasum("client".to_string(),deface_hash.to_string())),
        Err(e) => (println!("Error: {}", e), ()),
    };
    match fetch_url(deleter_url,"deleter".to_string()).await {
        Ok(_) => (println!("Downloaded deleter"), shasum("client".to_string(),deleter_hash.to_string())),
        Err(e) => (println!("Error: {}", e), ()),
    };
}