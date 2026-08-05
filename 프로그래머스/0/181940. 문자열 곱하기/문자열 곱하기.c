#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
// 파라미터로 주어지는 문자열은 const로 주어집니다. 변경하려면 문자열을 복사해서 사용하세요.

char* solution(const char* my_string, int k) {
    int len = strlen(my_string);

    // 문자열 길이 × k + 널 문자 1칸
    char* answer = malloc(sizeof(char) * (len * k + 1));

    int index = 0;

    // 문자열을 k번 반복
    for (int i = 0; i < k; i++) {
        // my_string의 문자들을 처음부터 끝까지 복사
        for (int j = 0; j < len; j++) {
            answer[index] = my_string[j];
            index++;
        }
    }

    // 문자열 끝 표시
    answer[index] = '\0';

    return answer;
}