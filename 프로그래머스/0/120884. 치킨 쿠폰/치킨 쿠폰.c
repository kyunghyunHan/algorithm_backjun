#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int chicken) {
    int answer = 0;
    int cnt = 0;
    for (int i = 0;i<chicken;i++){
        cnt+=1;
        if (cnt==10){
            cnt= 0;
            chicken+=1;
            answer+=1;
        }
    }
    return answer;
}