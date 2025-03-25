use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::stream_handler::StreamHandler;

pub struct StreamManager {
    streamlink_cli: String,
    stream_handlers: Vec<StreamHandler>,
    ripping: Arc<AtomicBool>,
}

impl StreamManager {
    pub fn new(stream_urls: Vec<String>, streamlink_cli: String) -> Self {
        let mut stream_manager = Self {
            streamlink_cli,
            stream_handlers: vec![],
            ripping: Arc::from(AtomicBool::new(false)),
        };

        for url in stream_urls {
            let stream_handler = StreamHandler::new(stream_manager.streamlink_cli.clone(), url);
            log::debug!("Initializing Stream Handler: {stream_handler:?}.");
            stream_manager.stream_handlers.push(stream_handler);
        }

        stream_manager
    }

    /// Blocking, for now.
    pub fn do_ripping(&mut self) {
        self.ripping.store(true, Ordering::Release);
        for stream_handler in &mut self.stream_handlers {
            stream_handler.start();
        }

        while self.ripping.load(Ordering::Acquire) {
            for stream_handler in &mut self.stream_handlers {
                let process = stream_handler
                    .process_mut()
                    .as_mut()
                    .expect("process should be running at this point");

                // TODO: process stdout & stderr. Use async?

                match process.try_wait() {
                    Ok(None) => continue,
                    Ok(Some(status)) => log::warn!(
                        "Ripper of `{}` exited with: {status}.",
                        stream_handler.stream_url()
                    ),
                    Err(e) => log::error!("Error attempting to wait: {e}."),
                }

                stream_handler.start();
            }

            // std::hint::spin_loop();
            // std::thread::yield_now();
            std::thread::park_timeout(Duration::from_secs(15)); // TODO: use some human way to rate limit requests to check the availability of streams.
        }

        log::debug!("Main ripping loop done.")
    }

    pub fn stop(&mut self) {
        for stream_handler in &mut self.stream_handlers {
            stream_handler.terminate();
        }
    }

    pub fn ripping(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.ripping)
    }
}
