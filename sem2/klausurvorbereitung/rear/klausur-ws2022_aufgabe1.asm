

g:
	# b := $a0, a ;= $a1
	addi $sp, $sp, -32
	sw $ra, 28($sp)
	sb $s0, 0($sp) # A
	sb $s1, 24($sp) # j
	
	addiu $t0, $zero, 19
	addiu $t1, $a0, 20
	sw $t1, $t0($s0)
	addiu $s1, $zero, 18
loop:
	beq $s1, $zero, end
	addiu $t1, $s1, 1
	lb $t2, $t1($s0)
	addiu $t2, $t2, $a1
	sb $t2, $s1($s0)
	addiu $s1, $s1, -1
	j loop
end:
	lb $s1, 24($sp)
	lb $s0, 0($sp)
	lb $v0, 0($s0)
	lw $ra, 28($sp)
	jr $ra
	
	
	