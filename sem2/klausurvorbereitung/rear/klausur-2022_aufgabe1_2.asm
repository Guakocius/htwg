

f:
	addi $sp, $sp, -12
	sw $ra, 8($sp)
	sw $s0, 4($sp) # A
	sw $s1, 0($sp) # m
	### a: $a1, i: $a2, j: $a3
	addi $s1, $a1, 4
	
	move $a0, $a1
	move $a1, $a3
	jal H
	
	move $a0, $a2
	move $a1, $v0
	jal G
	
	sllv $t0, $s1, 2
	addu $s0, $s0, $v0
	addu $t0, $t0, $s0
	
	
	lw $s1, 0($sp)
	lw $s0, 4($sp)
	lw $ra, 8($sp)
	addi $sp, $sp, 12
	
	move $v0, $t0
	jr $ra
	
	