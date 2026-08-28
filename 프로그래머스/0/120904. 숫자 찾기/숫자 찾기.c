#include <stdio.h>

int solution(int num, int k) {
    int answer = -1;
    int index = 0;
    int div = 1;

    while (num / div >= 10) {
        div *= 10;
    }

    while (div > 0) {
        index++;

        if ((num / div) % 10 == k) {
            answer = index;
            break;
        }

        div /= 10;
    }

    return answer;
}