#!/bin/zsh
declare -a FILES=("benotung" "fachnote" "fachnoten_liste")
SRC="../../aufgabe5"

for f in "${FILES[@]}"; do
  compile_command="g++ -c -o $f.o $SRC/$f.cpp"
  lib_command="ar rcs libaufgabe6.a $f.o"
  echo "$compile_command"
  eval "$compile_command"
  echo "$lib_command"
  eval "$lib_command"
  if [ $? -ne 0 ]; then
    echo "Error: Build failed!"
    exit 1
  fi
  echo "Build Successful!"
done
exit 0
