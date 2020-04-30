use std::error::Error;

use tokio::fs::File;
use tokio::prelude::*; // for write_all()

async fn file_create() -> std::io::Result<()> {
let mut file = File::create("foo.txt").await?;
file.write_all(b"hello, world!").await?;
Ok(())
}

async fn file_open() -> std::io::Result<()> {
let mut file = File::open("foo.txt").await?;
let mut contents = vec![];
file.read_to_end(&mut contents).await?;
println!("len = {}", contents.len());Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    file_create().await?;
    file_open().await?;
    Ok(())
}