#!/bin/sh
testrig=org.antlr.v4.gui.TestRig
grammar=expr.grammar.Expr

cmd="java -cp Expr.jar $testrig $grammar tokens -tokens $1"
echo $cmd
eval $cmd

cmd="java -cp Expr.jar expr.ExprTokenizer < $1"
echo $cmd
eval $cmd
