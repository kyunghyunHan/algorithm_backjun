#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
int compare(const void *a,const void *b){
    return *(const int *)a - *(const int *)b;
}
// num_list_len은 배열 num_list의 길이입니다.
int* solution(int num_list[], size_t num_list_len) {
    // return 값은 malloc 등 동적 할당을 사용해주세요. 할당 길이는 상황에 맞게 변경해주세요.
    int* answer = (int*)malloc(sizeof(int)*30);
    qsort(num_list,num_list_len,sizeof(int),compare);
    int index = 0;
    for (int i = 5; i<num_list_len;i++){
        answer[index++] = num_list[i];
    }
    return answer;
}