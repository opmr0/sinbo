Register-ArgumentCompleter -Native -CommandName sinbo -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $words = $commandAst.CommandElements
    $count = $words.Count

    $subcommands = @('get','g','add','a','list','l','remove','r','edit','e','search','s','encrypt','decrypt','export','import','copy','c','rename','completions')

    $snippetCommands = @('get','g','remove','r','edit','e','encrypt','decrypt','export','copy','c','rename')

    if ($count -eq 1 -or ($count -eq 2 -and $wordToComplete -ne '')) {
        $subcommands | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
            [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
        }
    } elseif ($count -ge 2 -and $snippetCommands -contains $words[1].Value) {
        sinbo list-names 2>$null | Where-Object { $_ -like "$wordToComplete*" } | ForEach-Object {
            [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_)
        }
    }
}