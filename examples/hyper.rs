use http_body_util::{BodyExt, Empty};
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // This is where we will setup our HTTP client requests.
    // Parse our URL...
    let uri = "http://httpbin.org/ip".parse::<hyper::Uri>()?;
    // Get the host and the port
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(80);
    let address = format!("{}:{}", host, port);

    println!("{}", host);
    println!("{}", port);

    // Open a TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;
    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.
    let io = TokioIo::new(stream);
    // Perform a TCP handshake
    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    // Spawn a task to poll the connection, driving the HTTP state
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    // The authority of our URL will be the hostname of the httpbin remote
    let authority = uri.authority().unwrap().clone();
    // Create an HTTP request with an empty body and a HOST header
    // let req = Request::builder()
    //     .method("GET")
    //     .uri(uri)
    //     .header(hyper::header::HOST, authority.to_string())
    //     .body(Empty::<Bytes>::new())?;

    let req = Request::get(uri)
        .header(hyper::header::HOST, authority.to_string())
        .body(Empty::<Bytes>::new())?;

    // Await the response...
    let mut res = sender.send_request(req).await?;
    let mut response_body = String::from("");
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(bytes) = frame.data_ref() {
            response_body = String::from_utf8(bytes.to_vec()).unwrap();
        }
    }

    println!("Response status: {:?}", res.status());
    println!("Response headrers: {:?}", res.headers());
    println!("Response body: {:?}", response_body);

    Ok(())
}
