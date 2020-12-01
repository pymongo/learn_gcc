build:
	gcc -Wall -fPIC -c add.c
	gcc -Wall -fPIC -c sub.c
	gcc -shared -o libarithmetic.so add.o sub.o

	# read symbol list
	readelf -Ws libarithmetic.so
	nm -D libarithmetic.so # or nm libarithmetic.so
	objdump -TC libarithmetic.so

	gcc -I . -c main.c
	# gcc static and dynatic linking both use -l arg
	gcc -o executable main.o -larithmetic # or: gcc -o executable main.o libarithmetic.so
	ldd executable

	# 1. 将so文件放到默认的ld search_dir例如/usr/lib内`ld --verbose | grep SEARCH_DIR | tr -s ' ;'`
	# 2. 运行时添加`LD_LIBRARY_PATH=.`的环境变量
	LD_LIBRARY_PATH=. ./executable
run: build
	./executable
clean:
	rm *.o
	rm *.so
	rm executable 2> /dev/null
