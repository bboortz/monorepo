use std::error::Error;
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{BytesCodec, Decoder};

#[derive(Debug, StructOpt)]
#[structopt(name = "proxy", version = "0.1.0", about = "proxy functionality")]
pub struct ProxyCommand {
    /// Specifies the downstream_address to listen on
    #[structopt(name = "SERVER_ADDR")]
    pub downstream_addr: String,

    /// Specifies the downstream_address to connect
    #[structopt(name = "CLIENT_ADDR")]
    pub upstream_addr: String,
}

impl ProxyCommand {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.async_run()
    }

    #[tokio::main]
    async fn async_run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.downstream_addr).await?;

        info!("Listening on {}.", self.downstream_addr);

        let mut thread_id = 0;
        // let tcp_proxy = TcpProxy {};
        loop {
            let (stream, downstream_client_addr) = listener.accept().await?;
            info!("t{} - Connect: {}", thread_id, downstream_client_addr);

            /*
            match listener.accept().await {
                Ok((_socket, downstream_addr)) => println!("new client: {:?}", downstream_addr),
                Err(e) => println!("couldn't get client: {:?}", e),
            }
            */

            tokio::spawn(async move {
                if let Err(e) =
                    Self::process_stream(thread_id, stream, downstream_client_addr).await
                {
                    error!("Processing Error: {:?}", e);
                }
            });

            thread_id += 1;
        }
    }

    async fn process_stream(
        thread_id: u32,
        stream: TcpStream,
        downstream_addr: SocketAddr,
    ) -> Result<(), Box<dyn Error>> {
        info!("real main!");
        let mut framed = BytesCodec::new().framed(stream);

        let mut frame_id = 0;
        while let Some(message) = framed.next().await {
            match message {
                Ok(bytes) => info!(
                    "t{} - f{} - {} - bytes: {:?}",
                    thread_id, frame_id, downstream_addr, bytes
                ),
                Err(err) => warn!(
                    "t{} - f{} - {} - Socket closed with error: {:?}",
                    thread_id, frame_id, downstream_addr, err
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
