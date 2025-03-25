use std::{
    process::{Child, Command},
    time::SystemTime,
};

use crate::{cli_args::CliArgs, metadata_variables::MetadataVariables};

#[derive(Debug, Default)]
pub struct StreamHandler {
    raw_streamlink_cli: String,
    stream_url: String,
    process: Option<Child>,
    started_at: Option<SystemTime>,
}

impl Drop for StreamHandler {
    fn drop(&mut self) {
        log::debug!("Dropping ripper of stream `{}`.", self.stream_url());
        self.terminate();
    }
}

impl StreamHandler {
    pub fn new(raw_streamlink_cli: String, stream_url: String) -> Self {
        Self {
            raw_streamlink_cli,
            stream_url,
            process: None,
            started_at: None,
        }
    }

    pub fn start(&mut self) {
        // TODO: kill if already running?

        let raw_args = MetadataVariables::augment_cli(&self.raw_streamlink_cli, &self.stream_url);
        let args = raw_args.split_as_args();
        let mut args = args.into_iter();

        let (executable, args) = (
            args.next().expect(
                "expected to split CLI string with executable and at least one argument (URL)",
            ),
            args.into_iter(),
        );

        let mut command = Command::new(executable);
        command.args(args);
        log::debug!("Started ripper process with args: \"{raw_args}\".");
        let process = command.spawn().unwrap();
        self.started_at = Some(SystemTime::now());
        // TODO: handle pipes.

        self.process = Some(process);
    }

    pub fn terminate(&mut self) {
        if let Some(mut process) = self.process.take() {
            log::debug!("Terminated ripper of stream `{}`.", self.stream_url());
            _ = process.kill(); // Don't care about result.
        }
    }

    pub fn process_mut(&mut self) -> &mut Option<Child> {
        &mut self.process
    }

    pub fn stream_url(&self) -> &str {
        &self.stream_url
    }
}
