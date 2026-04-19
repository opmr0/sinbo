function __sinbo_snippets
    sinbo list-names 2>/dev/null
end

set -l subcommands get g add a list l remove r edit e search s encrypt decrypt export import copy c rename completions

complete -c sinbo -f -n "not __fish_seen_subcommand_from $subcommands" -a "$subcommands"

for cmd in get g remove r edit e encrypt decrypt export copy c rename
    complete -c sinbo -f -n "__fish_seen_subcommand_from $cmd" -a "(__sinbo_snippets)"
end