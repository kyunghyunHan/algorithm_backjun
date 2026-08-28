#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

char** solution(const char* strArr[], size_t strArr_len) {

    char** answer = malloc(sizeof(char*) * strArr_len);

    for (int i = 0; i < strArr_len; i++) {

        int len = strlen(strArr[i]);

        answer[i] = malloc(sizeof(char) * (len + 1));

        for (int j = 0; j < len; j++) {

            // 짝수 번째 문자열 → 소문자로
            if (i % 2 == 0) {

                if (strArr[i][j] >= 'A' && strArr[i][j] <= 'Z') {
                    answer[i][j] = strArr[i][j] + 32;
                } else {
                    answer[i][j] = strArr[i][j];
                }

            // 홀수 번째 문자열 → 대문자로
            } else {

                if (strArr[i][j] >= 'a' && strArr[i][j] <= 'z') {
                    answer[i][j] = strArr[i][j] - 32;
                } else {
                    answer[i][j] = strArr[i][j];
                }
            }
        }

        answer[i][len] = '\0';
    }

    return answer;
}