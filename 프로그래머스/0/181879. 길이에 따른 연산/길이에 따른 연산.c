#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// num_list_len은 배열 num_list의 길이입니다.
int solution(int num_list[], size_t num_list_len) {
    int answer = 0;
    
    if (num_list_len>=11){
        int sum = 0;
        for (int i =0; i<num_list_len;i++){
            sum+=num_list[i];
        }
        answer = sum;
    }else{
        int mul = 1;
        for (int i = 0; i<num_list_len;i++){
            mul *=num_list[i];
            answer =mul;
        }
    }
    return answer;
}