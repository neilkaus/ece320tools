#!/bin/bash
#Copyright (C) 2024 John Jekel
#
#Compares a trace file (first argument) against a golden trace file (second argument)
set -e

TRACE=$1
REF=$2

echo Making irvedecoder
pushd irvedecoder
make
popd

echo Comparing $TRACE trace against $REF reference

NUM_LINES=$(cat <(irvedecoder/irvedecoder e $REF) | wc -l)

echo "irvedecode'd golden $REF has $NUM_LINES [R] and [E] lines, so we'll only compare that many"

diff -u <(irvedecoder/irvedecoder e $REF) <(grep -E '^\[(E|R)\]' $TRACE | head -n $NUM_LINES)
