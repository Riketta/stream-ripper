# Stream Ripper

This is a tool I created to record streams that I am interested in, but I can't watch live and their VODs are only available to subscribers.

## Usage

On first run, a default configuration file `config.toml` will be created (or it can be specified manually using the `--config` CLI option). Edit it to suit your needs.

**Example:**

```toml
log_level = "DEBUG"
logs_folder = "logs"
stream_urls = ["https://www.twitch.tv/riketta", "https://www.twitch.tv/agurin", "https://www.twitch.tv/theprimeagen"]
streamlink_cli = 'streamlink --force --logfile "logs\{source}_{timestamp}.log" --output "streams\{author}_{time:%Y%m%d-%H%M%S}.mp4" --progress no --twitch-disable-ads --default-stream "1080p, 720p, best" --url {url}'
```
