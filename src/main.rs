use wtransport::{Endpoint, ServerConfig, Result, Stream};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::path::Path;
use rustls::internal::pemfile::{certs, rsa_private_keys};
use rustls::{NoClientAuth, ServerConfig as RustlsServerConfig};
use std::fs::File as StdFile;
use std::io::BufReader;

#[tokio::main]
async fn main() -> Result<()> {
 // Load public and private keys
 let cert_file = &mut BufReader::new(StdFile::open("cert.pem").unwrap());
 let key_file = &mut BufReader::new(StdFile::open("key.pem").unwrap());
 let cert = certs(cert_file).unwrap();
 let mut keys = rsa_private_keys(key_file).unwrap();

 let mut config = RustlsServerConfig::new(NoClientAuth::new());
 config.set_single_cert(cert, keys.remove(0)).unwrap();

 let config = ServerConfig::builder()
  .with_bind_address("[::]:4433".parse()?)
  .with_tls_config(config)
  .build();

 let connection = Endpoint::server(config)?
  .accept()
  .await     // Awaits connection
  .await?    // Awaits session request
  .accept()  // Accepts request
  .await?;   // Awaits ready session

 // Echo messages
 let mut send_stream = connection.create_send_stream().await?;
 let mut receive_stream = connection.accept_receive_stream().await?;
 let mut buffer = vec![0; 1024];
 let n = receive_stream.read(&mut buffer).await?;
 send_stream.write_all(&buffer[..n]).await?;

 // File save
 let mut bi_stream = connection.accept_bi().await?;
 let mut file_name = String::new();
 bi_stream.read_to_string(&mut file_name).await?;
 let mut file = File::create(Path::new(&file_name)).await?;
 let n = bi_stream.read(&mut buffer).await?;
 file.write_all(&buffer[..n]).await?;

 Ok(())
}
