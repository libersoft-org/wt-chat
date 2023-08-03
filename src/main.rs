use anyhow::Result;
use wtransport::endpoint::IncomingSession;
use wtransport::tls::Certificate;
use wtransport::Endpoint;
use wtransport::ServerConfig;

#[tokio::main]
async fn main() -> Result<()> {
 let port = "4433";
 let config = ServerConfig::builder()
  .with_bind_address("[::1]:4433".parse()?)
  .with_certificate(Certificate::load("cert.pem", "key.pem")?)
  .keep_alive_interval(Some(std::time::Duration::from_secs(3)))
  .build();
 let server = Endpoint::server(config)?;
 println!("Server is running on port {port} ...");
 server.accept().await;
 let incoming_session = server.accept().await;
 tokio::spawn(handle_connection(incoming_session));
 Ok(())
}

async fn handle_connection(incoming_session: IncomingSession) {
 let result = handle_connection_impl(incoming_session).await;
 println!("{:?}", result);
}

async fn handle_connection_impl(incoming_session: IncomingSession) -> Result<()> {
 let mut buffer = vec![0; 65536].into_boxed_slice();
 println!("Waiting for session request...");
 let session_request = incoming_session.await?;
 println!("New session: Authority: '{}', Path: '{}'", session_request.authority(), session_request.path());
 let connection = session_request.accept().await?;
 println!("Waiting for data from client...");
 loop {
  tokio::select! {
   stream = connection.accept_bi() => {
    let mut stream = stream?;
    println!("Accepted BI stream");
    let bytes_read = match stream.1.read(&mut buffer).await? {
     Some(bytes_read) => bytes_read,
     None => continue,
    };
    let str_data = std::str::from_utf8(&buffer[..bytes_read])?;
    println!("Received (bi) '{str_data}' from client");
    stream.0.write_all(b"ACK").await?;
   }
   stream = connection.accept_uni() => {
    let mut stream = stream?;
    println!("Accepted UNI stream");
    let bytes_read = match stream.read(&mut buffer).await? {
     Some(bytes_read) => bytes_read,
     None => continue,
    };
    let str_data = std::str::from_utf8(&buffer[..bytes_read])?;
    println!("Received (uni) '{str_data}' from client");
    let mut stream = connection.open_uni().await?.await?;
    stream.write_all(b"ACK").await?;
   }
   dgram = connection.receive_datagram() => {
    let dgram = dgram?;
    let str_data = std::str::from_utf8(&dgram)?;
    println!("Received (dgram) '{str_data}' from client");
    connection.send_datagram(b"ACK")?;
   }
  }
 }
}
