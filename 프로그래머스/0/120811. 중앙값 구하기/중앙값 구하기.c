#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int compare(const int *a,const int *b){
    return *(int*) a - *(int*)b;
}
// array_len은 배열 array의 길이입니다.
int solution(int array[], size_t array_len) {
    int answer = 0;
    int len = array_len;

    qsort(array,len,sizeof(int),compare);
    
    answer = array[len/2];
    return answer;
}