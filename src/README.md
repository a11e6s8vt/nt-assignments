# How To Guide for nt-tools

## Introduction
This document describes how to use the command line tool developed as part of the assignments.

## How To
1. Double clicking the “.exe” file will open the app. Below is a screen shot of the landing page of the app.

![Landing Page](/notes/images/landing_page.png?raw=true "Landing Page")

2. Typing “help” or just pressing the “Enter” ←- key will display the help.

![Help Page](/notes/images/help_page.png?raw=true "Help Page")

#### Command Syntax
1. primes
```
primes -s 2800 -e 3100
```

The below screenshot shows a sample output:

![List of prime numbers in a range](/notes/images/list_primes.png?raw=true "List of prime numbers in a range")

2. composites
```
composites --start 2800 --end 3100
```

The below screenshot shows a sample output:

![List of composite numbers in a range](/notes/images/list_composites.png?raw=true "List of composite numbers in a range")

3. composites-pq
```
composites-pq --start 2800 --end 3100
```

The below screenshot shows a sample output:

![List of composite numbers of the form N = P.Q in a range](/notes/images/composites-pq_list.png?raw=true "List of composite numbers of the form N = P.Q in a range")

4. nums-with-primitive-roots

```
nums-with-primitive-roots --start 600 --end 750
```

The below screenshot shows a sample output:

![List of numbers with primitive roots in a range](/notes/images/nums-with-primitive-roots_list.png?raw=true "List of numbers with primitive roots in a range")

5. carmichael-nums
```
# Carmichael Numbers using FLT
1. carmichael-nums --method fermat --start 2800 --end 3100
# Carmichael Numbers using Korselt criteria
2. carmichael-nums --method korselt --start 2800 --end 3100
```

The below screenshot shows a sample output:

![Carmichael Numbers in a range](/notes/images/carmichael-nums_list.png?raw=true "Carmichael Numbers in a range")

6. ifactors

```
#Integer Factorisation of a single number using trial and error
1. ifactors --num1 2452

# Integer factorisation of a range of numbers
2. ifactors --num1 2800 --num2 2850
```

The below screenshot shows a sample output:

![Integer Factorisation](/notes/images/ifactors.png?raw=true "Integer Factorisation")

7. primality

```
# Primality check using gcd test
primality --method gcd --num 71
# Primality check using trial division
primality --method trial-division --num 71
# Primality check using Miller Rabin
primality --method miller-rabin --num 71
# Primality check using AKS Algm
primality --method aks --num 71
# Primality check using FLT - Not Implemented
primality --method fermat --num 71
```

The below screenshot shows a sample output:

![List of prime numbers in a range](/notes/images/primality.png?raw=true "List of prime numbers in a range")

8. miller-rabin-liars

```
miller-rabin-liars --num 2869
```

The below screenshot shows a sample output:

![Find the Miller-Rabin Liars of a number](/notes/images/miller-rabin-liars.png?raw=true "Find the Miller-Rabin Liars of a number")

9. gcd

```
gcd --num1 2000 --num2 200
```

The below screenshot shows a sample output:

![GCD of two numbers](/notes/images/gcd.png?raw=true "GCD of two numbers")

10. quadratic-sieve

```
quadratic-sieve --num 391
```

The below screenshot shows a sample output:

![Quadratic Sieve Evaluation Matrix](/notes/images/quadratic-sieve.png?raw=true "Quadratic Sieve Evaluation Matrix")

11. pollards-p-minus-1

```
pollards-p-minus-1 --num 78719 --base 13
```

The below screenshot shows a sample output:

![Pollard’s P-1 Factorisation](/notes/images/pollards-p-minus-one.png?raw=true "Pollard’s P-1 Factorisation")

12. pollards-rho

```
pollards-rho --primitive-root 21 -b 47 -m 71
```

The below screenshot shows a sample output:

Figure 14: Discrete Logarithm - Pollard’s Rho

![Discrete Logarithm - Pollard’s Rho](/notes/images/pollards-rho.png?raw=true "Discrete Logarithm - Pollard’s Rho")

13. modular-pow

```
modular-pow -b 26 -e 32 -m 53
```

The below screenshot shows a sample output:

![Modular Exponentiation](/notes/images/mod-pow.png?raw=true "Modular Exponentiation")

14. aks-findr

```
aks-findr --num 71
```

The below screenshot shows a sample output:

!['r' value of AKS Algm](/notes/images/aks-findr.png?raw=true "'r' value of AKS Algm")

15. list-primitive-roots

```
list-primitive-roots --num 17
```

The below screenshot shows a sample output:

![List Primitive Roots of a number](/notes/images/list_primitive-roots.png?raw=true "List Primitive Roots of a number")

16. ass2q2b

```
ass2q2b --start 50 --end 100
```

The below screenshot shows a sample output:

![Assignment No. 2 Question 2(b) - Number of Primitive Roots](/notes/images/primitive-roots-count.png?raw=true "Assignment No. 2 Question 2(b) - Number of Primitive Roots")

17. ass2q2c

```
ass2q2c --start 50 --end 100
```

The below screenshot shows a sample output:

![Number’s of the form p^k, 2p^k](/notes/images/p_k_2p_k.png?raw=true "Number’s of the form p^k, 2p^k")

18. ass2q3d

```
ass2q3d --start 2800 --end 2850
```

The below screenshot shows a sample output:

![Primitive Roots & Numbers of form N = P.Q](/notes/images/ass2q3d.png?raw=true "Primitive Roots & Numbers of form N = P.Q")

19. aks-failed-steps-for-n

```
aks-failed-steps-for-n --start 2800 --end 3100
```

The below screenshot shows a sample output:

![List of numbers failed the AKS at each of steps](/notes/images/aks-failure-steps.png?raw=true "List of numbers failed the AKS at each of steps")

20. clear or cls

Clears the screen.

21. quit or exit
