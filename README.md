[![Build status](https://img.shields.io/github/workflow/status/eikendev/turs/Main)](https://github.com/eikendev/turs/actions)
[![License](https://img.shields.io/crates/l/turs)](https://crates.io/crates/turs)
[![Version](https://img.shields.io/crates/v/turs)](https://crates.io/crates/turs)
[![Downloads](https://img.shields.io/crates/d/turs)](https://crates.io/crates/turs)

## About

Turs is a [Purs](https://github.com/xcambar/purs)-inspired prompt.
I love to keep things simple, and this prompt gives me everything I need.

## Usage

1. Install using `$ cargo install turs`
1. Add Cargo binaries to your `$PATH` environment variable
1. Add the following snippet to your ZSH configuration:

```zsh
autoload -Uz add-zsh-hook

function _prompt_turs_precmd() {
	turs precmd
}

function _prompt_turs_prompt() {
	PROMPT=`turs prompt -k "$KEYMAP" -r "$?"`
}

function _prompt_turs_rprompt() {
	RPROMPT=`turs rprompt`
}

function zle-line-init zle-keymap-select {
	_prompt_turs_prompt
	_prompt_turs_rprompt
	zle reset-prompt
}

zle -N zle-line-init
zle -N zle-keymap-select
_prompt_turs_prompt
_prompt_turs_rprompt

add-zsh-hook precmd _prompt_turs_precmd
```

## Installation

### From crates.io

```bash
cargo install turs
```
