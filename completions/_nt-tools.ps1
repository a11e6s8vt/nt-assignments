
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'nt-tools' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'nt-tools'
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
        'nt-tools' {
            [CompletionResult]::new('primes', 'primes', [CompletionResultType]::ParameterValue, 'Search for prime numbers between START and END numbers')
            [CompletionResult]::new('composites', 'composites', [CompletionResultType]::ParameterValue, 'Search for composite numbers between START and END numbers')
            [CompletionResult]::new('composites-pq', 'composites-pq', [CompletionResultType]::ParameterValue, 'Search for composite numbers of the form "p.q" between START and END numbers')
            [CompletionResult]::new('nums-with-primitive-roots', 'nums-with-primitive-roots', [CompletionResultType]::ParameterValue, 'Search for numbers with primitive roots between START and END numbers')
            [CompletionResult]::new('carmichael-nums', 'carmichael-nums', [CompletionResultType]::ParameterValue, 'Carmichael Number search in a range.')
            [CompletionResult]::new('ifactors', 'ifactors', [CompletionResultType]::ParameterValue, 'Finds the Integer Factorisation of a number.')
            [CompletionResult]::new('primality', 'primality', [CompletionResultType]::ParameterValue, 'Primality checking capabilities.')
            [CompletionResult]::new('miller-rabin-liars', 'miller-rabin-liars', [CompletionResultType]::ParameterValue, 'List the Miller-Rabin Liars of a number if any exist')
            [CompletionResult]::new('gcd', 'gcd', [CompletionResultType]::ParameterValue, 'Finds the GCD of two numbers using Euclid''s algorithm.')
            [CompletionResult]::new('quadratic-sieve', 'quadratic-sieve', [CompletionResultType]::ParameterValue, 'Integer Factorisation - Quadratic Sieve.')
            [CompletionResult]::new('pollards-p-minus-1', 'pollards-p-minus-1', [CompletionResultType]::ParameterValue, 'Integer Factorisation - Pollard''s P-1 Algm.')
            [CompletionResult]::new('pollards-rho', 'pollards-rho', [CompletionResultType]::ParameterValue, 'Pollards Rho Alogorithm to find the logarithm modulo p')
            [CompletionResult]::new('modular-pow', 'modular-pow', [CompletionResultType]::ParameterValue, 'Fast Modular Exponentiation')
            [CompletionResult]::new('aks-findr', 'aks-findr', [CompletionResultType]::ParameterValue, 'Finds the ''r'' value for the AKS algorithm.')
            [CompletionResult]::new('list-primitive-roots', 'list-primitive-roots', [CompletionResultType]::ParameterValue, 'List the primitive roots of a number')
            [CompletionResult]::new('ass2q2b', 'ass2q2b', [CompletionResultType]::ParameterValue, 'Assignment 2 - Question 2b - Primitive Roots - Euler''s Totient Function')
            [CompletionResult]::new('ass2q2c', 'ass2q2c', [CompletionResultType]::ParameterValue, 'Assignment 2 - Question 2c - Primitive Roots - Euler''s Totient Function')
            [CompletionResult]::new('quit', 'quit', [CompletionResultType]::ParameterValue, 'quit')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'nt-tools;primes' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;composites' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;composites-pq' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;nums-with-primitive-roots' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;carmichael-nums' {
            [CompletionResult]::new('--method', 'method', [CompletionResultType]::ParameterName, 'Choose the algorithm')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;ifactors' {
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--num1', 'num1', [CompletionResultType]::ParameterName, 'num1')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--num2', 'num2', [CompletionResultType]::ParameterName, 'num2')
            [CompletionResult]::new('--pq', 'pq', [CompletionResultType]::ParameterName, 'pq')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;primality' {
            [CompletionResult]::new('--method', 'method', [CompletionResultType]::ParameterName, 'Choose the primality Checking algorithm')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--num', 'num', [CompletionResultType]::ParameterName, 'num')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;miller-rabin-liars' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--num', 'num', [CompletionResultType]::ParameterName, 'num')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;gcd' {
            [CompletionResult]::new('-a', 'a', [CompletionResultType]::ParameterName, 'a')
            [CompletionResult]::new('--num1', 'num1', [CompletionResultType]::ParameterName, 'num1')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--num2', 'num2', [CompletionResultType]::ParameterName, 'num2')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;quadratic-sieve' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--num', 'num', [CompletionResultType]::ParameterName, 'num')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;pollards-p-minus-1' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--num', 'num', [CompletionResultType]::ParameterName, 'num')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'b')
            [CompletionResult]::new('--base', 'base', [CompletionResultType]::ParameterName, 'base')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;pollards-rho' {
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Primitive Root modulo N')
            [CompletionResult]::new('--primitive-root', 'primitive-root', [CompletionResultType]::ParameterName, 'Primitive Root modulo N')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'b ∈ Z/pZ - Find the logarithm of b to the base r')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'Odd Prime Number')
            [CompletionResult]::new('--modulo', 'modulo', [CompletionResultType]::ParameterName, 'Odd Prime Number')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;modular-pow' {
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Base number which we are raising to some power')
            [CompletionResult]::new('--base', 'base', [CompletionResultType]::ParameterName, 'Base number which we are raising to some power')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Exponent')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'Modulus')
            [CompletionResult]::new('--modulo', 'modulo', [CompletionResultType]::ParameterName, 'Modulus')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;aks-findr' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--num', 'num', [CompletionResultType]::ParameterName, 'num')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;list-primitive-roots' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'n')
            [CompletionResult]::new('--num', 'num', [CompletionResultType]::ParameterName, 'num')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;ass2q2b' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;ass2q2c' {
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 's')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'e')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;quit' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'nt-tools;help' {
            [CompletionResult]::new('primes', 'primes', [CompletionResultType]::ParameterValue, 'Search for prime numbers between START and END numbers')
            [CompletionResult]::new('composites', 'composites', [CompletionResultType]::ParameterValue, 'Search for composite numbers between START and END numbers')
            [CompletionResult]::new('composites-pq', 'composites-pq', [CompletionResultType]::ParameterValue, 'Search for composite numbers of the form "p.q" between START and END numbers')
            [CompletionResult]::new('nums-with-primitive-roots', 'nums-with-primitive-roots', [CompletionResultType]::ParameterValue, 'Search for numbers with primitive roots between START and END numbers')
            [CompletionResult]::new('carmichael-nums', 'carmichael-nums', [CompletionResultType]::ParameterValue, 'Carmichael Number search in a range.')
            [CompletionResult]::new('ifactors', 'ifactors', [CompletionResultType]::ParameterValue, 'Finds the Integer Factorisation of a number.')
            [CompletionResult]::new('primality', 'primality', [CompletionResultType]::ParameterValue, 'Primality checking capabilities.')
            [CompletionResult]::new('miller-rabin-liars', 'miller-rabin-liars', [CompletionResultType]::ParameterValue, 'List the Miller-Rabin Liars of a number if any exist')
            [CompletionResult]::new('gcd', 'gcd', [CompletionResultType]::ParameterValue, 'Finds the GCD of two numbers using Euclid''s algorithm.')
            [CompletionResult]::new('quadratic-sieve', 'quadratic-sieve', [CompletionResultType]::ParameterValue, 'Integer Factorisation - Quadratic Sieve.')
            [CompletionResult]::new('pollards-p-minus-1', 'pollards-p-minus-1', [CompletionResultType]::ParameterValue, 'Integer Factorisation - Pollard''s P-1 Algm.')
            [CompletionResult]::new('pollards-rho', 'pollards-rho', [CompletionResultType]::ParameterValue, 'Pollards Rho Alogorithm to find the logarithm modulo p')
            [CompletionResult]::new('modular-pow', 'modular-pow', [CompletionResultType]::ParameterValue, 'Fast Modular Exponentiation')
            [CompletionResult]::new('aks-findr', 'aks-findr', [CompletionResultType]::ParameterValue, 'Finds the ''r'' value for the AKS algorithm.')
            [CompletionResult]::new('list-primitive-roots', 'list-primitive-roots', [CompletionResultType]::ParameterValue, 'List the primitive roots of a number')
            [CompletionResult]::new('ass2q2b', 'ass2q2b', [CompletionResultType]::ParameterValue, 'Assignment 2 - Question 2b - Primitive Roots - Euler''s Totient Function')
            [CompletionResult]::new('ass2q2c', 'ass2q2c', [CompletionResultType]::ParameterValue, 'Assignment 2 - Question 2c - Primitive Roots - Euler''s Totient Function')
            [CompletionResult]::new('quit', 'quit', [CompletionResultType]::ParameterValue, 'quit')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'nt-tools;help;primes' {
            break
        }
        'nt-tools;help;composites' {
            break
        }
        'nt-tools;help;composites-pq' {
            break
        }
        'nt-tools;help;nums-with-primitive-roots' {
            break
        }
        'nt-tools;help;carmichael-nums' {
            break
        }
        'nt-tools;help;ifactors' {
            break
        }
        'nt-tools;help;primality' {
            break
        }
        'nt-tools;help;miller-rabin-liars' {
            break
        }
        'nt-tools;help;gcd' {
            break
        }
        'nt-tools;help;quadratic-sieve' {
            break
        }
        'nt-tools;help;pollards-p-minus-1' {
            break
        }
        'nt-tools;help;pollards-rho' {
            break
        }
        'nt-tools;help;modular-pow' {
            break
        }
        'nt-tools;help;aks-findr' {
            break
        }
        'nt-tools;help;list-primitive-roots' {
            break
        }
        'nt-tools;help;ass2q2b' {
            break
        }
        'nt-tools;help;ass2q2c' {
            break
        }
        'nt-tools;help;quit' {
            break
        }
        'nt-tools;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
