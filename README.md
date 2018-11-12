[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

# Smoke

Smoke is a simple programming language now being interpreted in Rust, the goal is just to be a
simple learning experience and simple scripting language with familiar syntax. No idea if
it'll get finished or go anywhere, nor do I care.

## How to use smoke

Build it using `cargo build` in the main directory once you have Rust installed, install instructions can be found on [The Rust github](https://github.com/rust-lang/rust)

Then create a file with the extension `.smo`, run it using `./smoke FILENAME.smo`

## What you can do right now

At the moment, Smoke only allows for `print`'ing and `stop`'ing.
An example script would be:

[main.smo]

```
print("Hello, World!")
stop()
print("This will not print because we exited on the previous line!")
```

## Features upcoming:

Working on creating variables, and will get that done once I get error checking for a missing closed parantheses on commands working.