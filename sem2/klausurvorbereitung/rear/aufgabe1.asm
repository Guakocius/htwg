# Aufgabe 1
# A: s0, j: s1, m: s2, b: a2, i: a3, I: s3
f: # int f
	### Prolog
	addi $sp, $sp, -16
	sw $ra, 12($sp)
	sw $s0, 8($sp) # A
	sw $s1, 4($sp) # j
	sw $s2, 0($sp) # m
	###
	move $a0, $s0 # move => addu $a0, $s0, $zero
	move $a1, $s1
	move $t2, $s2
	addi $t2, $a2, 8 # Annahme: b ist in a2
	move $s2, $t2
	
	# call t(I,j); v0 => return value
	move $a0, $s3
	move $a1, $s1
	jal G
	
	addu, $s2, $s2, $v0
	srav 
	
	# call r(j,i)
	move $a0, $s1
	move $a1, $a3
	jal H
	
	addu $s2, $s2, $v0
	
	# return A[m]
	sll $t0, $s2, 2 # Index => Byte-Offset = m*4
	addu $t0, $s0, $t0 # &A[m]
	lw $v0, 0($t0)
	
	### Epilog
	lw $s2, 0($sp)
	lw $s1, 4($sp)
	lw $s0, 8($sp)
	lw $ra, 12($sp)
	addi $sp, $sp, 16
	jr $ra
	###
