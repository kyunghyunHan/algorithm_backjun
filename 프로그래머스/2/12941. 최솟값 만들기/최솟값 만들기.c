#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h> 
int compare(const int * a, const int *b){
    return *(const int *)a-*(const int *)b;
}
int compare2(const int * a, const int *b){
    return *(const int *)b-*(const int *)a;
}
// A_len은 배열 A의 길이입니다.
// B_len은 배열 B의 길이입니다.
int solution(int A[], size_t A_len, int B[], size_t B_len) {
    int answer = 0;
    qsort(A,A_len,sizeof(int),compare);
    qsort(B,B_len,sizeof(int),compare2);
    
    for (int i = 0;i<A_len;i++){
        answer+=A[i]*B[i];
    }
    return answer;
}