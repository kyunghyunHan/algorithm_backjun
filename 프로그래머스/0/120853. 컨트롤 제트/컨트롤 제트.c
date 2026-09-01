#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int solution(const char* s) {
    int answer = 0;
    int prev = 0;

    char *str = malloc(strlen(s) + 1);
    strcpy(str, s);

    char *p = strtok(str, " ");

    while (p != NULL) {

        if (strcmp(p, "Z") == 0) {
            answer -= prev;
        } else {
            prev = atoi(p);
            answer += prev;
        }

        p = strtok(NULL, " ");
    }

    free(str);

    return answer;
}