#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

char* solution(const char* my_string, int m, int c) {
    size_t len = strlen(my_string);

    int count = len / m;
    char* answer = malloc(count + 1);

    int index = 0;

    for (int i = c - 1; i < len; i += m) {
        answer[index++] = my_string[i];
    }

    answer[index] = '\0';

    return answer;
}