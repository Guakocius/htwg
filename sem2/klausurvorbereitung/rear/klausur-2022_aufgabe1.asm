

f:
	addi $sp, $sp, -16
	sw $ra, 16($sp)
	sw $a0, 12($sp) # A
	sw $a1, 8($sp) # a
	sw $a2, 4($sp) # i
	sw $a3, 0($sp) # j
	
	sw $s0, -4($sp) # m
	
	move $t0, $a0
	move $a0, $a1
	move $a1, $a3
	
	addi $s0, $a0, 4	
	jal H
	move $a0, $a2
	move $a1, $v0
	
	jal G
	sllv $t1, $s0, 2
	addu $t1, $s0, $t1
	addu $t1, $v0, $t1
	lw $v0, 0($t1)
	
	lw $s0, -4($sp)
	lw $a3, 0($sp)
	lw $a2, 4($sp)
	lw $a1, 8($sp)
	lw $a0, 12($sp)
	lw $ra, 16($sp)
	
	addi $sp, $sp, 16
	jr $ra
	
	
	
	
	
