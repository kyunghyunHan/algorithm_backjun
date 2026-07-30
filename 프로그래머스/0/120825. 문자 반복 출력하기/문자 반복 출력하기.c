#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* solution(const char* my_string, int n) {
    int len = strlen(my_string);

    char* answer = malloc(len * n + 1);

    int idx = 0;

    for (int i = 0; i < len; i++) {
        for (int j = 0; j < n; j++) {
            answer[idx++] = my_string[i];
        }
    }

    answer[idx] = '\0';

    return answer;
}