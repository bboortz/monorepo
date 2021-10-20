use std::error::Error;
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};

#[derive(Debug, StructOpt)]
#[structopt(name = "server", version = "0.1.0", about = "server functionality")]
pub struct ServerCommand {
    /// Specifies the address to listen on
    #[structopt(name = "SERVER_ADDR")]
    pub addr: String,
}

impl ServerCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.async_run()
    }

    #[tokio::main]
    async fn async_run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;

        info!("Listening on {}.", self.addr);

        let mut thread_id = 0;
        // let tcp_server = TcpServer {};
        loop {
            let (stream, client_addr) = listener.accept().await?;
            info!("t{} - Connect: {}", thread_id, client_addr);

            /*
            match listener.accept().await {
                Ok((_socket, addr)) => println!("new client: {:?}", addr),
                Err(e) => println!("couldn't get client: {:?}", e),
            }
            */

            tokio::spawn(async move {
                if let Err(e) = Self::process_stream(thread_id, stream, client_addr).await {
                    error!("Processing Error: {:?}", e);
                }
            });

            thread_id += 1;
        }
    }

    async fn process_stream(
        thread_id: u32,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(), Box<dyn Error>> {
        info!("real main!");
        let mut framed = BytesCodec::new().framed(stream);

        let mut frame_id = 0;
        while let Some(message) = framed.next().await {
            match message {
                Ok(bytes) => info!(
                    "t{} - f{} - {} - bytes: {:?}",
                    thread_id, frame_id, addr, bytes
                ),
                Err(err) => warn!(
                    "t{} - f{} - {} - Socket closed with error: {:?}",
                    thread_id, frame_id, addr, err
                ),
            }

            frame_id += 1;
        }
        info!(
            "t{} - Socket received FIN packet and closed connection",
            thread_id
        );
        Ok(())
    }
}
