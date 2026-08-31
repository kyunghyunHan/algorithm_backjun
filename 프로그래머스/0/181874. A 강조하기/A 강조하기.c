#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

char* solution(const char* myString) {
    int len = strlen(myString);
    char* answer = malloc(len + 1);

    int i;

    for (i = 0; i < len; i++) {
        if (myString[i] == 'a') {
            answer[i] = 'A';
        }
        else if (myString[i] > 'A' && myString[i] <= 'Z') {
            answer[i] = myString[i] + 32;
        }
        else {
            answer[i] = myString[i];
        }
    }

    answer[i] = '\0';

    return answer;
}