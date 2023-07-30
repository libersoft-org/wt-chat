use wtransport::{ServerConfig, Endpoint};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs::File;
use std::io::Read;

async fn receive_message(recv_stream: &mut (impl AsyncReadExt + Unpin)) -> Result<String, Box<dyn std::error::Error>> {
 let mut buffer = Vec::new();
 recv_stream.read_to_end(&mut buffer).await?;
 let message = String::from_utf8(buffer)?;
 Ok(message)
}

async fn send_message(send_stream: &mut (impl AsyncWriteExt + Unpin), message: &str) -> Result<(), Box<dyn std::error::Error>> {
 send_stream.write_all(message.as_bytes()).await?;
 Ok(())
}

async fn receive_file(recv_stream: &mut (impl AsyncReadExt + Unpin), file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
 let mut file_buffer = Vec::new();
 recv_stream.read_to_end(&mut file_buffer).await?;
 let mut file = File::create(file_path)?;
 file.write_all(&file_buffer)?;
 Ok(())
}

async fn send_file(send_stream: &mut (impl AsyncWriteExt + Unpin), file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
 let mut file = File::open(file_path)?;
 let mut file_buffer = Vec::new();
 file.read_to_end(&mut file_buffer)?;
 send_stream.write_all(&file_buffer).await?;
 Ok(())
}

#[tokio::main]
async fn main() {
 let mut certificate = Vec::new();
 if File::open("path/to/your/certificate.pem").unwrap().read_to_end(&mut certificate).is_err() {
  println!("Failed to read certificate");
  return;
 }

 let config = match ServerConfig::builder()
  .with_bind_address("[::1]:4433".parse().unwrap())
  .with_certificate(certificate)
  .build() {
   Ok(config) => config,
   Err(_) => {
    println!("Failed to create server config");
    return;
   }
  };

 let connection = match Endpoint::server(config).unwrap()
  .accept()
  .await.unwrap()     // Awaits connection
  .await.unwrap()    // Awaits session request
  .accept()  // Accepts request
  .await {   // Awaits ready session
   Ok(connection) => connection,
   Err(_) => {
    println!("Failed to establish connection");
    return;
   }
  };

 let (mut send_stream, mut recv_stream) = match connection.accept_bi().await {
  Ok(stream) => stream.split(),
  Err(_) => {
   println!("Failed to accept bidirectional stream");
   return;
  }
 };

 // Receiving a message
 match receive_message(&mut recv_stream).await {
  Ok(message) => println!("Received message: {}", message),
  Err(_) => println!("Failed to receive message"),
 }

 // Sending a message
 if send_message(&mut send_stream, "Hello from server").await.is_err() {
  println!("Failed to send message");
 }

 // Receiving a file
 if receive_file(&mut recv_stream, "received_file").await.is_err() {
  println!("Failed to receive file");
 }

 // Sending a file
 if send_file(&mut send_stream, "path/to/your/file").await.is_err() {
  println!("Failed to send file");
 }
}
