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
simple_compile_two_source_files: clean
	cp static_linking_multi_files/*.* target/ && cd target/
	gcc -o executable main.c add.c
	./executable
# 动态链接库用`-L . -ladd`除非main.c引入了add.h头文件而且so文件在ld_search_dir内
.ONESHELL:
dll: clean
	cp static_linking_multi_files/*.* target/ && cd target/
	# omit gcc -c -fPIC add.c
	gcc -shared -o libadd.so add.c
	gcc -o executable main.c ./libadd.so
	./executable
# 由于没有add.h头文件，所以不能`-L . -ladd`
.ONESHELL:
sll: clean
	cp static_linking_multi_files/*.* target/ && cd target/
	gcc -c add.c
	ar rs libadd.a add.o
	gcc -o executable main.c libadd.a
	./executable
.ONESHELL:
rustc_link_dll_sll: clean
	cp rust_call_c_dll_sll/src/*.* target/ && cd target/
	gcc -shared -o libdll.so dll.c
	gcc -c sll.c && ar rs libsll.a sll.o
	rustc -o executable main.rs -L native=. -l dylib=dll -l static=sll
	LD_LIBRARY_PATH=. ./executable
