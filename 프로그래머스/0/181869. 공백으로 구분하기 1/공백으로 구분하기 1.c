#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char** solution(const char* my_string) {
    char **answer = malloc(sizeof(char *) * 1001);

    int idx = 0;
    int n = 0;

    while (my_string[idx] != '\0') {

        while (my_string[idx] == ' ') {
            idx++;
        }

        if (my_string[idx] == '\0') {
            break;
        }

        char *s = malloc(sizeof(char) * 1001);
        int i = 0;

        while (my_string[idx] != ' ' &&
               my_string[idx] != '\0') {
            s[i++] = my_string[idx++];
        }

        s[i] = '\0';

        answer[n++] = s;
    }

    return answer;
}