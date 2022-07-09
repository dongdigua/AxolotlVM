push 1
set 0
push 1
push 6
==
pop_jump_if_not 7
pop_jump_if 14
copy
get 0
swap
*
set 0
--
jump 4
pop
pop
get 0
HALT
