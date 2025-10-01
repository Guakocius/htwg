.data
A: .word 1000
B: .word 3040
C: .word 9196
n: .word 3

.text
la $a0, A
la $a1, B
la $a2, C
la $t0, n
lw $a3, 0($t0)
.globl main

main:	jal ARRSUM
	li $v0, 10
	syscall

ARRSUM: beq $a3, $zero, BACK
	lw $s0,0($a0)
	lw $s1, 0($a1)
	add $s2,$s0,$s1
	sw $s2,0($a2)
	addi $a0,$a0,4
	addi $a1,$a1,4
	addi $a2,$a2,4
	addi $a3,$a3,-1
	j ARRSUM
BACK: 	jr $ra