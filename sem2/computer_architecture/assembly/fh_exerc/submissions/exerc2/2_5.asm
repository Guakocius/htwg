.data

.text
.globl main

main:
	addi $s0, $zero, 3 # n
	addi $s1, $zero, 10 # k
	addi $s2, $zero, 7 # if condition
	
	jal f
	
	li $v0, 10
	syscall
	
f:
	# if (k-n > 7)
	addi $sp, $sp, -4
	sw $ra, 0($sp)
	sub $t0, $s1, $s0
	slt $at, $s2, $t0
	beq $at, $zero, else
	
	# return n + k + 5
	add $s0, $s0, $s1
	addi $s0, $s0, 5
	li $v0, 1
	move $a0, $s0
	syscall
	j return
	

else:
	
	addi $s0, $s0, -1 # n-1
	addi $t1, $t1, 8
	j max
	
	
max:
	# max(8, g(k))
	move $s3, $t1 # 8
	jal G
	add $a1, $a1, $s1 # 200 + k
	slt $at, $s3, $a1
	bne $at, $zero, g_k_larger
	move $s1, $s3
	lw $ra, 0($sp)
	addi $sp, $sp, 4
	jal f
	
g_k_larger:
	move $s1, $a1
	lw $ra, 0($sp)
	jal f
	
return:
	lw $ra, 0($sp)
	addi $sp, $sp, 4
	jr $ra
	

# willkürliche Prozedur G (überschreibt alle "ungesicherten" Register mit dem Wert $a0+200, liefert auch $a0+200 zurück)
G:	addi $a0,$a0,100
	addi $a1,$a0,100
	addi $a2,$a0,100
	addi $a3,$a0,100
	addi $t0,$a0,100
	addi $t1,$a0,100
	addi $t2,$a0,100
	addi $t3,$a0,100
	addi $t4,$a0,100
	addi $t5,$a0,100
	addi $t6,$a0,100
	addi $t7,$a0,100
	addi $t8,$a0,100
	addi $t9,$a0,100
	addi $v0,$a0,100
	addi $v1,$a0,100
	jr $ra
