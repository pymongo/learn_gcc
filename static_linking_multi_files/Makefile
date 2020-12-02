# 虽然gcc一行命令能解决简单的多个.c文件的编译问题，但是正统的做法还是要像这样逐个将源文件编译成.o object文件再去link
.PHONY: clean # 如果当前目录恰好有一个文件叫clean，则make发现clearn文件已存在则不会重新构建。因此需要将clean目标声明为伪目标(phony target)
# ONESHELL允许make(开新的shell里)执行cd命令
.ONESHELL:
clean:
	cd .. && make clean
.ONESHELL:
main_with_add_h: clean
	cp *.c *.h ../target/ && cd ../target/
	gcc -c add.c # `-c`: compile to an object file, omit `-I .` beacause our lib(add) is has no header to include
	# `-I .`: add path . to gcc header files search path
	gcc -o main.o -I . -c main_with_add_h.c # or -include add.h
	gcc -o executable main.o add.o # alternative: use ld linker rather than gcc compiler
	./executable
.ONESHELL:
# only support in x86_64-linux, only asm can compile pure static linking executable file
statically_linked_executable: clean
	cp statically_linked_executable.s ../target/hello.s && cd ../target
	nasm -o hello.o -f elf64 hello.s
	ld -o hello hello.o
	file hello
	./hello
