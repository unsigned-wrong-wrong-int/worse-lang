# Worse

Worse is an esoteric programming language based on combinatory logic.

## Syntax

```
Expr ::= "+"
       | "-"
       | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
       | Expr Expr "."
```

`+`, `-`, and ASCII digits are the combinators described below.

`.` is application operator. `x y .` means "x is applied to y" (i.e. `(x y)`).

Whitespaces and any characters between `#` and end-of-line are ignored.

## Primitive Combinators

ASCII digits `0`-`9` represents Church numerals (https://en.wikipedia.org/wiki/Church_encoding).

```
0 f x = x
1 f x = f x
2 f x = f (f x)
3 f x = f (f (f x))
:
9 f x = f (f (f (f (f (f (f (f (f x))))))))
```

`+` is the addition function.

```
+ m n f x = m f (n f x)
```

`-` is the subtraction function.

```
- m n = n P m
where
  P n f x = n (Q f) (K x) I
  Q f g h = h (g f)
  K x y   = x
  I x     = x
```

## Turing Completeness

Surprisingly, it is possible to write S and K combinators using only two combinators, `+` and `-`.

```
K = - + (+ -) (- - -) (- + (+ -))
S = + (- + (+ -)) (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)))
```

<details>

```
K combinator
  - + (+ -) (- - -) (- + (+ -)) x y
= + - P + (- - -) (- + (+ -)) x y
= - + (P + (- - -)) (- + (+ -)) x y
= P + (- - -) P + (- + (+ -)) x y
= + (Q (- - -)) (K P) I + (- + (+ -)) x y
= Q (- - -) I (K P I +) (- + (+ -)) x y
= K P I + (I (- - -)) (- + (+ -)) x y
= P + (I (- - -)) (- + (+ -)) x y
= + (Q (I (- - -))) (K (- + (+ -))) I x y
= Q (I (- - -)) I (K (- + (+ -)) I x) y
= K (- + (+ -)) I x (I (I (- - -))) y
= - + (+ -) x (I (I (- - -))) y
= + - P + x (I (I (- - -))) y
= - + (P + x) (I (I (- - -))) y
= P + x P + (I (I (- - -))) y
= + (Q x) (K P) I + (I (I (- - -))) y
= Q x I (K P I +) (I (I (- - -))) y
= K P I + (I x) (I (I (- - -))) y
= P + (I x) (I (I (- - -))) y
= + (Q (I x)) (K (I (I (- - -)))) I y
= Q (I x) I (K (I (I (- - -))) I y)
= K (I (I (- - -))) I y (I (I x))
= I (I (- - -)) y (I (I x))
= I (- - -) y (I (I x))
= - - - y (I (I x))
= - P - y (I (I x))
= - P P y (I (I x))
= P P P y (I (I x))
= P (Q P) (K y) I (I (I x))
= Q P (Q (K y)) (K I) I (I (I x))
= K I (Q (K y) P) I (I (I x))
= I I (I (I x))
= I (I (I x))
= I (I x)
= I x
= x

S combinator
  + (- + (+ -)) (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -))) x y z
= - + (+ -) x (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= + - P + x (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= - + (P + x) (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= P + x P + (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= + (Q x) (K P) I + (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= Q x I (K P I +) (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= K P I + (I x) (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= P + (I x) (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) z
= + (Q (I x)) (K (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y)) I z
= Q (I x) I (K (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) I z)
= K (+ + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y) I z (I (I x))
= + + (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -)) x y z (I (I x))
= + x (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -) x y) z (I (I x))
= x z (- + (+ -) (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -) x y z (I (I x)))
= x z (+ - P + (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) (- - -) x y z (I (I x)))
= x z (- + (P + (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -))) (- - -) x y z (I (I x)))
= x z (P + (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) P + (- - -) x y z (I (I x)))
= x z (+ (Q (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -))) (K P) I + (- - -) x y z (I (I x)))
= x z (Q (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) I (K P I +) (- - -) x y z (I (I x)))
= x z (K P I + (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -))) (- - -) x y z (I (I x)))
= x z (P + (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -))) (- - -) x y z (I (I x)))
= x z (+ (Q (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) (K (- - -)) I x y z (I (I x)))
= x z (Q (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -))) I (K (- - -) I x) y z (I (I x)))
= x z (K (- - -) I x (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (- - - x (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (- P - x (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (- P P x (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (P P P x (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (P (Q P) (K x) I (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (Q P (Q (K x)) (K I) I (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (K I (Q (K x) P) I (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (I I (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (I (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)))) y z (I (I x)))
= x z (I (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -))) y z (I (I x)))
= x z (I (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -)) y z (I (I x)))
= x z (+ (+ (- + (+ -) (- - -) (- + (+ -)))) (- - - -) y z (I (I x)))
= x z (+ (- + (+ -) (- - -) (- + (+ -))) y (- - - - y z) (I (I x)))
= x z (- + (+ -) (- - -) (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (+ - P + (- - -) (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (- + (P + (- - -)) (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (P + (- - -) P + (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (+ (Q (- - -)) (K P) I + (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (Q (- - -) I (K P I +) (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (K P I + (I (- - -)) (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (P + (I (- - -)) (- + (+ -)) (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (+ (Q (I (- - -))) (K (- + (+ -))) I (- - - - y z) (y (- - - - y z) (I (I x))))
= x z (Q (I (- - -)) I (K (- + (+ -)) I (- - - - y z)) (y (- - - - y z) (I (I x))))
= x z (K (- + (+ -)) I (- - - - y z) (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (- + (+ -) (- - - - y z) (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (+ - P + (- - - - y z) (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (- + (P + (- - - - y z)) (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (P + (- - - - y z) P + (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (+ (Q (- - - - y z)) (K P) I + (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (Q (- - - - y z) I (K P I +) (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (K P I + (I (- - - - y z)) (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (P + (I (- - - - y z)) (I (I (- - -))) (y (- - - - y z) (I (I x))))
= x z (+ (Q (I (- - - - y z))) (K (I (I (- - -)))) I (y (- - - - y z) (I (I x))))
= x z (Q (I (- - - - y z)) I (K (I (I (- - -))) I (y (- - - - y z) (I (I x)))))
= x z (K (I (I (- - -))) I (y (- - - - y z) (I (I x))) (I (I (- - - - y z))))
= x z (I (I (- - -)) (y (- - - - y z) (I (I x))) (I (I (- - - - y z))))
= x z (I (- - -) (y (- - - - y z) (I (I x))) (I (I (- - - - y z))))
= x z (- - - (y (- - - - y z) (I (I x))) (I (I (- - - - y z))))
= x z (- P - (y (- - - - y z) (I (I x))) (I (I (- - - - y z))))
= x z (- P P (y (- - - - y z) (I (I x))) (I (I (- - - - y z))))
= x z (P P P (y (- - - - y z) (I (I x))) (I (I (- - - - y z))))
= x z (P (Q P) (K (y (- - - - y z) (I (I x)))) I (I (I (- - - - y z))))
= x z (Q P (Q (K (y (- - - - y z) (I (I x))))) (K I) I (I (I (- - - - y z))))
= x z (K I (Q (K (y (- - - - y z) (I (I x)))) P) I (I (I (- - - - y z))))
= x z (I I (I (I (- - - - y z))))
= x z (I (I (I (- - - - y z))))
= x z (I (I (- - - - y z)))
= x z (I (- - - - y z))
= x z (- - - - y z)
= x z (- P - - y z)
= x z (- P P - y z)
= x z (P P P - y z)
= x z (P (Q P) (K -) I y z)
= x z (Q P (Q (K -)) (K I) I y z)
= x z (K I (Q (K -) P) I y z)
= x z (I I y z)
= x z (I y z)
= x z (y z)
```

</details>

## Evaluation Strategy and I/O

Worse is purely functional language with lazy evaluation.

I/O is represented using Church pairs.

```
cons x y f = f x y
car p      = p K
cdr p      = p 0
```

Program `p` is run in following steps.

1. If `car p` is Church numeral `n`, then
  - If `n` is 256, then
    1. Terminate execution of the program.
  - If `n` is 257, then
    1. Get byte `m` from input stream. (EOF is mapped to 256)
    2. Run `cdr p m`.
  - If `n` is less than 256, then
    1. Append byte `n` to output stream.
    2. Run `cdr p`.
  - If `n` is any other value then, raise error.
2. Otherwise, raise error.

## Hello World Program

This program prints `Hello, world!` to stdout.

```
-+.1.9+8..0..-+.1.-+.1.+22+5..0...1..-+.1.-+.1.26.+3..0..-+.1.-+.1.26.+3..0..-+
.1.-+.1.+26..1.+3..0..-+.1.-+.1.+26..8..-+.1.-+.1.52..-+.1.-+.1.-72..9..-+.1.-+
.1.+26..1.+3..0..-+.1.-+.1.+26..2.+3..0..-+.1.-+.1.26.+3..0..-+.1.-+.1.22+5..0.
..-+.1.-+.1.+52..1..-+.1.-+.1.5+2..0..-+.1.-+.1.44..0..........................
...
```

Other programs are avaliable in ./samples/ directory.
