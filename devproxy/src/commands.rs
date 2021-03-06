use crate::client;
use crate::proxy;
use crate::server;
use std::fmt::Debug;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "devproxy",
    version = "0.1.0",
    about = "A proxy for development purposes",
    author = "Benjamin Boortz <benjamn dot boortz at mailbox dot org>"
)]
pub enum CommandsFassade {
    #[structopt(name = "server", version = "0.1.0", about = "server functionality")]
    ServerCommand(server::ServerCommand),
    #[structopt(name = "client", version = "0.1.0", about = "client functionality")]
    ClientCommand(client::ClientCommand),
    #[structopt(name = "proxy", version = "0.1.0", about = "proxy functionality")]
    ProxyCommand(proxy::ProxyCommand),
}

impl CommandsFassade {
    /*
      pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
          let command = client::ClientCommand {
              addr: String::from("127.0.0.1:8090"),
          };
          let cmd_fassade = CommandsFassade::ClientCommand(command);
          Ok(cmd_fassade)
      }
    */

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        use crate::commands::CommandsFassade::ClientCommand;
        use crate::commands::CommandsFassade::ProxyCommand;
        use crate::commands::CommandsFassade::ServerCommand;
        match self {
            ClientCommand(command) => {
                info!("running command: {:?} ...", command);
                command.run()?;
            }
            ServerCommand(command) => {
                info!("running command: {:?} ...", command);
                command.run()?;
            }
            ProxyCommand(command) => {
                info!("running command: {:?} ...", command);
                command.run()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::client;
    use crate::commands;

    /*
    #[test]
    fn test_commandsfassade_grep_ok() {
        let grep_command = grep::ClientCommand {
            pattern: String::from("foobar"),
            filename: PathBuf::from(r"./tests/test.file"),
            case_insensitive: true,
        };
        let cmd_fassade = commands::CommandsFassade::GrepCommand(grep_command);
        cmd_fassade.print();
        let _result = cmd_fassade.run().unwrap();
        assert_eq!((), _result);
    }
    */

    #[test]
    fn test_commandsfassade_grep_file_not_found() {
        let command = client::ClientCommand {
            addr: String::from("127.0.0.1:8090"),
        };
        let cmd_fassade = commands::CommandsFassade::ClientCommand(command);

        let _result = cmd_fassade.run().unwrap_err();
        let err = 8;
        assert!(matches!(err, _result));
    }
}
