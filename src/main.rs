use anyhow::Result;
use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;
use rcurl::{App, Cli};
fn main() -> Result<()> {
    let cli = Cli::parse();
    // 初始化日志
    Builder::new()
        .filter_level(if cli.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init();
    let app = App::new(cli);
    app.run()?;
    Ok(())
}
