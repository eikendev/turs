# Turs

A [Purs](https://github.com/xcambar/purs)-inspired prompt.

## Usage

1. Build `turs` using `$ cargo build --release`
1. Add the binary to your `$PATH`
1. Add the following snippet to your ZSH configuration:

```
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
