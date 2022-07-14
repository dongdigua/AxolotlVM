push 1 ;; acc
set 0
push 1 ;; compare
push 6 ;; n
==
pop_jmp_if_not 7
pop_jmp_if 14
dup
get 0
swap
*
set 0
--
jmp 4
pop
pop
get 0
HALT
