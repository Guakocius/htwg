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
    Arr b = { n, arr };
    Arr c = { n, arr };
    Arr d = { n, arr };

    //int cnts = s_sort(&a);
    //int cnti = i_sort(&b);
    int cntq = q_sort(&c, 0, n - 1);
    int cntm = m_sort(&aux, &d, 0, n - 1);
    
   // printf("cnt selection sort: %d\n", cnts);
   // printf("cnt insertion sort: %d\n", cnti);
    printf("cnt quick sort: %d\n", cntq);
    printf("cnt merge sort: %d\n", cntm);
    

    

    

    
}
