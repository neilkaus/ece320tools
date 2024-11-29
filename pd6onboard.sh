#!/bin/bash

# Instructions for use with pd6autotestboard.sh
# put this file on the xilinx board in /tmp/ece320_to_pynq with the contents from 'make prepare'
# put the tests (.x files) you want to run into /tmp/data
# run this file with source /tmp/ece320_to_pynq/pd6onboard.sh
# the resultant traces will be in /tmp/traces
# copy those traces back into your pd6 project in <project>/verif/sim/board
# then from ece320tools run: source pd6autotestboard.sh <path to your project>
# you will get a grade (a differing stack pointer (x2) is expected for simple programs due to different memory sizes)

for xfile in /tmp/data/*.x; do
    echo "$xfile"
    source /tmp/ece320_to_pynq/run.sh "$xfile"
done
mkdir -p /tmp/traces
mv /tmp/data/*.trace /tmp/traces/