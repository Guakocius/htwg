
#include <stdio.h>
#include <stdlib.h>
int main(void) {
  char *data = malloc(100);
  int i = 3;
  data[1000] = 5;
  printf("dat: %d\n", data[1000]);
  return 0;
}
