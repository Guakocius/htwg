#include <stdio.h>
#include <stdalign.h>
#include <stdbool.h>

int a = 10;
short int b = 20;
long int c = 30L;
long long int d = 40LL;
unsigned int e = 50U;
unsigned short int f = 60U;
unsigned long int g = 70UL;
unsigned long long int h = 80ULL;
float i = 90.0F;
double j = 100.0;
long double k = 110.0L;
char l = 'A';
signed char m = -1;
unsigned char n = 0xff;
_Bool o = 1;
bool p = true;
const char* s = "Hallo";

int main(void) {
    printf("%-15s %-20s %-15s %-25s %-10s %-10s\n", "Address:", "Space requirement:", "Alignment:", "Type:", "Name:", "Value:");
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10d\n", (void*)&a, sizeof(a), alignof(int), "int", "a", a);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10hd\n", (void*)&b, sizeof(b), alignof(short int), "short int", "b", b);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10ld\n", (void*)&c, sizeof(c), alignof(long int), "long int", "c", c);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10lld\n", (void*)&d, sizeof(d), alignof(long long int), "long long int", "d", d);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10u\n", (void*)&e, sizeof(e), alignof(unsigned int), "unsigned int", "e", e);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10hu\n", (void*)&f, sizeof(f), alignof(unsigned short int), "unsigned short int", "f", f);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10lu\n", (void*)&g, sizeof(g), alignof(unsigned long int), "unsigned long int", "g", g);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10llu\n", (void*)&h, sizeof(h), alignof(unsigned long long int), "unsigned long long int", "h", h);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10f\n", (void*)&i, sizeof(i), alignof(float), "float", "i", i);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10f\n", (void*)&j, sizeof(j), alignof(double), "double", "j", j);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10Lf\n", (void*)&k, sizeof(k), alignof(long double), "long double", "k", k);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10c\n", (void*)&l, sizeof(l), alignof(char), "char", "l", l);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10hhd\n", (void*)&m, sizeof(m), alignof(signed char), "signed char", "m", m);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10hhu\n", (void*)&n, sizeof(n), alignof(unsigned char), "unsigned char", "n", n);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10d\n", (void*)&o, sizeof(o), alignof(_Bool), "_Bool", "o", o);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10d\n", (void*)&p, sizeof(p), alignof(bool), "bool", "p", p);
    printf("%-15p %-20zu %-15zu %-25s %-10s %-10s\n", (void*)&s, sizeof(s), alignof(char*), "char*", "s", s);


    if (sizeof(int) == 4 && sizeof(long int) == 4 && sizeof(char*) == 4) {
        printf("ILP32 detected\n");
    } else if (sizeof(int) == 4 && sizeof(long int) == 8 && sizeof(char*) == 8) {
        printf("LP64 detected\n");
    } else if (sizeof(int) == 4 && sizeof(long int) == 4 && sizeof(long long int) == 8 && sizeof(char*) == 8) {
        printf("LLP64 detected\n");
    }
}

