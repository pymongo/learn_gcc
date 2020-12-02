#.PHONY: dynamic_linking_library, statically_linking_library
gcc_headers_search_path:
	echo | gcc -E -Wp,-v -
gcc_linker_library_search_path:
	ld --verbose | grep SEARCH_DIR | tr -s ' ;'
.ONESHELL:
clean:
	cd target
	rm -rf *.s *.c *.h *.cpp *.hpp
	rm -rf *.o *.so *.a
	rm -rf executable
.ONESHELL:
# only support in x86_64-linux
statically_linked_executable: clean
	cp statically_linked_executable.s target/hello.s
	cd target
	nasm -o hello.o -f elf64 hello.s
	ld -o hello hello.o
	file hello
	./hello
.ONESHELL:
simple_compile_two_source_files: clean
	cp *.c target && cd target
	gcc -o executable main.c add.c
	./executable
# 动态链接库用`-L . -ladd`除非main.c引入了add.h头文件而且so文件在ld_search_dir内
.ONESHELL:
dynamic_linking_library: clean
	cp *.c target && cd target
	# omit gcc -c -fPIC add.c
	gcc -shared -o libadd.so add.c
	gcc -o executable main.c ./libadd.so
	./executable
# 由于没有add.h头文件，所以不能`-L . -ladd`
.ONESHELL:
statically_linking_library: clean
	cp *.c target/ && cd target/
	gcc -c add.c
	ar crus libadd.a add.o
	gcc -o executable main.c libadd.a
	./executable
