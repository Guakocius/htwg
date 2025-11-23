#include "./sort.h"

int s_sort(Arr *arr) {
    for (int i = 0; i < arr->n - 1; i++) {
        int min = i;

        for (int j = i + 1; j < arr->n; j++) {
            if (*(arr->arr + j) < *(arr->arr + min)) min = j;
        }
        swp(arr, i, min);
    }
}

int part(Arr* arr, int left, int right) {
    int i = left - 1, j = right;
    int piv = *(arr->arr + right);

    while (1) {
        while (*(arr->arr + (++i)) < piv);
        while (*(arr->arr + (--j)) > piv) if (j == left) break;
        if (i >= j) break;
        swp(arr, i, j);
    }
    swp(arr, i, right);
    return i;
}

int i_sort(Arr *arr) {
    int idx = 0;
    for (int i = 1; arr->n; i++) {
        int x = *(arr->arr + i);
        int j = i - 1;

        while (j >= 0 && x < *(arr->arr + j)) {
            idx++;
            j--;
        } 

        for (int k = i; k >= j + 2; k--) *(arr->arr + k) = *(arr->arr + (k - 1));
        *(arr->arr + (j + 1)) = x;
    }
}

int q_sort(Arr *arr, int left, int right) {
    if (right <= left) return;
    int i = part(arr, left, right);
    q_sort(arr, left, i - 1);
    q_sort(arr, i + 1, right);
}

void mrg(Arr* aux, Arr* arr, int l, int m, int r) {
    int i, j, k;
    for (i = m + 1; i > l; i--) *(aux->arr + (i - 1)) = *(arr->arr + (i - 1));
    for (j = m; j < r; j++) *(aux->arr + (r + m - j)) = *(arr->arr + (i - 1));
}

int m_sort(Arr* aux, Arr *arr, int left, int right) {
    if (right <= left) return;
    int m = (left + right) >> 1;
    m_sort(aux, arr, m + 1, right);
    mrg(aux, arr, left, m, right);
}

void swp(Arr *arr, int i, int j) {
    int t = *(arr->arr + i);
    *(arr->arr + i) = *(arr->arr + j);
    *(arr->arr + j) = t;
}
