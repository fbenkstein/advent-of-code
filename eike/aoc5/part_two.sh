#!/bin/bash

# execute with the given letter deleted
sed s/$1//gI < input5_spaced > input_wo_$1.tmp
./reduction.pl $(< input_wo_$1.tmp)
rm input_wo_$1.tmp

