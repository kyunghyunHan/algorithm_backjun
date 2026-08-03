#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// 파라미터로 주어지는 문자열은 const로 주어집니다. 변경하려면 문자열을 복사해서 사용하세요.
char* solution(const char* n_str) {
    int index =0;
    // return 값은 malloc 등 동적 할당을 사용해주세요. 할당 길이는 상황에 맞게 변경해주세요.
    char* answer = malloc(strlen(n_str) + 1);
    bool s = false;
    for  (int i = 0; i < strlen(n_str);i++){
        if (n_str[i]!='0' || s==true){
            answer[index++] = n_str[i];
            s= true;
        }
    }
    answer[index] = '\0';
    return answer;
}