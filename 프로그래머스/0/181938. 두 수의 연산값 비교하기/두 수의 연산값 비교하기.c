#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int a, int b) {
    int temp = b;
    int mul = 10;

    while (temp >= 10) {
        mul *= 10;
        temp /= 10;
    }

    int ab = a * mul + b;

    if (ab < 2 * a * b) {
        return 2 * a * b;
    } else {
        return ab;
    }
}