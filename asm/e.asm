push 1   ;; n
set 0
push 1.0 ;; sum
set 1
push 1   ;; n!
set 2

get 0    <- decide
push 20  ;; where to end
<
pop_jump_if calc
pop_jump_if_not end

pop      <- calc
get 2
copy -1
*
set 2    ;; set n! back
++
set 0    ;; set n back
push 1.0
get 2
/
get 1
+
set 1    ;; set sum back
jump decide

pop      <- end
pop      ;;clean up
get 1
HALT



