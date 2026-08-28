#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int a, int b, bool flag) {
    int answer = 0;
    answer = (flag==true)?a+b:a-b;
    return answer;
}