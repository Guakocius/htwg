#include <stdio.h>
#include <stdlib.h>
void *_calloc(unsigned long n, unsigned long size) {
  int *mem = malloc(n * size);
  for (int i = 0; i < n; i++) {
    mem[i] = 0;
  }
  return mem;
}

int main(void) {
  int *array = _calloc(5,sizeof(int));

  printf("Sizeof void: %zu\n", sizeof(void));
  int a = 5;
  int b = 5;
  int *aptr = &a;
  int *bptr = &b;
  printf("a == b? %d\n", aptr == bptr);
 
  for (int i = 0; i < 5; i++) {
     printf("Array mit calloc: %d\n", array[i]);
    *(array + i) = i;
    printf("Array[i]: %d\n", *(array + i));
  }
  return 0; 
}
