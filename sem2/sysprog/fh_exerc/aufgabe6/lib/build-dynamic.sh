#!/bin/zsh
declare -a FILES=("benotung" "fachnote" "fachnoten_liste")
SRC="../../aufgabe5"
declare -a OBJECTS=()

for f in "${FILES[@]}"; do
  compile_command="g++ -Wall -fpic -c -o $f.o $SRC/$f.cpp"
  echo "$compile_command"
  eval "$compile_command"
  if [ $? -ne 0 ]; then
    echo "Error: Build failed!"
    exit 1
  fi
  OBJECTS+=("$f.o")
done

lib_command="g++ -shared -o libaufgabe6.so ${OBJECTS[*]}"
echo "$lib_command"
eval "$lib_command"
if [ $? -ne 0 ]; then
  echo "Error: Build failed!"
  exit 1
fi
echo "Build successful!"
exit 0
