push 10 ;; n
set 0
push 0  ;; initial
push 1  ;; initial
get 0
push 1
>
pop_jmp_if 9
pop_jmp_if_not 16
pop
--
set 0
copy -1
copy -1
+
jmp 4
pop
pop
HALT
