#include <stdio.h>
#include <stdbool.h>

long long min(long long a, long long b) {
    return (a < b) ? a : b;
}

long long max(long long a, long long b) {
    return (a > b) ? a : b;
}
long long solution(int a, int b) {
   long long answer = 0;
    long long x = min(a, b);  // a와 b 중 최솟값
    long long y = max(a, b);  // a와 b 중 최댓값
    for (long long i = x; i <= y; i++) {  // x부터 y까지 반복
        answer += i;  // i 값을 answer에 더함
    }
    return answer;
}
