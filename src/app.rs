use crate::Cli;
use crate::models::Method;
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
        self.client.set_timeout(self.cli.timeout);
        let request = match self.cli.x {
            Method::GET => self.client.get(&self.cli.url),
            Method::DELETE => self.client.delete(&self.cli.url),
            Method::POST => self.client.post(&self.cli.url),
            Method::PUT => self.client.put(&self.cli.url),
            Method::HEAD => self.client.head(&self.cli.url),
            Method::OPTIONS => self.client.options(&self.cli.url),
            Method::PATCH => self.client.patch(&self.cli.url),
            Method::TRACE => self.client.trace(&self.cli.url),
            _ => {
                return Err(anyhow::anyhow!("Unsupported method"));
            }
        };
        for header in self.cli.headers.iter() {
            let header = header.split(':').collect::<Vec<&str>>();
            if header.len() != 2 {
                return Err(anyhow::anyhow!("Invalid header format"));
            }
            request
                .borrow_mut()
                .set(header[0].to_string(), header[1].to_string());
        }
        if self.cli.data.is_some() {
            request
                .borrow_mut()
                .set_body(self.cli.data.as_ref().unwrap().as_bytes());
        }
        if self.cli.file.is_some() {
            let file = std::fs::read(self.cli.file.as_ref().unwrap())?;
            request.borrow_mut().set_body(&file);
        }
        let response = self.client.execute()?;
        println!("{}", String::from_utf8_lossy(&response.body));
        Ok(())
    }
}
