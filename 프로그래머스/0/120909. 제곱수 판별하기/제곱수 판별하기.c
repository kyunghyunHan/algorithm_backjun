#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <math.h>
int solution(int n) {
    int answer = 0;
    int root = (int)sqrt(n);
    if (root* root==n){
        answer=1;
    }else{
        answer =2;
    }
    return answer;
}