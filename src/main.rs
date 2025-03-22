use std::path::Path;

use args::Args;
use clap::Parser;
use config::Config;
use flexi_logger::{Duplicate, FileSpec, Logger};
mod args;
mod config;

fn main() {
    let args = Args::try_parse().unwrap();
    let config_path = args.config;
    let config = Config::load_or_default(Path::new(&config_path)).unwrap();

    Logger::with(config.log_level)
        .log_to_file(FileSpec::default().directory(config.logs_folder))
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();

    log::info!("start");
    for step in 0..10 {
        log::info!("step {}", step);
    }
    log::info!("done");
}
