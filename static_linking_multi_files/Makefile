# 虽然gcc一行命令能解决简单的多个.c文件的编译问题，但是正统的做法还是要像这样逐个将源文件编译成.o object文件再去link
.PHONY: clean # 如果当前目录恰好有一个文件叫clean，则make发现clearn文件已存在则不会重新构建。因此需要将clean目标声明为伪目标(phony target)
clean:
	rm *.o
	rm executable 2> /dev/null
build:
	gcc -c add.c # `-c`: compile to an object file, omit `-I .` beacause our lib is has no header to include
	gcc -I . -c main.c # `-I .`: add path . to gcc header files search path
	gcc -o executable add.o main.o # alternative: use ld linker rather than gcc compiler
run: build
	./executable
