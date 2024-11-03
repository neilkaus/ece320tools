#!/bin/bash
echo "This script will automatically run all the benchmarks you've placed in $1/verif/data, and compare your trace output to the golden trace files using JZJ's autograder!"
echo "To add more benchmarks, copy the .x files for the desired benchmarks from the rv32-benchmarks repo into $1/verif/data." 

source $1/env.sh

num_benchmarks=0
num_passed=0
for xfile in $1/verif/data/*; do
    num_benchmarks=$(($num_benchmarks + 1));
    benchmark=$(basename "$xfile" .x)

    make -C $1/verif/scripts -s run TEST=test_pd MEM_PATH=$1/verif/data/$benchmark.x

    output=$(cargo run --bin betterpd4diff $1/verif/golden/$benchmark.trace $1/verif/sim/verilator/test_pd/$benchmark.trace)
    echo "$output"

    if [[ $output != *"At least one error"* ]] then
       num_passed=$(( num_passed + 1 )) 
    fi
done
echo "$num_passed/$num_benchmarks passed! See output for details on (potential) error messages"
echo "Thanks for using autotest :)"
