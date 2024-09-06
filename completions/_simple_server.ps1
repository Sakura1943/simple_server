
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'simple_server' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'simple_server'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'simple_server' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Server port')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'Server port')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Server bind address')
            [CompletionResult]::new('--bind', '--bind', [CompletionResultType]::ParameterName, 'Server bind address')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Server bind directory')
            [CompletionResult]::new('--dir', '--dir', [CompletionResultType]::ParameterName, 'Server bind directory')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Server index file')
            [CompletionResult]::new('--index-file', '--index-file', [CompletionResultType]::ParameterName, 'Server index file')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
