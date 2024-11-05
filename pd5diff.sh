#!/bin/bash
#Copyright (C) 2024 John Jekel
#
#Compares a trace file (first argument) against a golden trace file (second argument)

cargo run --release --bin pd5diff -- $@
