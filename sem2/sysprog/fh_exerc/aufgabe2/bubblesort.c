#include <stdio.h>
#include <stdlib.h>
#include <time.h>

/* Bubblesort liest ganze Zahlen ein und sortiert sie aufsteigend.*/
int main(int argc, const char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "./bubblesort Anzahl\n");
        return 1;
    }
    srand((unsigned int) time(NULL));
    int n = atoi(argv[1]);
    int* a = malloc(n * sizeof(int));

    if (a == NULL) {
        fprintf(stderr, "Error: a wurde kein Speicher zugewiesen");
    }
    int scanner;
    int len = n;

    //---------------------------------------------------- Zahlen einlesen
    printf("Bitte %d ganze Zahlen eingeben: ", n);
    int k = 0;
    for (int i = 0; i < len && (scanf("%d", &scanner) > 0); ++i) {
        a[i] = scanner;
        ++k;
    }
    for (int i = k; i < len; ++i) {
        a[i] = rand();
        printf("%d\n", a[i]);
    }

    //--------------------------------------------------- Zahlen sortieren
    for (int i = len; i > 1; --i) {
        // groessten Wert nach hinten schieben
        for (int j = 0; j < i - 1; ++j) {
            if (a[j] > a[j + 1]) {
                // Werte tauschen
                int tmp = a [j + 1];
                a[j + 1] = a[j];
                a[j] = tmp;
            }
        }
    }

    //---------------------------------------------------- Zahlen ausgeben
    printf("Sortierte Zahlenfolge: \n");
    for (int m = 0; m < len; ++m) {
        printf("%d\n", a[m]);
    }
    free(a);
    return 0;
}
