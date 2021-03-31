# target: linux_x86_64, syntax: AT&T
# gcc -c statically_linked_executable.s && ld statically_linked_executable.o && ./a.out
# or
# as statically_linked_executable.s && ld statically_linked_executable.o && ./a.out # GNU assembler (GNU Binutils)
# or use gcc -nostdlib compile c code(main->_start) get statically linked binary
# echo "int _start(){return 0;}" > main.c && gcc -nostdlib main.c && ldd ./a.out

    .global _start
    .text
_start:
    # write(1, message, 13)
    mov     $1, %rax                # system call 1 is write
    mov     $1, %rdi                # file_descriptor 1 is stdout
    mov     $message, %rsi          # address of string to output
    mov     $13, %rdx               # len of string
    syscall                         # invoke operating system to do the write
    # exit(0)
    mov     $60, %rax               # system call 60 is exit
    xor     %rdi, %rdi              # we want return code 0
    syscall                         # invoke operating system to exit
message:
    .ascii  "Hello, world\n"
