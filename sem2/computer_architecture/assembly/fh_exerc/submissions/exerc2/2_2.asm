.data
.align 0
is_odd: .asciiz "Odd Number:"
newline: .asciiz "\n"
is_even: .asciiz "Even Number:"

.text
.globl main

main:
	addi $t0, $zero, -1 # x
	# is negative
	#blt $t0, $zero, isNegative
	#isNegative:
	#	sub $t0, $zero, $t0
	jal ISODD
	jal ISEVEN
	
ISODD:
	andi $s1, $t0, 1
	
	jr $ra

ISEVEN:
	jal ISODD
	seq $s2, $s1, 0
	
	j end

end:
	li $v0, 4
	la $a0, is_odd
	syscall
	la $a0, newline
	syscall
	
	li $v0, 1
	move $a0, $s1
	syscall
	li $v0, 4
	la $a0, newline
	syscall
	
	li $v0, 4
	la $a0, is_even
	syscall
	la $a0, newline
	syscall
	
	li $v0, 1
	move $a0, $s2
	syscall
	li $v0, 4
	la $a0, newline
	syscall

	
