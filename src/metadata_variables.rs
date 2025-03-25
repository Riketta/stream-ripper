use std::{collections::HashMap, path::Path};

use chrono::{Local, Timelike};

pub struct MetadataVariables;

impl MetadataVariables {
    fn templates<'a>(stream_url: &'a str, timestamp: &'a str) -> HashMap<&'a str, &'a str> {
        let source = Path::file_name(Path::new(stream_url)).unwrap_or_default();

        let templates = HashMap::from([
            ("{url}", stream_url),
            ("{source}", source.to_str().unwrap()),
            ("{timestamp}", timestamp),
        ]);

        templates
    }

    fn augment_cli_with_templates(cli: &str, templates: &HashMap<&str, &str>) -> String {
        let mut result = cli.to_owned();
        for pair in templates {
            result = result.replace(pair.0, pair.1);
        }

        result
    }

    /// Augment external command line string with metadata variables.
    pub fn augment_cli(cli: &str, stream_url: &str) -> String {
        let timestamp = Local::now().with_minute(0).unwrap().format("%Y%m%d-%H%M");
        let timestamp = format!("{timestamp}");
        let templates = MetadataVariables::templates(stream_url, &timestamp);

        Self::augment_cli_with_templates(cli, &templates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_templates() -> Result<()> {
        let stream_url = "https://www.twitch.tv/riketta";
        let timestamp_original = "123456789";
        let templates = MetadataVariables::templates(stream_url, timestamp_original);

        let source = templates["{url}"];
        assert_eq!(source, stream_url);

        let source = templates["{source}"];
        assert_eq!(source, "riketta");

        let timestamp = templates["{timestamp}"];
        assert_eq!(timestamp, timestamp_original);

        Ok(())
    }

    #[test]
    fn test_augments() -> Result<()> {
        let streamlink_cli = r##"streamlink --logfile "logs\{source}_{timestamp}.log" --output "streams\{author}_{time:%Y%m%d-%H%M%S}.mp4" --default-stream "1080p, 720p, best" --url {url}"##.to_owned();
        let stream_url = "https://www.twitch.tv/riketta";
        let timestamp = "123456789";
        let templates = MetadataVariables::templates(stream_url, &timestamp);
        let streamlink_cli =
            MetadataVariables::augment_cli_with_templates(&streamlink_cli, &templates);

        assert_eq!(
            streamlink_cli,
            r##"streamlink --logfile "logs\riketta_123456789.log" --output "streams\{author}_{time:%Y%m%d-%H%M%S}.mp4" --default-stream "1080p, 720p, best" --url https://www.twitch.tv/riketta"##
        );

        Ok(())
    }
}
