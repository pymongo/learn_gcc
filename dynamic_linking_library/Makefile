run:
	gcc -Wall -fPIC -c add.c
	gcc -Wall -fPIC -c sub.c
	gcc -shared -o libarithmetic.so add.o sub.o

	# read symbol list
	readelf -Ws libarithmetic.so
	nm -D libarithmetic.so # or nm libarithmetic.so
	objdump -TC libarithmetic.so

	gcc -I . -c main.c
	# gcc static and dynatic linking both use -l arg
	 # or: gcc -o executable main.o libarithmetic.so

	# method_1. 将so文件放到默认的ld search_dir例如/usr/lib内`ld --verbose | grep SEARCH_DIR | tr -s ' ;'`
	# method_2. 运行时添加`LD_LIBRARY_PATH=.`的环境变量
	# method_2: gcc -o executable main.o -larithmetic
	# method_2: LD_LIBRARY_PATH=. ./executable
	# method_3. add ./ prefix to libarithmetic.so let executable find it from relative path
	gcc -o executable main.o ./libarithmetic.so
	./executable
	
	ldd executable
clean:
	rm *.o
	rm *.so
	rm executable 2> /dev/null
