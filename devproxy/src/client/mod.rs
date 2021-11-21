use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use snafu::{ResultExt, Snafu};
use std::io;
use structopt::StructOpt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to connect to {}: {}", addr, source))]
    Connect { source: io::Error, addr: String },

    #[snafu(display("Unable to write {} bytes to {}: {}", bytes_len, addr, source))]
    WriteBytes {
        source: io::Error,
        addr: String,
        bytes_len: usize,
    },
}

#[derive(Debug, StructOpt)]
pub struct ClientCommand {
    /// Specifies the address to connect
    #[structopt(name = "ADDR")]
    pub addr: String,
}
// const CONNECTION_TIME: u64 = 1;

impl ClientCommand {
    // pub fn run(&self) -> Result<(), Box<(dyn snafu::Error + 'static)>> {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.async_run()
    }

    /*
    pub fn print_type_of<T>(&self, _: &T) {
        println!("{}", std::any::type_name::<T>())
    }

    pub fn get_type_of<T>(&self, _: &T) -> &str {
        std::any::type_name::<T>()
    }
    */

    #[tokio::main]
    // async fn async_run(&self) -> Result<(), Box<(dyn snafu::Error + 'static)>> {
    async fn async_run(&self) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Connecting to addr {} ...", self.addr);

        // let mut stream = TcpStream::connect(&self.addr).await?;
        let mut stream = TcpStream::connect(&self.addr)
            .await
            .context(Connect { addr: &self.addr })?;
        info!("Connected to addr {}!", self.addr);

        // connect with timeout but poor error handling
        /*
        let mut stream = match tokio::time::timeout(
            Duration::from_millis(CONNECTION_TIME),
            tokio::net::TcpStream::connect(&self.addr),
        )
        .await
        {
            Ok(ok) => ok,
            Err(e) => panic!(format!("timeout while connecting to server : {}", e)),
            //error!("timeout while connecting to server : {}", e),
            /*
            Err(e) => Err(error::Error {
                error_type: error::ErrorType::Io(e),
                affected: self.addr.clone(),
                suggestion: String::from(
                    "Verify if the file exists and you have specified the filename correct.",
                ),
            }),
            */
        }
        .expect("Error while connecting to server");
        */

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(900000)
            .map(char::from)
            .collect();
        let rand_bytes = rand_string.as_bytes();

        let result = stream.write(rand_bytes).await.context(WriteBytes {
            addr: &self.addr,
            bytes_len: rand_bytes.len(),
        })?;
        info!("Wrote {} bytes to stream", result);

        // write with timeout but poor error handling
        /*
        // tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;
        match tokio::time::timeout(
            tokio::time::Duration::from_secs(5),
            stream.write(rand_bytes),
        )
        .await?
        {
            Ok(s) => {
                info!("Wrote to stream: result={:?}", s);
            }
            Err(e) => {
                eprintln!("timed out: {:?}", e);
            }
        }
        */

        // write with own error class
        /*
        let result = stream.write(rand_string.as_bytes());
        let result = match result.await {
            Ok(_size) => Ok(()),
            Err(e) => Err(error::Error {
                error_type: error::ErrorType::Io(e),
                affected: self.addr.clone(),
                suggestion: String::from(
                    "Verify the connection to the target and load on the target.",
                ),
            }),
        };
        info!("Wrote to stream: result={:?}", result.is_ok());
        */

        Ok(())
    }
}
