push 10 ;; n
set 0
push 0  ;; initial
push 1  ;; initial

get 0 <- loop
push 1
>
pop_jmp_if caculate
pop_jmp_if_not 16
pop <- caculate
--
set 0
copy -1
copy -1
+
jmp loop
pop
pop
HALT
