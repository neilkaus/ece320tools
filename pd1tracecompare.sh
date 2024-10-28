#!/bin/bash
#Copyright (C) 2024 John Jekel
#
#Compares a trace file (first argument) against a golden trace file (second argument)
set -e

TRACE=$1
REF=$2

echo Comparing $TRACE trace against $REF reference

NUM_LINES=$(grep "[F]" $REF | wc -l)

echo "$REF has $NUM_LINES [F] lines, so we'll only compare that many"

#Returns 1 if they mismatch
diff -u <(grep "[F]" $REF) <(grep "[F]" $TRACE | head -n $NUM_LINES)
