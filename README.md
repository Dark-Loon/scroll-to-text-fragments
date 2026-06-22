<!-- [![Crates.io](https://img.shields.io/crates/v/stf_cli.svg)](https://crates.io/crates/stf_cli) -->
<!-- [![License](https://img.shields.io/crates/l/stf_cli.svg)](#license) -->

# Scroll to Text Fragments (stf)

Produce a URL that links directly to specific text in a web page. When opened, the browser highlights the text and scrolls it into view.

## Demo

<!-- TODO: embed demo GIF here showing all three modes -->
<!-- [![Crates.io](https://img.shields.io/crates/v/stf_cli.svg)](https://crates.io/crates/stf_cli) -->

## Features

- Three ways to use it: pass text directly, pipe it in, or answer an interactive prompt
- Works with any script, including right-to-left text
- `--prefix`/`--suffix` to disambiguate text that appears more than once on a page
- Shell completions for bash, zsh, fish, PowerShell, and Nushell
- Linux, macOS, Windows, and Android (via Termux)

## Install

```bash
cargo install stf-cli
```

> Requires the Rust toolchain. On Termux, install it first with `pkg install rust` — the first build will take a few minutes to compile on-device.
```bash
pkg install rust
cargo install stf-cli
```

To install the latest unreleased version straight from source instead:

```bash
cargo install --git https://github.com/you/text-fragment-url --locked -p stf-cli
```

## Usage

### Direct mode

```bash
stf https://example.com "short simple text"
```

### Clipboard / pipe mode

Pipe text in from anywhere — a clipboard tool, a file, another command.

```bash
termux-clipboard-get | stf https://example.com   # Termux
wl-paste | stf https://example.com               # Wayland
xclip -selection clipboard -o | stf https://example.com   # X11
```

### Interactive mode

Run with no arguments for a guided prompt, with a live preview as you go.

```bash
stf
```

<details>
<summary>Quick links on Android, with one setup step</summary>

<br>

Out of the box, clipboard mode still needs you to type the URL yourself. To skip that, set up Termux's share-target script so the page URL is handed to `stf` automatically:

```bash
mkdir -p ~/bin
cat > ~/bin/termux-url-opener << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash
termux-clipboard-get | stf "$1" | termux-clipboard-set
termux-toast "Link copied"
EOF
chmod +x ~/bin/termux-url-opener
```

Then, day to day: select text on a page → **Copy** → tap **Share** → choose **Termux**. The link is generated and copied to your clipboard automatically, ready to paste. Requires the [Termux:API](https://wiki.termux.com/wiki/Termux:API) app.

</details>

## Disambiguating repeated text

If your text appears more than once on the page, anchor the match with `--prefix`/`-p` and `--suffix`/`-s`:

```bash
stf https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a "The Referer" --prefix "downgrade:" --suffix "to origins"
```

## Non-English and right-to-left scripts

```bash
stf https://example.com "مِصر" --prefix "البحرين"
# -> https://example.com/#:~:text=%D8%A7%D9%84%D8%A8%D8%AD%D8%B1%D9%8A%D9%86-,%D9%85%D9%90%D8%B5%D8%B1
```

## Flags

| Flag | Description |
|---|---|
| `-p`, `--prefix` | Text immediately before the match, to disambiguate repeated occurrences |
| `-s`, `--suffix` | Text immediately after the match, to disambiguate repeated occurrences |
| `-v`, `--verbose` | Print details about how the URL was constructed |
| `--completions <SHELL>` | Generate a shell completion script and print it to stdout |
| `-h`, `--help` | Print usage |

<details>
<summary>Shell completions setup</summary>

<br>

```bash
# bash
stf --completions bash > ~/.local/share/bash-completion/completions/stf

# zsh (needs a directory on $fpath; ~/.zfunc is a common choice)
mkdir -p ~/.zfunc && stf --completions zsh > ~/.zfunc/_stf
# then add to ~/.zshrc, before compinit: fpath+=~/.zfunc

# fish (auto-loaded, no shell config needed)
stf --completions fish > ~/.config/fish/completions/stf.fish

# powershell
stf --completions power-shell

# nushell -- note: o>, not >, since nushell's redirection syntax differs from POSIX shells
mkdir ~/.cache/stf
stf --completions nushell o> ~/.cache/stf/completions.nu
# then in config.nu: source ~/.cache/stf/completions.nu
```

</details>


## Notes

- If a browser doesn't support text fragments, or nothing matches, the fragment is silently ignored and the page just loads at the top.
- Some sites opt out via `Document-Policy: force-load-at-top` (GitHub is one).


## Future development plan

Multi-sentence range matching (`textStart,textEnd`)


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


## Acknowledgements

Text Fragments was implemented and specified by [Nick Burris](https://github.com/nickburris) and [David Bokan](https://github.com/bokand), with contributions from [Grant Wang](https://github.com/grantjwang).

Built with assistance from [Claude](https://claude.ai).













