#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// 파라미터로 주어지는 문자열은 const로 주어집니다. 변경하려면 문자열을 복사해서 사용하세요.
int solution(const char* before, const char* after) {
    int answer = 1;
    int len= strlen(before);
    int *arr = calloc(26, sizeof(int));
    int *brr = calloc(26, sizeof(int));
    int a = 0;
    int b = 0;
    for (int i = 0; i<len;i++){
        arr[after[i] - 'a']+=1;
        brr[before[i] - 'a']+=1;

    }
      for (int i = 0; i < 26; i++) {
        if (arr[i] != brr[i]) {
            free(arr);
            free(brr);
            answer = 0;
            break;
        }
    }
    return answer;
}