#ifndef VECTOR_H
#define VECTOR_H

#include <stdlib.h>
typedef struct {
  size_t nums, membuf, max;
  int *dat;
}vector;

void insert(vector* vptr, int num, int pos);


#endif // VECTOR_H
