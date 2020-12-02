.ONESHELL:
dll:
	cargo build
	cp main.c ../target/debug/ && cd ../target/debug/
	gcc -o executable main.c ./libarithmetic.so
	./executable
	ldd executable
.ONESHELL:
sll:
	cargo build
	cp main.c ../target/debug/ && cd ../target/debug/
	gcc -o executable main.c libarithmetic.a
	./executable
	ldd executable
