#include <stdio.h>

void print_from_static_linking_library() {
    printf("call %s() from %s\n", __FUNCTION__, __FILE__);
}
