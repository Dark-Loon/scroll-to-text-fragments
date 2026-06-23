# Scroll to Text Fragments (stf)

Produce a URL that links directly to specific text in a web page. When opened, the browser highlights the text and scrolls it into view.

<!-- [![Crates.io](https://img.shields.io/crates/v/stf-cli.svg)](https://crates.io/crates/stf-cli) -->
<!-- [![License](https://img.shields.io/crates/l/stf-cli.svg)](#license) -->
<!-- [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-stf-cli-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/stf-cli) -->


## Demo

<!-- TODO: Termux share GIF once set up -->
![Animated GIF making a demonstration of stf's three modes: direct, interactive, and clipboard](./assets/demo.gif)

> You can replay this in your terminal: `asciinema play ./assets/demo.cast`


## Install

```bash
cargo install stf-cli
```

On Termux (Android):

```bash
pkg install rust
cargo install stf-cli
```

The first build will take a few minutes to compile on-device.


## Mobile (Android + Termux)

**Basic clipboard mode**:

1. Long-press text in browser → **Copy**
2. In Termux: `termux-clipboard-get | stf https://the-page-url.com`
3. Done

**Recommended: one-time setup**:

Requires the [Termux:API](https://wiki.termux.com/wiki/Termux:API) app.

```bash
mkdir -p ~/bin
cat > ~/bin/termux-url-opener << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash
termux-clipboard-get | stf "$1" | termux-clipboard-set
termux-toast "Link copied"
EOF
chmod +x ~/bin/termux-url-opener
```

Then:

1. Long-press text in browser → **Copy**
2. Tap **Share** → **Termux**
3. Done


## Direct

```bash
stf https://example.com "short simple text"
```

For text with quotes or spanning multiple paragraphs, use interactive mode.


## Interactive

```bash
stf
```

Guided prompt with a live preview. Press Enter through each step.

## Clipboard / pipe

```bash
wl-paste | stf https://example.com               # Wayland
xclip -selection clipboard -o | stf https://example.com   # X11
termux-clipboard-get | stf https://example.com   # Termux
```


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
| `-v`, `--verbose` | Print details about how the URL was constructed |
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

- Works with any writing system, including right-to-left text (Arabic, Hebrew, etc.)
- If the browser doesn't support text fragments, or nothing matches, the link just loads the page normally.
- Some sites opt out via `Document-Policy: force-load-at-top` (GitHub is one).

## Roadmap

- Better handling of long passages with a start/end range
- Fetch the page and verify the fragment actually matches before returning the URL

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

## Acknowledgements

Text Fragments was specified and implemented by [Nick Burris](https://github.com/nickburris) and [David Bokan](https://github.com/bokand), with contributions from [Grant Wang](https://github.com/grantjwang).

Built with assistance from [Claude](https://claude.ai).
