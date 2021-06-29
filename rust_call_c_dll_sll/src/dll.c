#include <stdio.h>

int const_int_one = 1; // shared static
static int static_int_two = 2; // private static

void print_from_dynamic_linking_library() {
    printf("call %s() from %s\n", __FUNCTION__, __FILE__);
}
