#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

char* solution(const char* my_string, int s, int e) {
    size_t len = strlen(my_string);

    char* answer = malloc(len + 1);
    int index = 0;

    // s 이전
    for (int i = 0; i < s; i++) {
        answer[index++] = my_string[i];
    }

    // s ~ e 뒤집기
    for (int i = e; i >= s; i--) {
        answer[index++] = my_string[i];
    }

    // e 이후
    for (int i = e + 1; i < len; i++) {
        answer[index++] = my_string[i];
    }

    answer[index] = '\0';

    return answer;
}