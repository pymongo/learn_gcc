build:
	gcc -c add.c
	gcc -c sub.c
	# archive/zip/pack multi object files to single static linking library .a file
	ar rs libarithmetic.a add.o sub.o
	readelf -h libarithmetic.a
	# `-I .`: add path . to gcc header_files/include search path
	gcc -I . -c main.c
	# `-L .` add path . to linker(ld) search path
	gcc -o executable main.o -L . -larithmetic # or: gcc -o executable main.o libarithmetic.a
run: build
	./executable
clean:
	rm *.o
	rm *.a
	rm executable 2> /dev/null
