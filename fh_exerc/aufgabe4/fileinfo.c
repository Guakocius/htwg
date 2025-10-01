#define _POSIX_C_SOURCE 200112L

#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#include "fileinfo.h"

static fileinfo *list_directory(char *filename) {
    DIR *dir = opendir(filename);
    struct stat f_stat;
    lstat(filename, &f_stat);

    chdir(filename);
    fileinfo *info = malloc(sizeof(fileinfo));
    fileinfo *head = NULL, *tail = NULL;

    if (!chdir(filename)) {
        free(info);
        return NULL;
    }

    if (!info) {
        fprintf(stderr, "%s couldn't be read! (%d)\n", filename, errno);
        return info = NULL;
    }
    memset(info, 0, sizeof(fileinfo));

    strcpy(info->filename, filename);

    const struct dirent *d;

    if (!dir) {
        fprintf(stderr, "%s:\t%d", strerror(errno), errno);
    }
    errno = 0;
    while ((d = readdir(dir)) != NULL) {

        if (strcmp(d->d_name, ".") == 0 || strcmp(d->d_name, "..") == 0) {
            // info = info->next;
            continue;
        }
        fileinfo *info_next = fileinfo_create(d->d_name);

        if (!info_next)
            continue;

        if (!head) {
            head = tail = info_next;
        } else {
            tail->next = info_next;
            tail = info_next;
        }
    }
    if (errno != 0) {
        fprintf(stderr, "%d : Error : %s\n", errno, strerror(errno));
        closedir(dir);
        return info = NULL;
    }
    chdir("..");
    closedir(dir);
    fileinfo_destroy(info);
    return head;
}

static void print_regular(const char *filename, size_t size) {

    fprintf(stdout, "%s (%s, %zu %s)\n", filename, "regular", size, "Byte");
}
static void print_other(const char *filename) {
    printf("%s (other)\n", filename);
}

static void print_directory(const char *path_relative, const char *filename,
                            const fileinfo *sub_dirs) {
    char path[PATH_MAX];
    printf("%s%s:\n", path_relative, filename);

    /*if (!path_relative) {
        fprintf(stderr, "%s\t%d\n", "Error: no allocated memory!", errno);
        return;
    }*/

    // snprintf(path_relative, PATH_MAX, "%s", filename);
    // printf("%s:\n", path_relative);

    /*if (strcmp(path_relative, "") == 0) {
        printf("%s:\n", filename);
        snprintf(path_relative, PATH_MAX, "%s", filename);
    }*/

    for (const fileinfo *i = sub_dirs; i; i = i->next) {

        switch (i->type) {
        case filetype_regular:
            print_regular(i->filename, i->size);
            break;
        case filetype_directory:
            printf("%s (directory)\n", i->filename);
            break;
        case filetype_other:
            print_other(i->filename);
            break;
        }

        while (i->next) {

            switch (i->next->type) {
            case filetype_regular:
                print_regular(i->next->filename, i->next->size);
                break;
            case filetype_directory:
                printf("%s (directory)\n", i->next->filename);
                break;
            case filetype_other:
                print_other(i->next->filename);
                break;
            }
            i = i->next;
        }
        printf("\n");
    }
    for (const fileinfo *i = sub_dirs; i; i = i->next) {
        if (i->type == filetype_directory) {
            snprintf(path, PATH_MAX, "%s%s/", path_relative, filename);
            print_directory(path, i->filename, i->subdir);
        }
    }
}

void fileinfo_print(const fileinfo *info) {
    errno = 0;

    if (info->type == filetype_regular) {
        size_t size = info->size;
        print_regular(info->filename, size);
    } else if (info->type == filetype_directory) {
        /*char *path_relative =
            malloc(sizeof(fileinfo) * (strlen(info->filename) + 1));
        if (!path_relative) {
            fprintf(stderr,
                    "Error: No memory allocated for the path!\t%d\t%s\n", errno,
                    strerror(errno));
            return;
        }*/
        // strcpy(path_relative, "");
        printf("\n");
        print_directory("", info->filename, info->subdir);
        // free(path_relative);

    } else if (info->type == filetype_other) {
        print_other(info->filename);
    } else
        fprintf(stderr, "Error: Printing failed!\t%d\t%s", errno,
                strerror(errno));
}

void fileinfo_destroy(fileinfo *info) {

    if (!info)
        return;

    if (info->type == filetype_directory && info->subdir) {
        fileinfo *subdir = info->subdir;
        while (subdir) {
            fileinfo *next_subdir = subdir->next;
            fileinfo_destroy(subdir);
            subdir = next_subdir;
        }
    }
    free(info);
}

fileinfo *fileinfo_create(const char *filename) {

    fileinfo *info = malloc(sizeof(fileinfo));
    struct stat f_stat;

    if (!info) {
        fprintf(stderr, "No allocation done, %d %s", errno, strerror(errno));
        return NULL;
    }

    memset(info, 0, sizeof(fileinfo));

    strcpy(info->filename, filename);

    if ((lstat(info->filename, &f_stat) == -1)) {
        fileinfo_destroy(info);
        return NULL;
    } else if (strlen(filename) > NAME_MAX) {
        errno = ENAMETOOLONG;
        fileinfo_destroy(info);
        return NULL;
    } else if (strlen(filename) == 0) {
        fileinfo_destroy(info);
        fprintf(stderr, "Error: No filename!\t%d\t%s", errno, strerror(errno));
        return NULL;
    } else {
        if (S_ISREG(f_stat.st_mode)) {
            info->type = filetype_regular;
            info->size = f_stat.st_size;
        } else if (S_ISDIR(f_stat.st_mode)) {
            info->type = filetype_directory;
            info->subdir = list_directory(info->filename);
        } else {
            info->type = filetype_other;
        }
    }
    return info;
}