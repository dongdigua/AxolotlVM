ASSEMBLER=cargo run -- asm
# ASM=$(basename *.asm)

all: e fib lable_fib factorial function_and_list string

%: %.asm
	$(ASSEMBLER) $?

clean:
	rm *.abin
