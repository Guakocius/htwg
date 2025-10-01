.data
newline : .asciiz "\n"
isO : .asciiz "isodd:\n"
isE : .asciiz "iseven:\n"
sizeB : .asciiz "Size of B:\n"
iOfA : .asciiz "A[i]:\n"
tab : .asciiz "\t"
itemsB : .asciiz "Values of B:\n"

n: .word 6
A: .word 3, 4, 6, 8, 11, 13
B: .space 24

.text
.globl main
main:
	li $t0, 6
	lw $t1, n	# size
	la $t2, A	# Array A
	la $t3, B	# Array B
	add $t4, $t4, $zero	# index i
	li $t5, 0	# index j
	li $t6, 4
	li $t8, 0
	jal fillWzeros
	jal reset
	li $v0, 4
	la $a0, iOfA
	syscall
	jal evenElem

evenElem:
	bge $t4, $t1, printBArray	# end => i>=n
	
	lw $s2, 0($t2)
	
	li $v0, 1
	move $a0, $s2
	syscall
	
	li $v0, 4
	la $a0, newline
	syscall
	
	jal ISEVEN
	
increment:
	addi $t4, $t4, 1
	add $t2, $t2, $t6
	j evenElem

fillWzeros:
	bge $t4, $t1, reset
	sw $zero, 0($t3)

	addi $t4, $t4, 1
	add $t3, $t3, $t6
	j fillWzeros

reset:
	la $t3, B
	li $t4, 0
	li $t5, 0
	jr $ra

printBArray:
	jal reset
	li $v0, 4
	la $a0, itemsB # Items of B
	syscall
	
	loop:
		bge $t4, $t1, end
		lw $t7, 0($t3)
		li $v0, 1
		move $a0, $t7 # B[j]
		syscall
		li $v0, 4
		la $a0, tab # \t
		syscall
		
		addi $t4, $t4, 1
		add $t3, $t3, $t6
		j loop
ISODD:
	andi $s1, $s2, 1
	
	jr $ra
ISEVEN:
	jal ISODD
	seq $t9, $s1, 0
	bne $t9, 1, increment
	addi $t8, $t8, 1
	sw $s2, 0($t3)
	add $t3, $t3, $t6
	j increment
end:
	li $v0, 4
	la $a0, newline
	syscall
	la $a0, sizeB # Size of B
	syscall
	li $v0, 1
	move $a0, $t8 # sizeof(ISEVEN(B[j]))
	syscall
