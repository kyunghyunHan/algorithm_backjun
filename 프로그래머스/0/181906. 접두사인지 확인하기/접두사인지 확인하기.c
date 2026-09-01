#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// 파라미터로 주어지는 문자열은 const로 주어집니다. 변경하려면 문자열을 복사해서 사용하세요.
int solution(const char* my_string, const char* is_prefix) {
    
    int answer = 0;
    int len = strlen(my_string);
    int cnt = 0;
    for (int i = 0; i<len;i++){
        if (cnt == strlen(is_prefix) ){
            answer=1;
            break;
        }
        if (my_string[i]==is_prefix[i]){
            cnt+=1;
        }else{
            cnt = 0;
        }

    }
    return answer;
}