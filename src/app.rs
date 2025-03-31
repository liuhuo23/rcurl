use crate::Cli;
#[cfg(feature = "reqwest")]
use crate::build_request;
use crate::error::RcurlError;
use anyhow::Result;
use log::{debug, trace};
use reqwest::blocking::Client;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
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
    #[cfg(feature = "reqwest")]
    pub fn run(&self) -> Result<()> {
        let request = build_request(
            &self.client,
            self.cli.x,
            &self.cli.url,
            Some(&self.cli.headers),
            None,
            self.cli.timeout,
        )?;
        let mut retry = self.cli.retry;
        let mut last_error = None;
        let mut response = None;
        while retry > 0 {
            debug!("retry: {}", retry);
            let request = match request.try_clone() {
                Some(r) => r,
                None => {
                    panic!("request not copy");
                }
            };
            match request.send() {
                Ok(res) => {
                    response = Some(res);
                    break;
                }
                Err(e) => {
                    last_error = Some(e);
                    retry -= 1;
                    if retry == 0 {
                        return Err(RcurlError::RequestError(last_error.unwrap()).into());
                    }
                    sleep(Duration::from_secs(self.cli.interval));
                }
            }
        }
        let response = match response {
            Some(res) => res,
            None => return Err(RcurlError::RequestError(last_error.unwrap()).into()),
        };
        let status = response.status();
        debug!("status: {}", status);
        trace!("status: {}", status);
        if let Some(out) = &self.cli.out {
            let mut file = std::fs::File::create(out)?;
            let body = response.text()?;
            file.write(body.as_bytes())?;
        } else {
            println!("{}", response.text()?);
        }
        Ok(())
    }

    #[cfg(feature = "beta")]
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
