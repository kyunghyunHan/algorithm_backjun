#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int order) {
    int answer = 0;
    char str[20];
    sprintf(str, "%d", order);
    for (int i = 0; i<strlen(str);i++){
        if (str[i]=='3' || str[i]=='6' || str[i]=='9'){
            answer+=1;
        }
    }
    return answer;
}