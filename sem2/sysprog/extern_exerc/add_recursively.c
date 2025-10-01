#include <stdio.h>
#include <stdlib.h>

#define n 6

int main() {
    int sum = 0;
    int* A = (int*)malloc(n * sizeof(int));

    for (int i = 0; i < n; i++) {
        A[i] = (i + 1) * n;
        sum += A[i];

        printf("Element %d: %d\n", i, A[i]);
    }
    printf("Sum of array elements: %d\n", sum);
}