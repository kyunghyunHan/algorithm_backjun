#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int compare(const void *a, const void *b) {
    const char *s1 = *(const char **)a;
    const char *s2 = *(const char **)b;

    return strcmp(s1, s2);
}

char** solution(const char* myString) {
    size_t len = strlen(myString);

    char** answer = malloc(sizeof(char*) * (len + 1));

    int answer_index = 0;
    int start = 0;

    for (int i = 0; i <= len; i++) {

        if (myString[i] == 'x' || myString[i] == '\0') {

            int word_len = i - start;

            if (word_len > 0) {
                answer[answer_index] = malloc(word_len + 1);

                for (int j = 0; j < word_len; j++) {
                    answer[answer_index][j] = myString[start + j];
                }

                answer[answer_index][word_len] = '\0';

                answer_index++;
            }

            start = i + 1;
        }
    }

    qsort(answer, answer_index, sizeof(char*), compare);

    return answer;
}