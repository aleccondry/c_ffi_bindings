#include "print.h"
#include <stdio.h>

void print_message(const char* message) {
    printf("%s", message);
    fflush(stdout);
}

