#!/bin/bash

QS=../exerc3/main
HS=./haskell-heap-sort/HeapSort.hs

echo -e "\e[1mShell Script for comparing the different sorting algorithms\e[0m"
echo -e "\e[34mQuick Sort and Merge Sort from $QS.c...\e[0m"

out="Result from $QS:
"
out+=$($QS | tail -2)
time_out="Time for Quick Sort / Merge Sort:
"
time_out+=$( { time $QS >/dev/null; } 2>&1 )
echo -e "\e[32mdone.\e[0m"
echo -e "\n\e[34mMerge Sort from $HS...\e[0m"
out+="
Result from $HS:
"
time_out+="
Time for Haskell Heap Sort:
"
out+=$(runhaskell $HS | tail -1)
time_out+=$( { time runhaskell $HS >/dev/null; } 2>&1 )
echo -e "\e[32mdone.\e[0m"
echo "$out"
echo -e "\e[1;34mTime for both programs:\e[0m"
echo "$time_out"

