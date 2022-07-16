ASSEMBLER=cargo run -- asm
# ASM=$(basename *.asm)

all: e fib lable_fib factorial function_call

%: %.asm
	$(ASSEMBLER) $?

clean:
	rm *.abin
