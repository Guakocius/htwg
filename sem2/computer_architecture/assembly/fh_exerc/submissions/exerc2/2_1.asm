.data
.align 2
n: .word 6
A: .word 3 4 1 1 1 1

.text
.globl main

main:
	li $t6, 0 # Sum
	lw $t0, n # n
	la $t1, A # Array
	
	loop:	# n * t3 + n

		bge $t2, $t0, end # Checks whether the index is at the end of the Array
		addi $t2, $t2, 1
		lw $t4, 0($t1)
		add $t6, $t6, $t4 # Sum
		addi $t1, $t1, 4 # Moves pointer of the address by 4 bytes

	 	j loop 
	
	end:
	
	li $v0, 1
	move $a0, $t6
	syscall 
