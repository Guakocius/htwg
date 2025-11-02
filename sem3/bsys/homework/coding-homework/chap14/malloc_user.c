#include <stdio.h>
#include <stdlib.h>
int main(void) {
  char *str = malloc(4 * sizeof(char*));
  int *unique = malloc(0);
  printf("Unique value with malloc(0): %p\n", unique);
  return 0;
}
