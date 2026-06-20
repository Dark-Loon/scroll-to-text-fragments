use std::{
    io::{self, IsTerminal, Read},
    process::ExitCode,
};

use clap::Parser;

use stf_core::{FragmentError, TextFragment, build_url};
use thiserror::Error;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let stdin_text = if !io::stdin().is_terminal() {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).ok();
        Some(buf.trim().to_string())
    } else {
        None
    };

    let result = resolve_mode(&cli, stdin_text)
        .map_err(RunError::from)
        .and_then(run);

    match result {
        Ok(url) => {
            println!("{}", url);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run(mode: Mode) -> Result<String, RunError> {
    match mode {
        Mode::Interactive => Err(RunError::InteractiveNotYetImplemented),

        Mode::FromStdin {
            base,
            text,
            prefix,
            suffix,
        } => {
            let fragment = TextFragment::new(text, None, prefix, suffix);
            let url = build_url(&base, &fragment)?;
            Ok(url)
        }

        Mode::Direct {
            base,
            text,
            prefix,
            suffix,
            stdin_ignored,
        } => {
            if stdin_ignored {
                eprintln!(
                    "note: text argument and piped stdin both provided -- using the argument, stdin ignored"
                );
            }
            let fragment = TextFragment::new(text, None, prefix, suffix);
            let url = build_url(&base, &fragment)?;
            Ok(url)
        }
    }
}

/// Produce a URL that links directly to specific text in a web page.
///
/// When opened, the brower highlights the text and scrolls it into view.
///
/// stf supports three ways of providing input -- see EXAMPLES below.
#[derive(Parser, Debug)]
#[command(
    name = "stf",
    version,
    about = "Produce a URL that links directly to specific text in a web page.",
    after_help = "EXAMPLES:\n    \
                stf https://example.com \"short simple text\"\n   \
                termux-clipboard-get | stf https://example.com\n   \
                stf"
)]
struct Cli {
    /// URL of the page to link to
    base: Option<String>,

    /// Text to highlight. Omit to read from a pipe, or omit both BASE and TEXT for an interactive prompt
    text: Option<String>,

    /// Text immediately before the match, to disambiguate repeated occurrences
    #[arg(short, long, help_heading = "Disambiguation")]
    prefix: Option<String>,

    /// Text immediately after the match, to disambiguate repeated occurrences
    #[arg(short, long, help_heading = "Disambiguation")]
    suffix: Option<String>,

    /// URL of the page to link to
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, PartialEq)]
enum Mode {
    Interactive,
    FromStdin {
        base: String,
        text: String,
        prefix: Option<String>,
        suffix: Option<String>,
    },
    Direct {
        base: String,
        text: String,
        prefix: Option<String>,
        suffix: Option<String>,
        stdin_ignored: bool,
    },
}

#[derive(Debug, Error, PartialEq)]
enum ModeError {
    #[error(
        "no text to highlight\npass it directly:  stf {{url}} \"text to highlight\"\nor pipe it in:     your clipboard-command | stf {{url}}"
    )]
    MissingText,

    #[error(
        "no base URL was given.\n   \
        usage: your-clipboard-command | stf <URL>"
    )]
    MissingBase,
}

#[derive(Debug, Error, PartialEq)]
enum RunError {
    #[error(transparent)]
    Mode(#[from] ModeError),

    #[error(transparent)]
    Fragment(#[from] FragmentError),

    #[error("interactive mode isn't implemented yet -- try passing a URL and text directly")]
    InteractiveNotYetImplemented,
}

fn resolve_mode(cli: &Cli, stdin_text: Option<String>) -> Result<Mode, ModeError> {
    if cli.base.is_none() && (cli.prefix.is_some() || cli.suffix.is_some()) {
        return Err(ModeError::MissingBase);
    }

    match (&cli.base, &cli.text, stdin_text) {
        (None, None, None) => Ok(Mode::Interactive),
        (Some(b), None, Some(t)) => Ok(Mode::FromStdin {
            base: b.clone(),
            text: t,
            prefix: cli.prefix.clone(),
            suffix: cli.suffix.clone(),
        }),
        (Some(b), Some(t), None) => Ok(Mode::Direct {
            base: b.clone(),
            text: t.clone(),
            prefix: cli.prefix.clone(),
            suffix: cli.suffix.clone(),
            stdin_ignored: false,
        }),
        (Some(b), Some(t), Some(_)) => Ok(Mode::Direct {
            base: b.clone(),
            text: t.clone(),
            prefix: cli.prefix.clone(),
            suffix: cli.suffix.clone(),
            stdin_ignored: true,
        }),
        (Some(_), None, None) => Err(ModeError::MissingText),
        (None, None, Some(_)) => Err(ModeError::MissingBase),
        (None, Some(_), _) => unreachable!("missing base"),
    }
}

#[cfg(test)]
mod cli_tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}

#[cfg(test)]
mod run_tests {
    use super::*;

    #[test]
    fn direct_mode_builds_url() {
        let mode = Mode::Direct {
            base: String::from("https://example.com"),
            text: String::from("iceberg"),
            prefix: None,
            suffix: None,
            stdin_ignored: false,
        };

        assert_eq!(run(mode), Ok("https://example.com/#:~:text=iceberg".into()));
    }

    #[test]
    fn interactive_mode_is_not_yet_implemented() {
        assert_eq!(
            run(Mode::Interactive),
            Err(RunError::InteractiveNotYetImplemented)
        );
    }
}

#[cfg(test)]
mod resolve_mode_tests {
    use super::*;

    fn cli(
        base: Option<&str>,
        text: Option<&str>,
        prefix: Option<&str>,
        suffix: Option<&str>,
    ) -> Cli {
        Cli {
            base: base.map(String::from),
            text: text.map(String::from),
            prefix: prefix.map(String::from),
            suffix: suffix.map(String::from),
            verbose: false,
        }
    }

    #[test]
    fn text_arg_takes_priority_over_piped_stdin() {
        let got = resolve_mode(
            &cli(Some("https://example.com"), Some("hi"), None, None),
            Some("piped".into()),
        );

        assert_eq!(
            got,
            Ok(Mode::Direct {
                base: String::from("https://example.com"),
                text: String::from("hi"),
                prefix: None,
                suffix: None,
                stdin_ignored: true,
            })
        );
    }

    #[test]
    fn nothing_at_all_is_interactive() {
        let got = resolve_mode(&cli(None, None, None, None), None);

        assert_eq!(got, Ok(Mode::Interactive));
    }

    #[test]
    fn base_plus_piped_text_is_clipboard_mode() {
        let got = resolve_mode(
            &cli(Some("https://example.com"), None, None, None),
            Some("piped".into()),
        );

        assert_eq!(
            got,
            Ok(Mode::FromStdin {
                base: String::from("https://example.com"),
                text: String::from("piped"),
                prefix: None,
                suffix: None,
            })
        );
    }

    #[test]
    fn base_and_text_is_direct_mode() {
        let got = resolve_mode(
            &cli(Some("https://example.com"), Some("human"), None, None),
            None,
        );

        assert_eq!(
            got,
            Ok(Mode::Direct {
                base: String::from("https://example.com"),
                text: String::from("human"),
                prefix: None,
                suffix: None,
                stdin_ignored: false,
            })
        );
    }

    #[test]
    fn base_alone_with_no_pipe_is_an_error() {
        let got = resolve_mode(&cli(Some("https://example.com"), None, None, None), None);

        assert_eq!(got, Err(ModeError::MissingText));
    }

    #[test]
    fn prefix_without_base_is_missing_base_error() {
        let mut c = cli(None, None, None, None);
        c.prefix = Some("before".into());

        let got = resolve_mode(&c, None);

        assert_eq!(got, Err(ModeError::MissingBase));
    }

    #[test]
    fn prefix_and_suffix_flow_into_direct_mode() {
        let mut c = cli(Some("https://example.com"), Some("human"), None, None);
        c.prefix = Some("before".into());
        c.suffix = Some("after".into());

        let got = resolve_mode(&c, None);

        assert_eq!(
            got,
            Ok(Mode::Direct {
                base: String::from("https://example.com"),
                text: String::from("human"),
                prefix: Some(String::from("before")),
                suffix: Some(String::from("after")),
                stdin_ignored: false,
            })
        );
    }
}
