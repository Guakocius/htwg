
f:
	addi $sp, $sp, -16
	sw $ra, 12($sp)
	sw $s0, 8($sp) # A
	sw $s1, 4($sp) # m
	sw $s2 0($sp) # b
	
	addi $s1, $s2, 8
	move $a0, $s3 # I
	jal G
	addu $s1, $v0, $s1
	
	move $a0, $a1 # j
	move $a1, $a3 # i
	jal H
	addu $s1, $v0, $s1
	
	sllv $t0, $s1, 2
	addu $t0, $s0, $t0
	
	move $v0, $t0
	move $a0, $s0
	move $a1, $s1
	
	lw $s2 0($sp)
	lw $s1, 4($sp)
	lw $s0, 8($sp)
	lw $ra, 12($sp)
	
	addi $sp, $sp, 16
	jr $ra