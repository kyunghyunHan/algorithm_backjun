#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
int solution(int n) {
    int fact = 1;
    int answer = 1;

    for (int i = 1; ; i++) {
        fact *= i;

        if (fact > n) {
            break;
        }

        answer = i;
    }

    return answer;
}