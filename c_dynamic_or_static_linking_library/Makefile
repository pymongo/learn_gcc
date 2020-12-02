.ONESHELL:
clean:
	cd .. && make clean

.ONESHELL:
sll: clean
	cp *.c *.h ../target && cd ../target
	gcc -c add.c sub.c
	# archive/zip/pack multi object files to single static linking library .a file
	ar crus libarithmetic.a add.o sub.o
	readelf -h libarithmetic.a

	# `-I .`: add path . to gcc header_files search path
	gcc -o main.o -c main.c -I .
	# or: gcc -o main.o -c main.c -include arithmetic.h

	# `-L .` add path . to linker(ld) lib search path
	gcc -o executable main.o -L . -larithmetic
	# or: gcc -o executable main.o libarithmetic.a

	./executable

.ONESHELL:
dll: clean
	cp *.c *.h ../target && cd ../target
	gcc -Wall -fPIC -c add.c sub.c
	gcc -shared -o libarithmetic.so add.o sub.o

	# read symbol list
	readelf -Ws libarithmetic.so
	nm -D libarithmetic.so # or nm libarithmetic.so
	objdump -TC libarithmetic.so

	gcc -o main.o -I . -c main.c
	# gcc static and dynatic linking both use -l arg

	# method_1. 将so文件放到默认的ld search_dir例如/usr/lib内`ld --verbose | grep SEARCH_DIR | tr -s ' ;'`
	# method_2. 运行时添加`LD_LIBRARY_PATH=.`的环境变量
	# method_2: gcc -o executable main.o -larithmetic
	# method_2: LD_LIBRARY_PATH=. ./executable
	# method_3. add ./ prefix to libarithmetic.so let executable find it from relative path
	gcc -o executable main.o libarithmetic.so
	./executable
	
	ldd executable
