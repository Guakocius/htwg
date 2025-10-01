#ifndef FILEINFO_H
#define FILEINFO_H

#include <stdlib.h>
#include <limits.h>

enum filetype {
    filetype_regular,
    filetype_directory,
    filetype_other
};

struct fileinfo {
    struct fileinfo *next;
    char filename[NAME_MAX + 1];
    enum filetype type;

    union {
        size_t size;
        struct fileinfo *subdir;

    };
};

typedef struct fileinfo fileinfo;

void fileinfo_print(const fileinfo *f);

void fileinfo_destroy(fileinfo *f);
struct fileinfo *fileinfo_create(const char* filename);



#endif // FILEINFO_H