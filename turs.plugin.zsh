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
