#include <setjmp.h>
#include <stdio.h>

static jmp_buf buf;

void risky_c_function(int should_fail) {
    if (should_fail) {
        printf("C: Error occurred, jumping back!\n");
        longjmp(buf, 1);
    }
    printf("C: Success!\n");
}

int call_with_jump(int should_fail) {
    if (setjmp(buf) == 0) {
        risky_c_function(should_fail);
        return 0; // success
    } else {
        return -1; // error jumped
    }
}
