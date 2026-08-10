#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// 파라미터로 주어지는 문자열은 const로 주어집니다. 변경하려면 문자열을 복사해서 사용하세요.
int solution(const char* my_string) {
    int result = 0;
    char *answer = malloc(strlen(my_string) + 1);
    strcpy(answer, my_string);
    int len = strlen(answer);
    for (int i = 0; i<len;i++){
        if (answer[i] >= '0' && answer[i] <= '9') {
             result+=answer[i]- '0';
        }
    }

    return result;
}