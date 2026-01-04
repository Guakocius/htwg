#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <pthread.h>


static volatile int counter = 0;

void *mythread(void *arg) {
  printf("%s: begin\n", (char*)arg);
  int i;
  for (i = 0; i < 1e7; i++) {
    counter = counter + 1;
  }
  printf("%s: done\n", (char*)arg);
  return NULL;
}

int main(int argc, char *argv[]) {
  pthread_t p1, p2;
  pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
  int rc = pthread_mutex_init(&lock, NULL);
  assert(rc == 0);

  printf("main: begin (counter = %d)\n", counter);
  if (pthread_create(&p1, NULL, mythread, "A") != 0) {
    printf("P1 creation encountered an error!\n");
    exit(-1);
  }
  if (pthread_create(&p2, NULL, mythread, "B") != 0) {
    printf("P2 creation encountered an error!\n");
    exit(-1);
  }
  // join waits for the threads to finish
  if (pthread_join(p1, NULL) != 0) {
    printf("P1 join encountered an error!\n");
    exit(-1);
  }
  if (pthread_join(p2, NULL) != 0) {
    printf("P2 join encountered an error!\n");
    exit(-1);
  }
  printf("main: done with both (counter = %d)\n", counter);
  return 0;
}
