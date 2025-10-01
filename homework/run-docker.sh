#!/bin/zsh

docker run -it --rm -v $(pwd):/workspace -w /workspace personal-bsys-homework:latest zsh
