use clap::Parser;

fn main() {
    println!("Hello, world!");
}

/// Produce a URL that links directly to specific text in a web page.
#[derive(Parser, Debug)]
#[command(name = "stf")]
#[command(version = "1.0")]
#[command(about = "Produce a URL that links directly to specific text in a web page.")]
struct Cli {
    base: Option<String>,
    text: Option<String>,
    #[arg(short, long)]
    prefix: Option<String>,
    #[arg(short, long)]
    suffix: Option<String>,
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, PartialEq)]
enum Mode {
    Interactive,
    FromStdin {
        base: String,
        text: String,
    },
    Direct {
        base: String,
        text: String,
        stdin_ignored: bool,
    },
}

#[derive(Debug, PartialEq)]
enum ModeError {
    MissingText,
    MissingBase,
}

fn resolve_mode(cli: &Cli, stdin_text: Option<String>) -> Result<Mode, ModeError> {
    match (&cli.base, &cli.text, stdin_text) {
        (None, None, None) => Ok(Mode::Interactive),
        (Some(b), None, Some(t)) => Ok(Mode::FromStdin {
            base: b.clone(),
            text: t,
        }),
        (Some(b), Some(t), None) => Ok(Mode::Direct {
            base: b.clone(),
            text: t.clone(),
            stdin_ignored: false,
        }),
        (Some(b), Some(t), Some(_)) => Ok(Mode::Direct {
            base: b.clone(),
            text: t.clone(),
            stdin_ignored: true,
        }),
        (Some(_), None, None) => Err(ModeError::MissingText),
        (None, None, Some(_)) => Err(ModeError::MissingBase),
        (None, Some(_), _) => unreachable!("missing base"),
    }
}

#[cfg(test)]
mod resolve_mode_tests {
    use super::*;

    fn cli(base: Option<&str>, text: Option<&str>) -> Cli {
        Cli {
            base: base.map(String::from),
            text: text.map(String::from),
            prefix: None,
            suffix: None,
            verbose: false,
        }
    }

    #[test]
    fn text_arg_takes_priority_over_piped_stdin() {
        let got = resolve_mode(
            &cli(Some("https://example.com"), Some("hi")),
            Some("piped".into()),
        );

        assert_eq!(
            got,
            Ok(Mode::Direct {
                base: "https://example.com".into(),
                text: "hi".into(),
                stdin_ignored: true,
            })
        );
    }

    #[test]
    fn nothing_at_all_is_interactive() {
        let got = resolve_mode(&cli(None, None), None);

        assert_eq!(got, Ok(Mode::Interactive));
    }

    #[test]
    fn base_plus_piped_text_is_clipboard_mode() {
        let got = resolve_mode(
            &cli(Some("https://example.com"), None),
            Some("piped".into()),
        );

        assert_eq!(
            got,
            Ok(Mode::FromStdin {
                base: "https://example.com".into(),
                text: "piped".into()
            })
        );
    }

    #[test]
    fn base_and_text_is_direct_mode() {
        let got = resolve_mode(&cli(Some("https://example.com"), Some("human")), None);

        assert_eq!(
            got,
            Ok(Mode::Direct {
                base: "https://example.com".into(),
                text: "human".into(),
                stdin_ignored: false,
            })
        );
    }

    #[test]
    fn base_alone_with_no_pipe_is_an_error() {
        let got = resolve_mode(&cli(Some("https://example.com"), None), None);

        assert_eq!(got, Err(ModeError::MissingText));
    }

    #[test]
    fn prefix_and_suffix_flow_into_direct_mode() {
        let mut c = cli(Some("https://example.com"), Some("human"));
        c.prefix = Some("before".into());
        c.suffix = Some("after".into());

        let got = resolve_mode(&c, None);

        assert_eq!(
            got,
            Ok(Mode::Direct {
                base: "https://example.com".into(),
                text: "human".into(),
                prefix: Some("human".into()),
                suffix: Some("human".into()),
                stdin_ignored: false,
            })
        );
    }
}
