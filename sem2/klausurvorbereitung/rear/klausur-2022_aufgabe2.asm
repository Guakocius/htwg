

.globl main
main:
	add $t0, $zero, $zero
	sb $t0, 0($sp)
	
	addi $sp, $sp, 1
	addi $t0, $zero, 8
	sb $t0, 0($sp)
	
	addi $sp, $sp, 1
	addi $t0, $zero, 255
	sb $t0, 0($sp)
	
	addi $sp, $sp, 1
	addi $t0, $zero, 255
	sb $t0, 0($sp)
	
	addi $sp, $sp, 1
	addi $t0, $zero, 16
	sb $t0, 0($sp)
 
	lw $s1, -4($sp)
	