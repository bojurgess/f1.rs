use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct Server {
    pub socket: UdpSocket,
}

impl Server {
    pub async fn new<T: ToSocketAddrs>(addr: T) -> std::io::Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self { socket })
    }

    pub async fn listen(&self) -> std::io::Result<()> {
        let mut buf = vec![0; 2048];
        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;

            println!("Received {} bytes from {}", len, addr);
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr: String = std::env::args()
        .nth(1)
        // fallback to loopback addr.
        .unwrap_or_else(|| "127.0.0.1:20777".to_string());
    let server = Server::new(&addr).await?;

    println!("Listening on {addr}");
    server.listen().await?;
    Ok(())
}
