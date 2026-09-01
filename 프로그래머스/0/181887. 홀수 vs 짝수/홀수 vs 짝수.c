#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// num_list_len은 배열 num_list의 길이입니다.
int solution(int num_list[], size_t num_list_len) {
    int answer = 0;
    int odd =0;
    int even = 0;
    for (int i = 0; i<num_list_len;i++){
        if (i%2!=0){
            odd+=num_list[i];
        }else{
            even+=num_list[i];
        }
    }
    answer= (odd>even)?odd:even;
    return answer;
}