#!/bin/bash
#Copyright (C) 2024 John Jekel
#
#Compares a trace file (first argument) against a reference .x file (second argument)
set -e

TRACE=$1
REF=$2

echo Making irvedecoder
pushd irvedecoder
make
popd

echo Comparing $TRACE trace against $REF reference

NUM_LINES=$(cat <(irvedecoder/irvedecoder t $REF) | wc -l)

echo "irvedecoder'ed $REF has $NUM_LINES lines, so we'll only compare that many"

#Returns 1 if they mismatch
diff -u <(irvedecoder/irvedecoder t $REF) <(grep "[D]" $TRACE | head -n $NUM_LINES)
