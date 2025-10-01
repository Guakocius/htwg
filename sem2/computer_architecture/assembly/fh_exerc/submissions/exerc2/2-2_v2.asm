.data
newline : .asciiz"\n"
isO : .asciiz"isodd:\n"
isE : .asciiz"iseven:\n"
.text
main:
	addi $s1, $zero, -10
	blt $s1, $zero, isNegative
	isNegative:
		sub $s1, $zero, $s1
	
	ISODD:
		andi $s2, $s1, 1
		li $v0, 4
		la $a0, isO
		syscall

		li $v0, 1
		add $a0, $zero, $s2
		syscall
		
		li $v0, 4
		la $a0, newline
		syscall
		j ISEVEN
	
	
	ISEVEN:
		andi $s2, $s1, 1
		li $v0, 4
		la $a0, isE
		syscall

		li $v0, 1
		sub $a0, $v0, $s2
		syscall
		
		li $v0, 4
		la $a0, newline
		syscall
		j end

end: