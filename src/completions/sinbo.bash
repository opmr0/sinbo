_sinbo() {
    local cur prev
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    local subcommands="get g add a list l remove r edit e search s encrypt decrypt export import copy c rename completions"

    if [[ $COMP_CWORD -eq 1 ]]; then
        COMPREPLY=($(compgen -W "$subcommands" -- "$cur"))
        return
    fi

    if [[ $COMP_CWORD -eq 2 ]]; then
        case "$prev" in
            get|g|remove|r|edit|e|encrypt|decrypt|export|copy|c|rename)
                local snippets=$(sinbo list-names 2>/dev/null)
                COMPREPLY=($(compgen -W "$snippets" -- "$cur"))
                return ;;
        esac
    fi

    COMPREPLY=()
}

complete -F _sinbo sinbo