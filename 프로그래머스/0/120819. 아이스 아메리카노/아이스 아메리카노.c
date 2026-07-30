#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int* solution(int money) {
    // return 값은 malloc 등 동적 할당을 사용해주세요. 할당 길이는 상황에 맞게 변경해주세요.
    int* answer = malloc(sizeof(int) * 2);
    answer[0] = money / 5500;   // 살 수 있는 커피 개수
    answer[1] = money % 5500;   // 남은 돈
    return answer;
}