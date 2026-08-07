#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
int compare(const void *a, const void *b);
// numbers_len은 배열 numbers의 길이입니다.
int solution(int numbers[], size_t numbers_len) {
    int answer = 0;
    int len =numbers_len ;

    qsort(numbers, numbers_len, sizeof(int), compare);
    answer = numbers[len-1] * numbers[len-2];
    return answer;
}

int compare(const void *a, const void *b) {
    return (*(int *)a - *(int *)b); // 오름차순
}