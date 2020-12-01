build:
	gcc -c add.c
	gcc -c sub.c
	ar rs liblib.a add.o sub.o
	gcc -I . -c main.c
	# `-L .` add . to linker(ld) search path
	gcc -o executable -L . main.o -llib # alternative: gcc -o executable main.o liblib.a
run: build
	./executable