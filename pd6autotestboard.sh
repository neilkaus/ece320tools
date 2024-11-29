#!/bin/bash

# There are instructions to generate the traces needed from the board in the pd6onboard.sh file

echo "This script will automatically run all the benchmarks you've placed in $1/verif/data, and compare your trace output to the golden trace files using JZJ's autograder!"
echo "To add more benchmarks, copy the .x files for the desired benchmarks from the rv32-benchmarks repo into $1/verif/data." 

source $1/env.sh

num_benchmarks=0
num_passed=0
for tfile in $1/verif/sim/board/*; do
    num_benchmarks=$(($num_benchmarks + 1));
    benchmark=$(basename "$tfile")

    TEMP_FILE=$(mktemp)
    echo "[W] 00000000 0 00 00000000" | cat - "$tfile" > "$TEMP_FILE"
    output=$(cargo run --release --bin pd6boarddiff $1/verif/golden_sim/$benchmark "$TEMP_FILE")
    echo "$output"
    rm -f "$TEMP_FILE"

    if [[ $output != *"At least one error"* ]]; then
       num_passed=$(( num_passed + 1 )) 
    fi
done
echo "$num_passed/$num_benchmarks passed! See output for details on (potential) error messages"
echo "Thanks for using PD6 autotest :)"
