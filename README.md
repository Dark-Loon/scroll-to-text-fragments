# Scroll to Text Fragments (stf)

Produce a URL that links directly to specific text in a web page. When opened, the browser highlights the text and scrolls it into view.

[![Crates.io](https://img.shields.io/crates/v/stf-cli.svg)](https://crates.io/crates/stf-cli)
[![License](https://img.shields.io/crates/l/stf-cli.svg)](#license)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-stf-cli?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/stf-cli)


## Demo

<div align="center">
  <img src="https://gitlab.com/Dark-Loon/scroll-to-text-fragments/-/raw/27b9f38fdc9ec2a566f4c3427767cee5c72901db/assets/demo-desktop.gif" width="600" alt="Desktop demo">
  <br><br>
  <img src="https://gitlab.com/Dark-Loon/scroll-to-text-fragments/-/raw/c152c3772901db37d7f38860eef230be8700da99/assets/demo-mobile.gif" width="280" alt="Mobile demo">
</div>


## Install

**On Termux (Android)**:

```bash
pkg install rust
cargo install stf-cli
```

**Desktop**:

```bash
cargo install stf-cli
```

Requires [Rust](https://www.rust-lang.org/tools/install).


## Mobile (Android + Termux)

Requires Termux, the [Termux:API](https://wiki.termux.com/wiki/Termux:API) app as well as the termux-api package.

Install Termux:API, then:
```bash
pkg install termux-api

# Add Cargo's bin directory to your PATH if not already present:
echo 'export PATH="$PATH:$HOME/.cargo/bin"' >> ~/.bashrc && source ~/.bashrc

mkdir -p ~/bin

cat > ~/bin/termux-url-opener << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash

export PATH="$PATH:$HOME/.cargo/bin"

URL=$(echo "${1:-$(termux-clipboard-get)}" | grep -oP 'https?://\S+' | head -1)

if [ -z "$URL" ]; then
  termux-toast "No URL found"
  exit 1
fi

SELECTED=$(termux-clipboard-get)

if [ -z "$SELECTED" ]; then
  termux-toast "Copy some text first"
  exit 1
fi

echo "$SELECTED" | stf "$URL" | termux-clipboard-set
termux-toast "Fragment link copied!"
EOF

chmod +x ~/bin/termux-url-opener
```

Then:

1. Long-press text in browser → **Copy**
2. Tap **Share** → **Termux**
3. Done


## Interactive

```bash
stf
```

Interactive mode is the simplest approach and handles any URL, including those with special characters, as well as any text with quotes or spanning multiple paragraphs.


## Direct

```bash
stf https://example.com "short simple text"
```

For text with quotes, spanning multiple paragraphs, or URLs with special characters, use interactive mode.


## Clipboard / pipe

```bash
wl-paste | stf https://example.com               # Wayland
xclip -selection clipboard -o | stf https://example.com   # X11
termux-clipboard-get | stf https://example.com   # Termux
```

For URLs with special characters, quote them or use interactive mode.


## Disambiguating repeated text

If the same text appears more than once on a page:

```bash
stf https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a "The Referer" --prefix "downgrade:" --suffix "to origins"
```


## Flags

| Flag | Description |
|---|---|
| `-p`, `--prefix` | Text immediately before the match, to disambiguate repeated occurrences |
| `-s`, `--suffix` | Text immediately after the match, to disambiguate repeated occurrences |
| `-v`, `--verbose` | Print details about how the URL was constructed (`-vv` for more) |
| `--completions <SHELL>` | Generate a shell completion script and print it to stdout |
| `-h`, `--help` | Print usage |

<details>
<summary>Shell completions setup</summary>

<!-- <br> -->

```bash
# bash
stf --completions bash > ~/.local/share/bash-completion/completions/stf

# zsh
mkdir -p ~/.zfunc && stf --completions zsh > ~/.zfunc/_stf
# Add to ~/.zshrc before compinit: fpath+=~/.zfunc

# fish
stf --completions fish > ~/.config/fish/completions/stf.fish

# powershell
stf --completions power-shell

# nushell
mkdir ~/.cache/stf
stf --completions nushell o> ~/.cache/stf/completions.nu
# Add to config.nu: source ~/.cache/stf/completions.nu
```

</details>


## Notes

- Quote URLs that contain parentheses or other shell-special characters (e.g. "https://en.wikipedia.org/wiki/King_Roger_(opera)")
- Works with any writing system, including right-to-left text (Arabic, Hebrew, etc.)
- Does not work with PDF URLs, as browsers don't apply text fragments to PDF viewers
- If the browser doesn't support text fragments, or nothing matches, the link just loads the page normally
- Some sites opt out via `Document-Policy: force-load-at-top` (GitHub is one)

## Roadmap

- Fetch the page and verify the fragment actually matches before returning the URL

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

## Acknowledgements

Text Fragments was specified and implemented by [Nick Burris](https://github.com/nickburris) and [David Bokan](https://github.com/bokand), with contributions from [Grant Wang](https://github.com/grantjwang).

Built with assistance from [Claude](https://claude.ai).
