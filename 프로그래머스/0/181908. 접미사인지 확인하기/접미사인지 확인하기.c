#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int solution(const char* my_string, const char* is_suffix) {
    int len1 = strlen(my_string);
    int len2 = strlen(is_suffix);

    if (len2 > len1)
        return 0;
     
    if (strcmp(my_string + len1 - len2, is_suffix) == 0)
        return 1;

    return 0;
}