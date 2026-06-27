use std::{
    io::{self, IsTerminal, Read},
    process::ExitCode,
};

use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{Shell, generate};
use clap_complete_nushell::Nushell;

use inquire::{Confirm, InquireError, Text, required, validator::Validation};
use owo_colors::OwoColorize;
use stf_core::{FragmentError, TextFragment, build_url, extract_range};
use thiserror::Error;
use url::Url;

fn main() -> ExitCode {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        let mut cmd = Cli::command();
        let bin_name = cmd.get_name().to_string();
        match shell {
            CompletionShell::Bash => generate(Shell::Bash, &mut cmd, bin_name, &mut io::stdout()),
            CompletionShell::Elvish => {
                generate(Shell::Elvish, &mut cmd, bin_name, &mut io::stdout())
            }
            CompletionShell::Nushell => generate(Nushell, &mut cmd, bin_name, &mut io::stdout()),
            CompletionShell::PowerShell => {
                generate(Shell::PowerShell, &mut cmd, bin_name, &mut io::stdout())
            }
            CompletionShell::Zsh => generate(Shell::Zsh, &mut cmd, bin_name, &mut io::stdout()),
            CompletionShell::Fish => generate(Shell::Fish, &mut cmd, bin_name, &mut io::stdout()),
        }

        return ExitCode::SUCCESS;
    }

    let stdin_is_piped = !io::stdin().is_terminal();

    let stdin_text = if cli.text.is_none() && stdin_is_piped {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).ok();
        let trimmed = buf.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    } else {
        None
    };

    let result = resolve_mode(&cli, stdin_text, stdin_is_piped)
        .map_err(RunError::from)
        .and_then(|mode| run(mode, cli.verbose));

    match result {
        Ok(url) => {
            println!("{}", url);
            ExitCode::SUCCESS
        }
        Err(e) => {
            anstream::eprintln!("{} {}", "error:".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}

fn run(mode: Mode, verbose: u8) -> Result<String, RunError> {
    if verbose >= 1 {
        anstream::eprintln!("{} {:?}", "mode:".cyan().dimmed(), mode);
    }

    match mode {
        Mode::Interactive => {
            let (base, text, prefix, suffix) = prompt_for_fragment(verbose)?;
            build_fragment_url(&base, text, prefix, suffix, verbose)
        }

        Mode::FromStdin {
            base,
            text,
            prefix,
            suffix,
        } => build_fragment_url(&base, text, prefix, suffix, verbose),

        Mode::Direct {
            base,
            text,
            prefix,
            suffix,
            stdin_ignored,
        } => {
            if stdin_ignored {
                anstream::eprintln!(
                    "{} text argument and piped stdin both provided -- using the argument, stdin ignored",
                    "note:".yellow().bold()
                );
            }
            build_fragment_url(&base, text, prefix, suffix, verbose)
        }
    }
}

fn build_fragment_url(
    base: &str,
    text: String,
    prefix: Option<String>,
    suffix: Option<String>,
    verbose: u8,
) -> Result<String, RunError> {
    let (start, end) = extract_range(&text);
    let fragment = TextFragment::new(start, end, prefix, suffix);
    if verbose >= 2 {
        anstream::eprintln!("{} {:?}", "fragment:".cyan().dimmed(), fragment);
    }
    let url = build_url(base, &fragment)?;
    Ok(url)
}

fn prompt_for_fragment(
    verbose: u8,
) -> Result<(String, String, Option<String>, Option<String>), InquireError> {
    let base = Text::new("What is the base URL?")
        .with_validator(required!("This field is required"))
        .with_validator(|input: &str| {
            if Url::parse(input).is_ok() {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "not a valid URL -- include the scheme, e.g. https://example.com".into(),
                ))
            }
        })
        .with_help_message("e.g. https://example.com")
        .prompt()?;

    let text = Text::new("What text do you want to link to?")
        .with_validator(required!("This field is required"))
        .with_help_message("Paste the exact passage you want highlighted")
        .prompt()?;

    if let Ok(preview) = build_fragment_url(&base, text.clone(), None, None, verbose) {
        anstream::eprint!("\n{}:\n{}", "preview".cyan().bold(), preview);
        anstream::eprintln!("\n   (you can stop here and use this, or continue to disambiguate)\n");
    }

    let wants_disambiguation = Confirm::new("Disambiguate repeated matches with a prefix/suffix?")
        .with_default(false)
        .with_help_message("Most pages won't need this -- only useful if your text appears more than once on the page")
        .prompt()?;

    let (prefix, suffix) = if wants_disambiguation {
        let prefix = Text::new("Prefix (optional)")
            .with_help_message(
                "Text immediately before the match, to disambiguate repeated occurrences -- press Enter to skip",
            )
            .prompt_skippable()?
            .filter(|s| !s.trim().is_empty());

        let suffix = Text::new("Suffix (optional)")
            .with_help_message(
                "Text immediately after the match, to disambiguate repeated occurrences -- press Enter to skip",
            )
            .prompt_skippable()?
            .filter(|s| !s.trim().is_empty());

        (prefix, suffix)
    } else {
        (None, None)
    };

    Ok((base, text, prefix, suffix))
}

/// Produce a URL that links directly to specific text in a web page.
///
/// When opened, the browser highlights the text and scrolls it into view.
///
/// stf supports three ways of providing input -- see EXAMPLES below.
#[derive(Parser, Debug)]
#[command(
    name = "stf",
    version,
    about = "Produce a URL that links directly to specific text in a web page.",
    after_help = "EXAMPLES:\n   \
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

    /// Print details about how the URL was constructed
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Generate a shell completion script and print it to stdout
    #[arg(
        long,
        value_enum,
        value_name = "SHELL",
        exclusive = true,
        help_heading = "Shell Completions"
    )]
    completions: Option<CompletionShell>,
}

/// The shells one can generate completions for. A thin wrapper around
/// clap_complete::Shell plus Nushell, since the two crates' generator types
/// can't share a single enum directly
#[derive(Copy, Clone, Debug, ValueEnum)]
enum CompletionShell {
    Bash,
    Elvish,
    Fish,
    Nushell,
    PowerShell,
    Zsh,
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

#[derive(Debug, Error)]
enum RunError {
    #[error(transparent)]
    Mode(#[from] ModeError),

    #[error(transparent)]
    Fragment(#[from] FragmentError),

    #[error(transparent)]
    Prompt(#[from] InquireError),
}

fn resolve_mode(
    cli: &Cli,
    stdin_text: Option<String>,
    stdin_is_piped: bool,
) -> Result<Mode, ModeError> {
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
            // If someone runs echo -n "" | stf https://example.com "iceberg", it'll now warn about stdin being ignored even though there was nothing in it
            base: b.clone(),
            text: t.clone(),
            prefix: cli.prefix.clone(),
            suffix: cli.suffix.clone(),
            stdin_ignored: stdin_is_piped,
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

        // RunError can't derive PartialEq (InquireError wraps std::io::Error,
        // which doesn't implement it), so we unwrap and compare the String
        // payload directly rather than comparing the whole Result.
        assert_eq!(
            run(mode, 0).unwrap(),
            "https://example.com/#:~:text=iceberg"
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
            verbose: 0,
            completions: None,
        }
    }

    #[test]
    fn text_arg_present_means_stdin_is_flagged_ignored_without_being_read() {
        //stdin_text is None here on purpose
        // main() never reads it when cli.text is already Some
        let got = resolve_mode(
            &cli(Some("https://example.com"), Some("hi"), None, None),
            None,
            true,
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
        let got = resolve_mode(&cli(None, None, None, None), None, false);

        assert_eq!(got, Ok(Mode::Interactive));
    }

    #[test]
    fn base_plus_piped_text_is_clipboard_mode() {
        let got = resolve_mode(
            &cli(Some("https://example.com"), None, None, None),
            Some("piped".into()),
            true,
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
            false,
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
        let got = resolve_mode(
            &cli(Some("https://example.com"), None, None, None),
            None,
            false,
        );

        assert_eq!(got, Err(ModeError::MissingText));
    }

    #[test]
    fn prefix_without_base_is_missing_base_error() {
        let mut c = cli(None, None, None, None);
        c.prefix = Some("before".into());

        let got = resolve_mode(&c, None, false);

        assert_eq!(got, Err(ModeError::MissingBase));
    }

    #[test]
    fn prefix_and_suffix_flow_into_direct_mode() {
        let mut c = cli(Some("https://example.com"), Some("human"), None, None);
        c.prefix = Some("before".into());
        c.suffix = Some("after".into());

        let got = resolve_mode(&c, None, false);

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
