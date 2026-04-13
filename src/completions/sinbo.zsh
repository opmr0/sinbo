#compdef sinbo

_sinbo() {
    local state

    _arguments \
        '1: :->subcommand' \
        '*: :->args'

    case $state in
        subcommand)
            local subcommands=(
                'get:Print or copy a snippet'
                'g:Print or copy a snippet'
                'add:Add a new snippet'
                'a:Add a new snippet'
                'list:List all snippets'
                'l:List all snippets'
                'remove:Remove a snippet'
                'r:Remove a snippet'
                'edit:Edit an existing snippet'
                'e:Edit an existing snippet'
                'search:Search snippets'
                's:Search snippets'
                'encrypt:Encrypt a snippet'
                'decrypt:Decrypt a snippet'
                'export:Export a snippet'
                'import:Import a snippet'
                'copy:Copy a snippet to clipboard'
                'c:Copy a snippet to clipboard'
                'rename:Rename a snippet'
                'completions:Generate shell completions'
            )
            _describe 'subcommand' subcommands ;;
        args)
            case $words[2] in
                get|g|remove|r|edit|e|encrypt|decrypt|export|copy|c|rename)
                    local snippets=(${(f)"$(sinbo list-names 2>/dev/null)"})
                    _describe 'snippet' snippets ;;
            esac ;;
    esac
}

_sinbo