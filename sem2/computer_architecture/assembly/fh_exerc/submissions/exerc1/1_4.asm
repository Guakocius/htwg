addi $s3, $s3, 10
addi $s0, $s0, 0
addi $s1, $s1, 1

Loop:
	
	beqz $s3, end
	add $s2, $s0, $s1
	add $s0, $s1, $zero
	add $s1, $s2, $zero
	subi $s3, $s3, 1
	j Loop
end:
