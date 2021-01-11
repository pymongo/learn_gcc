#include <stdio.h>

const int three = 3;

void print_from_dynamic_linking_library() {
    printf("call %s() from %s\n", __FUNCTION__, __FILE__);
}
