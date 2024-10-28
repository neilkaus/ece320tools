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

NUM_LINES=$(cat <(irvedecoder/irvedecoder 4 $REF) | wc -l)

echo "irvedecode'd golden $REF has $NUM_LINES lines, so we'll only compare that many"

diff -u <(irvedecoder/irvedecoder 4 $REF) <(head -n $NUM_LINES $TRACE)

echo "Nice stuff, I couldn't find any (legitimate) differences!"
