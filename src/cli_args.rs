pub trait CliArgs {
    fn split_as_args(&self) -> Vec<String>;
}

impl CliArgs for &str {
    fn split_as_args(&self) -> Vec<String> {
        let mut args: Vec<String> = vec![];
        let mut escape_next_symbol = false;
        let mut inside_double_quote = false;

        let mut current_arg = String::new();
        for symbol in self.chars() {
            let escape_symbol = escape_next_symbol;
            escape_next_symbol = false;

            match symbol {
                ' ' if !inside_double_quote => {
                    if !current_arg.is_empty() {
                        args.push(current_arg);
                        current_arg = String::new();
                    }
                }
                '\\' if !escape_symbol => {
                    current_arg.push(symbol);
                    escape_next_symbol = true; // TODO: is this necessary?
                }
                '"' if !escape_symbol => {
                    // current_arg.push(symbol);
                    inside_double_quote = !inside_double_quote;
                }
                _ => {
                    current_arg.push(symbol);
                    escape_next_symbol = false;
                }
            };
        }

        // If the remaining argument is not empty, push it. Or if there are no arguments yet, push even an empty string.
        if !current_arg.is_empty() || args.is_empty() {
            args.push(current_arg);
        }

        args
    }
}

impl CliArgs for String {
    fn split_as_args(&self) -> Vec<String> {
        self.as_str().split_as_args()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_args_splitter() -> Result<()> {
        assert_eq!("".split_as_args(), [""]);
        assert_eq!("\"".split_as_args(), [""]);
        assert_eq!("\\".split_as_args(), ["\\"]);
        assert_eq!("\\ \\".split_as_args(), ["\\", "\\"]);
        assert_eq!("hello world".split_as_args(), ["hello", "world"]);
        assert_eq!("he said 'hi'".split_as_args(), ["he", "said", "'hi'"]);
        assert_eq!("a   b c".split_as_args(), ["a", "b", "c"]);
        assert_eq!(
            "this is \"probably another\" test".split_as_args(),
            ["this", "is", "probably another", "test"]
        );

        assert_eq!(
            r#""streamlink.exe" --loglevel info --player "C:\Users\Weird User Name\AppData\Local\Programs\mpv.net\mpvnet.exe" --twitch-disable-ads --default-stream "1080p, 720p, best" --url https://www.twitch.tv/theprimeagen"#.split_as_args(),
            ["streamlink.exe", "--loglevel", "info", "--player", r#"C:\Users\Weird User Name\AppData\Local\Programs\mpv.net\mpvnet.exe"#, "--twitch-disable-ads", "--default-stream", "1080p, 720p, best", "--url", "https://www.twitch.tv/theprimeagen"]
        );

        assert_eq!(
            r#".\ffmpeg.exe -ss 00:09:15 -i "Xilem: Let's Build High Performance Rust UI - Raph Levien [OvfNipIcRiQ].webm" -t 00:00:59 -map 0:1 track1.wav -map 0:2 "second track.wav""#.split_as_args(),
            [r#".\ffmpeg.exe"#, "-ss", "00:09:15", "-i", "Xilem: Let's Build High Performance Rust UI - Raph Levien [OvfNipIcRiQ].webm", "-t", "00:00:59", "-map", "0:1", "track1.wav", "-map", "0:2", "second track.wav"]
        );

        Ok(())
    }
}
