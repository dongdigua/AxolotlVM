;; this an example of function call
;; pass in (a=1, b=2, c=3)
;; then calculate b (a c +) /

;; 1
;; 2
;; 3

push 3
push 2
push 1
jmp main
set 0              <- function
copy -2
+
swap
/
swap
pop
get 0
ret
call function      <- main
HALT
