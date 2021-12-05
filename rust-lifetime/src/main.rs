use async_trait::async_trait;

#[derive(Debug)]
pub struct ProxyTransfer {}

#[async_trait]
trait Transfer {
    fn new() -> Self;
    async fn transfer(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl Transfer for ProxyTransfer {
    fn new() -> ProxyTransfer {
        ProxyTransfer {}
    }

    async fn transfer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Proxying via ProxyTransfer");

        Ok(())
    }
}

impl Drop for ProxyTransfer{
    fn drop(&mut self) {
        println!("> Dropping ProxyTransfer");
    }
}

#[derive(Debug)]
pub struct ProxyCommand {
    pub downstream_addr: String,
    pub upstream_addr: String,
}

trait Command {
    fn new(downstream_addr: String, upstream_addr: String) -> Self;
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error + '_>>;
}

impl Command for ProxyCommand {
    fn new(downstream_addr: String, upstream_addr: String) -> ProxyCommand {
        ProxyCommand {
            downstream_addr: downstream_addr,
            upstream_addr: upstream_addr,
        }
    }

    fn run(&mut self) -> Result<(), Box<dyn std::error::Error + '_>> {
        self.async_run()

    }
}

impl Drop for ProxyCommand{
    fn drop(&mut self) {
        println!("> Dropping {}", self.downstream_addr);
    }
}

impl ProxyCommand {
    #[tokio::main]
    // async fn main() -> Result<(), Box<dyn Error>> {
    async fn async_run(&mut self) -> Result<(), Box<dyn std::error::Error + '_>> {
        println!("Proxying to {}.", self.upstream_addr);
        println!("Listening on {}.", self.downstream_addr);

        // let transfer = self.transfer();

        let mut t: ProxyTransfer = Transfer::new();
        tokio::spawn(async move {
            if let Err(e) = t.transfer().await {
                println!("Processing Error: {:?}", e);
            }
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        Ok(())
    }
}


// fn main() {
fn main() {
    let downstream_addr = String::from("la");
    let upstream_addr = String::from("hu");
    let mut c: ProxyCommand = Command::new(downstream_addr, upstream_addr);
    /*
    let mut pc: dyn ProxyCommand + 'static = ProxyCommand {
        downstream_addr: String::from("la"),
        upstream_addr: String::from("hu"),
    };
    */
    c.run();
}
