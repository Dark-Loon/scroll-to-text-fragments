use std::{io::Write, process::Stdio, time::Duration};

use assert_cmd::Command;
use predicates::prelude::predicate;
use wait_timeout::ChildExt;

#[test]
fn does_not_hang_when_stdin_is_piped_but_never_closed() {
    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_stf"))
        .args(["https://example.com", "iceberg"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn stf");

    let mut stdin = child.stdin.take().expect("stdin should be piped");
    stdin
        .write_all(b"never going to EOF\n")
        .expect("failed to write to child stdin");

    match child
        .wait_timeout(Duration::from_secs(2))
        .expect("error polling child")
    {
        Some(status) => assert!(status.success(), "stf exited non-zero: {status:?}"),
        None => {
            child.kill().ok();
            let _ = child.wait();
            panic!(
                "stf hung reading stdin even though base and text were \
                 already supplied as arguments"
            );
        }
    }

    drop(stdin);
}

#[test]
fn warns_on_stderr_when_text_arg_and_piped_stdin_both_given() {
    Command::cargo_bin("stf")
        .unwrap()
        .args(["https://example.com", "iceberg"])
        .write_stdin("ignored clipboard text")
        .assert()
        .success()
        .stdout("https://example.com/#:~:text=iceberg\n")
        .stderr(predicate::str::contains("stdin ignored"));
}

#[test]
fn verbose_prints_mode_to_stderr() {
    Command::cargo_bin("stf")
        .unwrap()
        .args(["--verbose", "https://example.com", "iceberg"])
        .assert()
        .success()
        .stderr(predicate::str::contains("mode:"));
}

#[test]
fn level_two_verbose_prints_fragment_to_stderr() {
    Command::cargo_bin("stf")
        .unwrap()
        .args(["-vv", "https://example.com", "iceberg"])
        .assert()
        .success()
        .stderr(predicate::str::contains("fragment:"));
}

#[test]
fn direct_mode_end_to_end() {
    Command::cargo_bin("stf")
        .unwrap()
        .args(["https://example.com", "iceberg"])
        .assert()
        .success()
        .stdout("https://example.com/#:~:text=iceberg\n");
}

#[test]
fn clipboard_mode_reads_piped_stdin() {
    Command::cargo_bin("stf")
        .unwrap()
        .arg("https://example.com")
        .write_stdin("piped text")
        .assert()
        .success()
        .stdout("https://example.com/#:~:text=piped%20text\n");
}

#[test]
fn missing_text_fails_with_helpful_stderr() {
    Command::cargo_bin("stf")
        .unwrap()
        .arg("https://example.com")
        .assert()
        .failure()
        .stderr(predicate::str::contains("no text to highlight"));
}

#[test]
fn completions_bash_runs_and_mentions_binary_name() {
    Command::cargo_bin("stf")
        .unwrap()
        .args(["--completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("stf"));
}
