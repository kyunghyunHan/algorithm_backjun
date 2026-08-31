#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

// 파라미터로 주어지는 문자열은 const로 주어집니다. 변경하려면 문자열을 복사해서 사용하세요.
int solution(const char* myString, const char* pat) {
    int answer = 0;
    char * s  = malloc(sizeof(char)* strlen(myString)+1);
    int i;
    for ( i = 0; i<strlen(myString);i++){
        if (myString[i]=='A'){
           s[i]=  myString[i]+1;
        }else if(myString[i]=='B'){
            s[i] =myString[i]-1;
       }
    }
    s[i]= '\0';
    printf("%s",s);
    if (strstr(s,pat)){
        answer = 1;
    }
    free(s);
    
    return answer;
}