#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <string.h>

/**
 * Sortiert alle Zahlen bis n in a nach dem Bubblesort-Algorithmus.
 * @param a
 * @param n
 * @param m
 * @param cmp
 */
void bubblesort(void* v, size_t n, size_t m, int (*cmp)(const void*, const void*) ) {
    char* a = (char*) v; // Pointer a in char* umwandeln
    void* tmp = malloc(m * sizeof(char));

    if (tmp == NULL) {
        fprintf(stderr, "Error: tmp wurde kein Speicher zugewiesen");
        exit(1);
    }

    for (size_t i = n; i > 1; --i) {
        // größten Wert nach hinten schieben
        for (size_t j = 0; j < i - 1; ++j) {
            void* str1 = a + (j * m);
            void* str2 = a + (j + 1) * m;
            if (cmp(str1, str2) > 0) {
                memcpy(tmp, str2, m);
                memcpy(str2, str1, m);
                memcpy(str1, tmp, m);
            }
        }
    }
    free(tmp);
} // bubblesort

/**
 * main ist der Startpunkt des Programms.
 * @param argc
 * @param argv
 * @return (argc != 2 || n < 1 || a == NULL) ? 1 : 0
 */
int main(int argc, const char *argv[]) {
    //--------------------------------------------- Arraygroesse bestimmen
    if (argc != 2) {
        fprintf(stderr, "Aufruf: ./stringsort Anzahl\n");
        return 1;
    }

    int n = atoi(argv[1]);

    if (n < 1) {
        fprintf(stderr, "Anzahl muss mindestens 1 sein\n");
        return 1;
    }

    //--------------------------------------------------- Strings wuerfeln
    int m = strlen(argv[1]) + 1;
    char* a = malloc(n * m); // Pointer a genügend Platz zuweisen

    if (a == NULL) {
        fprintf(stderr, "Error: a wurde kein Speicher zugewiesen");
        return 1;
    }

    srand((unsigned int) time(NULL));



    printf("Unsortiertes Array:\n");
    for (int i = 0; i < n; ++i) {

        int r = rand() % n;

        sprintf(&a[i * m], "%d", r);
        printf("%s ", &a[i * m]);
    }
    printf("\n");

    //-------------------------------------------------- Strings sortieren
    bubblesort(a, n, m, (int (*) (const void*, const void*))strcmp);

    //--------------------------------------------------- Strings ausgeben
    printf("Sortiertes Array:\n");
    char* s = malloc(n * m);

    if (s == NULL) {
        fprintf(stderr, "Error: s wurde kein Speicher zugewiesen");
        free(a);
        return 1;
    }

    s[0] = '\0';
    strcat(s, &a[0]);

    // Ersetze mehrfach vorkommende Strings mit *
    for (int i = 1; i < n; ++i) {
        if (strcmp(&a[i * m], &a[(i - 1) * m]) == 0) {
            strcat(s, "*");
        } else {
            strcat(s, " ");
            strcat(s, &a[i * m]);
        }
    }

    // Konkatinierter String s (StringBuilder-Äquivalent) ausgeben.
    printf("%s\n", s);
    // Restliche Garbage Collection
    free(a);
    free(s);

    return 0;
} // main
