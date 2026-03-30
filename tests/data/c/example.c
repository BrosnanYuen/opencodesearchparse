#include <stdio.h>

int global_var = 42;

struct Point {
    int x;
    int y;
};

int add(int a, int b) {
    return a + b;
}

void print_hello() {
    printf("Hello, World!\n");
}

#define MAX 100