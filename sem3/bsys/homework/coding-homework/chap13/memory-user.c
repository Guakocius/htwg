#include <stddef.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

#define KiB 1024
#define MiB KiB * 1024

int main(int argc, char* argv[]) {
  printf("Please type in the number of Mebibyte you want to use\n");
  int m;
  scanf("%d", &m);
  getchar(); // clears newline character
  size_t b = (size_t)m * (MiB);
  char* arr = malloc(b);
  if (arr == NULL) {
    printf("ERROR: failed allocating memory!\n");
    return -1;
  }
  printf("Using %d Mebibyte.\n", m);
  printf("Using %zu Byte.\n", b);
  sleep(2);
  printf("This program's PID: %d\n", getpid());
  printf("Press a button to continue\n");
  getchar();
  
  while (1) {
    for (int i = 0; i < b; i += (4 * KiB)) { // 4 KiB, pagesize
      arr[i] += 1; // touch $i
      printf("Arr[i]: %d\ti: %d\n", arr[i], i);
    }

  }
  return 0;
}
