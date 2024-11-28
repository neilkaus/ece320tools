#!/bin/bash
echo "This script will automatically run all the benchmarks you've placed in $1/verif/data, and compare your trace output to the golden trace files using JZJ's autograder!"
echo "To add more benchmarks, copy the .x files for the desired benchmarks from the rv32-benchmarks repo into $1/verif/data." 

source $1/env.sh

make -C $1/build/scripts bitstream
mkdir -p $1/verif/sim/post-synth
num_benchmarks=0
num_passed=0
for xfile in $1/verif/data/*; do
    num_benchmarks=$(($num_benchmarks + 1));
    benchmark=$(basename "$xfile" .x)
    TRACE_FILE="$1/verif/sim/post-synth/$benchmark.trace"

    make -C $1/build/scripts post-synth-sim MEM_PATH=$1/verif/data/$benchmark.x > "$TRACE_FILE"
    
    TEMP_FILE=$(mktemp)
    awk '/\[W\] 00000000 0 00 00000000/ {found=1} found' "$TRACE_FILE" > "$TEMP_FILE"
    echo "[W] 00000000 0 00 00000000" | cat - "$TEMP_FILE" > "$TRACE_FILE"
    awk '/^\[W\]/ {last_match=NR} {lines[NR]=$0} END {for (i=1; i<=last_match; i++) print lines[i]}' "$TRACE_FILE" > "$TEMP_FILE"
    mv "$TEMP_FILE" "$TRACE_FILE"
    rm -f "$TEMP_FILE"

    output=$(cargo run --release --bin pd6boarddiff $1/verif/golden_sim/$benchmark.trace "$TRACE_FILE")
    echo "$output"

    if [[ $output != *"At least one error"* ]]; then
       num_passed=$(( num_passed + 1 )) 
    fi
done
echo "$num_passed/$num_benchmarks passed! See output for details on (potential) error messages"
echo "Thanks for using PD6 autotest :)"
