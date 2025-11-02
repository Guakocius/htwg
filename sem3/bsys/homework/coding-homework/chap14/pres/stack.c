#include <stdlib.h>
#include <stdio.h>

#define STACK_MAX 10

int idx = 0;

void push(int n, int *ptr) {
  if (idx == STACK_MAX - 1) printf("Stack full");
  while (ptr[idx] != 0) {
    idx++;
  }
  ptr[idx] = n;
}
void pop(int *arr) {
  arr[idx] = 0;
  idx--;
}

int main(void) {
  int *stack = malloc(STACK_MAX * sizeof(int));
  for (int i = 0; i < 3; i++) {
    push(i, stack);
    printf("Stack with push: %d\n", *stack);
  }
  pop(stack);
  for (int i = 0; i < idx; i++) {
    printf("Stack with pop: %d\n", *stack);
  }
}
