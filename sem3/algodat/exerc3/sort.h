#ifndef SORT_H
#define SORT_H

typedef struct Arr {
    int n;
    int *arr;
}Arr;

int s_sort(Arr*);
int i_sort(Arr*);
int q_sort(Arr*, int, int);
int m_sort(Arr*, Arr*, int, int);
void swp(Arr*, int, int);
int get(int);
int part(Arr*, int, int);
void mrg(Arr*, Arr*, int, int, int);

#endif
