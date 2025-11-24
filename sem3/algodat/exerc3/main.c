#include <stdio.h>
#include <stdlib.h>

#include "./sort.h"

int main(void) {
    int n = 1000;
    int *arr = malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        int r = rand();
        *(arr + i) = r;
        printf("Array idx %d: %d\n", i, *(arr + i));
    }
    Arr a = { n, arr };
    Arr aux = { n, .arr=calloc(n, sizeof(int)) };
}
