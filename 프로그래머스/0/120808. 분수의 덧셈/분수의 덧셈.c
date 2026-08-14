#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int* solution(int numer1, int denom1, int numer2, int denom2) {

    int numerator = numer1 * denom2 + numer2 * denom1;
    int denominator = denom1 * denom2;

    int gcd = 1;

    for (int i = 1; i <= numerator && i <= denominator; i++) {
        if (numerator % i == 0 && denominator % i == 0) {
            gcd = i;
        }
    }

    numerator /= gcd;
    denominator /= gcd;

    int* answer = (int*)malloc(sizeof(int) * 2);

    answer[0] = numerator;
    answer[1] = denominator;

    return answer;
}