# 1.3

sub $s2, $s0, $s1
slt $t1, $s2, $zero
bne $t1, $zero, abs
j end
abs:
    sub $s2, $zero, $s2
end:
