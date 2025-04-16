# Introduction

MathPotato is a programming language focusing on mathematics.

## About the language

The language is built on Rust according to the following:

- there is a brand new syntax which intends to make life easier when it comes to
  mathematics
- the language has its own lexer
- the language has its own interpreter and Rust is behind all the magic

Why all this?

I assume majority of the "I need a more comfortable syntax here" can be solved
by Rust macros, but, to be honest, nah... I don't want to write macros.
The still open question is where creating macros will be uncomfortable and where
are their limitations.
I don't have the motivation to find them.
It is not my goal to find them.

Why not adding the new syntax elements to Rust?

Just think it through.
Rust is a generic programming language and adding new, math specific syntax
elements, would increase the complexity of the language management and
maintenance.
On the other hand, I don't think the community would allow it.

Why not creating a new crate or using existing ones?

My study goal includes compilers and interpreters, and "just" using existing
libraries or creating new ones do not satisfy my goal.

Why Rust and why not a compiler?

In aerospace engineering Rust is gaining space and having experience with the
language in scientific calculation topic would mean some advantage for me.
Creating a compiler, creating a real new language, would be a overwhelming and
would not mean knowledge with Rust.

What type of interpreter will be used?

Tree-walking interpreter.

## Motivation

I study mathematics, mainly calculus and applied mathematics, because I want to
be an aerospace engineer.
I have some experience in programming and how languages work topic was always
interesting for me.
So, adding the two together results in this project.

Since my studies are more important than this project, there will be slow
progress.
