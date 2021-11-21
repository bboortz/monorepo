use std::net::SocketAddr;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;

//use async_std;
use async_trait::async_trait;
use snafu::Snafu;
use structopt::StructOpt;

/*
use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
// use tokio_stream::StreamExt;
// use tokio_util::codec::{BytesCodec, Decoder};
use futures::FutureExt;
use std::env;
use std::error::Error;
use tokio::io::AsyncWriteExt;
*/

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

#[derive(Debug, Snafu)]
enum ClientError {
    #[snafu(display("Unable to connect to {}: {}", addr, source))]
    Connect { source: io::Error, addr: String },

    #[snafu(display("Unable to write {} bytes to {}: {}", bytes_len, addr, source))]
    WriteBytes {
        source: io::Error,
        addr: String,
        bytes_len: usize,
    },
}

// #[async_trait]
// impl<'ltself> A<'ltself> for PCommand {
impl ProxyCommand {
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // self.async_run()
        let c = ProxyProcess::new(self);
        c.run()
    }
}

struct ProxyProcess {
    command: ProxyCommand,
}

#[async_trait]
trait Process {
    fn new(c: ProxyCommand) -> Self;
    fn run(self) -> Result<(), Box<dyn std::error::Error>>;
    //    async fn async_run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl Process for ProxyProcess {
    fn new(c: ProxyCommand) -> ProxyProcess {
        ProxyProcess { command: c }
    }
    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.async_run()
    }
}

impl ProxyProcess {
    #[tokio::main]
    // async fn main() -> Result<(), Box<dyn Error>> {
    async fn async_run(self) -> Result<(), Box<dyn std::error::Error>> {
        /* let Self {
             downstream_addr,
             upstream_addr,
         } = self;
        */
        // info!("Proxying to {}.", self.command.upstream_addr);
        let listener = TcpListener::bind(&self.command.downstream_addr).await?;
        info!("Listening on {}.", self.command.downstream_addr);
        /*
        let mut t: ProxyTransfer = Transfer::new();
        */
        let mut thread_id = 0;
        while let Ok((inbound, client_addr)) = listener.accept().await {
            info!("t{} - Connect from: {}", thread_id, client_addr);
            let transfer = transfer(
                thread_id,
                inbound,
                client_addr,
                self.command.upstream_addr.clone(),
            )
            .map(|r| {
                if let Err(e) = r {
                    println!("Failed to transfer; error={}", e);
                }
            });
            tokio::spawn(transfer);

            thread_id += 1;
        }
        // let mut t: ProxyTransfer = Transfer::new();
        // let Ok((inbound, _)) = listener.accept().await;
        /*
        tokio::spawn(async move {
            if let Err(e) = transfer(inbound, self.command.upstream_addr).await {
                println!("Processing error: {}", e);
            }
        });
        */

        Ok(())
    }
}

async fn transfer(
    thread_id: u32,
    mut inbound: TcpStream,
    client_addr: SocketAddr,
    proxy_addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        let bytes = io::copy(&mut ri, &mut wo).await?;
        info!("t{} -> {} - bytes: {:?}", thread_id, client_addr, bytes);
        wo.shutdown().await
    };

    let server_to_client = async {
        let bytes = io::copy(&mut ro, &mut wi).await?;
        info!("t{} <- {} - bytes: {:?}", thread_id, client_addr, bytes);
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;
    info!(
        "t{} - Socket received FIN packet and closed connection",
        thread_id
    );

    Ok(())
}

/*
#[async_trait]
pub trait A<'a> {
    fn run(&'a self) -> Result<(), Box<dyn std::error::Error + '_>>;
    async fn async_run(&'a self) -> Result<(), Box<dyn std::error::Error + '_>>;
    async fn transfer(
        &'a self,
        inbound: TcpStream,
        proxy_addr: String,
    ) -> Result<(), Box<dyn Error>>;
}

pub struct PCommand<'a> {
    foo: &'a (A + 'a),
    pub downstream_addr: String,
    pub upstream_addr: String,
}
impl<'a> PCommand<'a> {
    pub fn new(proxyCommand: ProxyCommand) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(PCommand {
            downstream_addr: proxyCommand.downstream_addr,
            upstream_addr: proxyCommand.upstream_addr,
        })
    }
}
*/

/*
struct ProxyTransfer {}

#[async_trait]
trait Transfer {
    fn new() -> Self;
    async fn transfer(
        &mut self,
        mut inbound: TcpStream,
        proxy_addr: String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl Transfer for ProxyTransfer {
    fn new() -> ProxyTransfer {
        ProxyTransfer {}
    }

    // async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    async fn transfer(
        &mut self,
        mut inbound: TcpStream,
        proxy_addr: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut outbound = TcpStream::connect(proxy_addr).await?;

        let (mut ri, mut wi) = inbound.split();
        let (mut ro, mut wo) = outbound.split();

        let client_to_server = async {
            io::copy(&mut ri, &mut wo).await?;
            wo.shutdown().await
        };

        let server_to_client = async {
            io::copy(&mut ro, &mut wi).await?;
            wi.shutdown().await
        };

        tokio::try_join!(client_to_server, server_to_client)?;

        Ok(())
    }
}
*/
