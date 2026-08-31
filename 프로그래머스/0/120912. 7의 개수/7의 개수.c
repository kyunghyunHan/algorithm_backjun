#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int array[], size_t array_len) {
    int answer = 0;

    for (int i = 0; i < array_len; i++) {
        int n = array[i];

        while (n > 0) {
            if (n % 10 == 7) {
                answer++;
            }

            n /= 10;
        }
    }

    return answer;
}