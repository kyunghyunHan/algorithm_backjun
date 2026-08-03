#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// num_list_len은 배열 num_list의 길이입니다.
int solution(int num_list[], size_t num_list_len) {
    int answer = 0;
    int x = 0;
    int y= 1;
    for (int i = 0; i < num_list_len ; i++ ){
         x += num_list[i];
         y *= num_list[i];
    }
    if (y<pow(x,2)){
        answer = 1;
    }
    return answer;
}