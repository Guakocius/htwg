#include "vector.h"
#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>

void insert(vector* vptr, int num, int pos) {
  while (vptr->dat[pos] != 0) {
    pos += 1;
    insert(vptr, num, pos);
  }
    
  vptr->dat[pos] = num; 
  vptr->nums += 1;
}

int main(void) {
  size_t nums = 0;
  size_t membuf = nums*sizeof(int);
  size_t max = 20;
  int* dat = malloc(max*sizeof(int));
  vector v = { .nums = nums, .membuf = membuf, .max = max, .dat = dat};
  vector *vptr = &v;
  insert(vptr, 5, 0);
  insert(vptr, 10, 0);
  printf("Nums: %zu\nDat0: %d\nDat1: %d", vptr->nums, vptr->dat[0], vptr->dat[1]);

  
}
