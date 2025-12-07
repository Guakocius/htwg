
#include <stdio.h>
int main(void) {
  int num = 10;
  int *ptr = &num;
  printf("Pointer to num: %d with address: %p\nReference to pointer: %p\n", *ptr, ptr, &ptr);
  return 0;
}
