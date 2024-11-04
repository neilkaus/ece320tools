# ece320tools

"This guy's doing *another* autograder?"

Yep, along with Nick Chan! :)

We didn't make this earlier on in the term because, until now, the golden traces we've been provided have been adequate for testing.

However, especially with PD4 and beyond, dealing with don't care values in the traces can be a bit painful. That's where ece320tools comes in!

ece320tools are in part based off of [IRVE](https://github.com/angry-goose-initiative/irve), the first part of our Linux-capable RISC-V emulator and CPU implementation project. If you're interested in learning more about it, [check out our Github organization here!](https://github.com/angry-goose-initiative)

Whelp hope this helps people out!

\- JZJ

## PD4

### New Rust decoder-based Trace Comparison

This should (hopefully) have less false positives than the IRVE-based trace comparison, since this software decoder is more general purpose than
one just torn out of an emulator.

Simply do `cargo run --bin betterpd4diff path/to/golden_trace.trace path/to/your_trace.trace` from the root of the repo (you must have Rust installed)!

Note the golden trace must be first and your trace must be second.

Example usage:

```bash
cargo run --bin betterpd4diff ~/example_traces/golden_bubble_sort.trace ~/example_traces/our_bubble_sort.trace
   Compiling ece320tools v0.0.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running `target/debug/betterpd4diff /home/jzj/example_traces/golden_BubbleSort.trace /home/jzj/example_traces/bad_BubbleSort.trace`
At least one error on line 14:
  Golden: [D] 01000008 37 0f 00 10 0 00 01000000 10
  Yours:  [D] 01000008 37 0e aa 10 0 00 01000000 00
  Golden Disassembly: lui x15, 4096
  Errors:
    Error 1: RDs do not match!
End of error report for line 14.
Done! If you didn't see any errors above, then you (should) be good!
```

Notice how it complains about the rd mismatch, but not about rs1: because lui doesn't have an rs1! :)

### IRVE-based Trace Comparison

Simply use the `pd4tracecompare.sh` script in place of doing a diff. Your trace must be first and the golden trace must be second (opposite of the Rust decoder for some reason haha).

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

### Autotesting

If you want to automatically check against all of the benchmarks (`.x`) files you've placed in `verif/data`, you can use the `autotest.sh` script, which will automatically simulate all of the benchmarks in your `verif/data` directory, and invoke the autograder to compare them!

Usage:
```bash
$ source autotest.sh <path-to-project-root>
```
e.g. 
```bash
$ source autotest.sh ~/ece_320/rhvisram-pd4
```

Since my pd4 is located at `~/ece_320/rhvisram-pd4`.

To test against all the benchmarks, simply copy the `.x` files in the `rv32-benchmarks` repo into your `verif/data` directory, and invoke `autotest.sh`!

## PD5

todo

## PD6

todo
