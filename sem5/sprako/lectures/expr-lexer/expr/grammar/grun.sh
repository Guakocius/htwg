#!/bin/sh
t="java -cp Expr.jar org.antlr.v4.gui.TestRig grammar.Expr tokens -tokens $1 > ${1}.tokens"
echo $t
eval $t
