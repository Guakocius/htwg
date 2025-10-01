#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include <string.h>

/**
 * Sortiert alle Zahlen bis n in a nach dem Bubblesort-Algorithmus.
 * @param a
 * @param n
 */
void bubblesort(char** a, int n) {
    for (long unsigned i = n; i > 1; --i) {
        // größten Wert nach hinten schieben
        for (long unsigned j = 0; j < i - 1; ++j) {
            if (strcmp(a[j], a[j + 1]) > 0) {
                char* tmp = a[j + 1];
                a[j + 1] = a[j];
                a[j] = tmp;
            }
        }
    }
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
    char** a = malloc(n * sizeof(char*)); // Pointer Array a genügend Platz zuweisen

    if (a == NULL) {
        fprintf(stderr, "Error: a wurde kein Speicher zugewiesen");
        return 1;
    }

    srand((unsigned int) time(NULL));

    int m = strlen(argv[1]) + 1;

    printf("Unsortiertes Array:\n");
    for (int i = 0; i < n; ++i) {

        int r = rand() % n;
        a[i] = malloc(m);

        sprintf(a[i], "%d", r);
        printf("%s ", a[i]);
    }
    printf("\n");

    //-------------------------------------------------- Strings sortieren
    bubblesort(a, n);

    //--------------------------------------------------- Strings ausgeben
    printf("Sortiertes Array:\n");
    char* s = malloc(n * m);

    if (s == NULL) {
        fprintf(stderr, "Error: s wurde kein Speicher zugewiesen");
        free(a);
        return 1;
    }
    s[0] = '\0';
    strcat(s, a[0]);

    // Ersetze mehrfach vorkommende Strings mit *
    for (int i = 1; i < n; ++i) {
        if (strcmp(a[i], a[i - 1]) == 0) {
            strcat(s, "*");
        } else {
            strcat(s, " ");
            strcat(s, a[i]);
        }
    }

    // MemAddr Garbage Collection für alle a[i]
    for (int i = 0; i < n; ++i) {
        free(a[i]);
    }

    // Konkatinierter String s (StringBuilder-Äquivalent) ausgeben.
    printf("%s\n", s);
    // Restliche Garbage Collection
    free(a);
    free(s);

    return 0;
} // main
