use crate::Cli;
use crate::models::client::Client;
use anyhow::Result;
pub struct App {
    cli: Cli,
    client: Client,
}

impl App {
    pub fn new(cli: Cli) -> Self {
        Self {
            cli,
            client: Client::new(),
        }
    }
    pub fn run(&mut self) -> Result<()> {
        let request = self.client.get(&self.cli.url);
        for header in self.cli.headers.iter() {
            let header = header.split(':').collect::<Vec<&str>>();
            if header.len() != 2 {
                return Err(anyhow::anyhow!("Invalid header format"));
            }
            request.set(header[0].to_string(), header[1].to_string());
        }
        let mut response = self.client.execute()?;
        println!("{}", String::from_utf8_lossy(&response.body));
        Ok(())
    }
}
