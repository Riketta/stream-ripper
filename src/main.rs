use args::Args;
use clap::Parser;
use config::Config;
use flexi_logger::{AdaptiveFormat, Duplicate, FileSpec, Logger};
use std::{path::Path, sync::atomic::Ordering, thread};
use stream_ripper::StreamManager;
mod args;
mod config;

fn main() {
    let args = Args::try_parse().unwrap();
    let config_path = args.config;
    let config = Config::load_or_default(Path::new(&config_path)).unwrap();

    Logger::with(config.log_level)
        .write_mode(flexi_logger::WriteMode::BufferAndFlush)
        .log_to_file(
            FileSpec::default()
                .use_timestamp(true)
                .directory(config.logs_folder),
        )
        .format_for_files(flexi_logger::detailed_format)
        .format_for_writer(flexi_logger::detailed_format)
        .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
        .adaptive_format_for_stdout(AdaptiveFormat::Detailed)
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();

    if config.stream_urls.is_empty() {
        log::error!("No stream URLs to rip has been specified!");
        return;
    }

    log::info!("Initializing Stream Manager.");
    let mut stream_manager = StreamManager::new(config.stream_urls, config.streamlink_cli);

    let current_thread = thread::current();
    let ripping = stream_manager.ripping();
    ctrlc::set_handler(move || {
        // <https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/signal#remarks>.
        log::debug!("SIGINT received, termingating."); // Is it thread-safe in Windows?
        ripping
            .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
            .unwrap();
        current_thread.unpark();
    })
    .expect("error setting Ctrl-C handler");

    log::info!("Starting stream ripping.");
    stream_manager.do_ripping();
    log::info!("Termingating stream rippers.");
    stream_manager.stop();
}
