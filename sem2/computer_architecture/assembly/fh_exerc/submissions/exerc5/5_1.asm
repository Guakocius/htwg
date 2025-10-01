.data
value: .word 0x12345678 # Beispielwert

.text
.globl main

main:
	li $t0, 2404 # Byte-Adresse
	srl $t1, $t0, 2 # 2404 >> 2 = 601
	sll $t2, $t1, 2 # 601 << 2 = 2404
	la $t3, value # Lade Adresse
	lw $t4, 0($t3) # Lade Word aus Adresse
	
	