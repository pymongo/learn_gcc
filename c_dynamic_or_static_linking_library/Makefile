clean:
	rm -rf ../target
	mkdir ../target
sll: clean
	gcc -o ../target/add.o -c add.c
	gcc -o ../target/sub.o -c sub.c
	# archive/zip/pack multi object files to single static linking library .a file
	ar rs ../target/libarithmetic.a ../target/add.o ../target/sub.o
	readelf -h ../target/libarithmetic.a
	# `-I .`: add path . to gcc header_files search path
	gcc -o ../target/main.o -I . -c main.c
	# `-L ../target` add path . to linker(ld) lib search path
	gcc -o ../target/executable ../target/main.o -L ../target -larithmetic
	# or: gcc -o ../target/executable ../target/main.o ../target/libarithmetic.a
	../target/executable
dll: clean
	gcc -o ../target/add.o -Wall -fPIC -c add.c
	gcc -o ../target/sub.o -Wall -fPIC -c sub.c
	gcc -shared -o ../target/libarithmetic.so ../target/add.o ../target/sub.o

	# read symbol list
	readelf -Ws ../target/libarithmetic.so
	nm -D ../target/libarithmetic.so # or nm libarithmetic.so
	objdump -TC ../target/libarithmetic.so

	gcc -o ../target/main.o -I . -c main.c
	# gcc static and dynatic linking both use -l arg

	# TODO: 以下注释中多种编译DLL的方法均假设源文件和中间产物都放在同一个文件夹内
	# method_1. 将so文件放到默认的ld search_dir例如/usr/lib内`ld --verbose | grep SEARCH_DIR | tr -s ' ;'`
	# method_2. 运行时添加`LD_LIBRARY_PATH=.`的环境变量
	# method_2: gcc -o executable main.o -larithmetic
	# method_2: LD_LIBRARY_PATH=. ./executable
	# method_3. add ./ prefix to libarithmetic.so let executable find it from relative path
	gcc -o ../target/executable ../target/main.o ../target/libarithmetic.so
	../target/executable
	
	ldd ../target/executable
