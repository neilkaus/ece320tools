# ece320tools

"This guy's doing *another* autograder?"

Yep, along with Nick Chan! :)

We didn't make this earlier on in the term because, until now, the golden traces we've been provided have been adequate for testing.

However, especially with PD4 and beyond, dealing with don't care values in the traces can be a bit painful. That's where ece320tools comes in!

ece320tools are in part based off of [IRVE](https://github.com/angry-goose-initiative/irve), the first part of our Linux-capable RISC-V emulator and CPU implementation project. If you're interested in learning more about it, [check out our Github organization here!](https://github.com/angry-goose-initiative)

Whelp hope this helps people out!

- JZJ

## PD4

Simply use the `pd4tracecompare.sh` script in place of doing a diff. Your trace must be first and the golden trace must be second.

This will build a helper tool called `irvedecoder` that uses code from our emulator to compare the two traces. You need to have a new enough `g++` installed that can compile C++17 code. (Also you need `diff`, `make`, and `bash`, which I'd assume you have...)

As opposed to ignoring don't care values, this tries to replace them as they appear in the golden trace file with more realistic "garbage" values.
So it's possible there may be some false positives, you have been warned...

Example usage:

```bash

$ ./pd4tracecompare.sh path/to/your/pd4_trace.trace path/to/corresponding/golden_trace.trace
Making irvedecoder
~/local_work/ece320/ece320megarepo/ece320tools/irvedecoder ~/local_work/ece320/ece320megarepo/ece320tools
g++ -std=c++17 -fsanitize=address -o irvedecoder decode.cpp main.cpp
[...]

```

## PD5

todo

## PD6

todo
