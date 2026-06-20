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
    // use clap::CommandFactory;

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

    // #[test]
    // fn verify_cli() {
    //     Cli::command().debug_assert();
    // }

    // #[test]
    // fn verify_less_than_three_arguments() {}
}
