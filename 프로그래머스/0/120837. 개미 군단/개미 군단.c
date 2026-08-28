#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int hp) {
    int answer = 0;
    int sp = hp;
    while(1){
        if (sp<1){
            break;
         }
        if (sp>=5){
            sp-=5;
            answer++;
        }else if(sp>=3){
            sp-=3;
            answer++;
        }else{
            sp-=1;
            answer++;
        }
    }
    return answer;
}