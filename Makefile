#.PHONY: dynamic_linking_library, statically_linking_library
gcc_headers_search_path:
	echo | gcc -E -Wp,-v -
gcc_linker_library_search_path:
	ld --verbose | grep SEARCH_DIR | tr -s ' ;'
clean:
	# cargo clean
	rm -rf target/
	mkdir target/
statically_linked_executable: clean
	# only support in x86_64-linux
	nasm -o target/statically_linked_executable.o -f elf64 statically_linked_executable.s
	ld -o target/statically_linked_executable target/statically_linked_executable.o
	file target/statically_linked_executable
	./target/statically_linked_executable
simple_compile_two_source_files: clean
	gcc -o target/executable main.c add.c
	./target/executable
# 动态链接库用`-L . -ladd`除非main.c引入了add.h头文件而且so文件在ld_search_dir内
dynamic_linking_library: clean
	gcc -shared -o target/libadd.so add.c
	gcc -o target/executable main.c target/libadd.so
	./target/executable
# 由于没有add.h头文件，所以不能`-L . -ladd`
statically_linking_library: clean
	gcc -o target/add.o -c add.c
	ar rs target/libadd.a target/add.o
	gcc -o target/executable main.c target/libadd.a
	./target/executable
