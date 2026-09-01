#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int n) {
  int  piece = 6;

    while(true) {
        if (piece % n == 0) {
            break;
        }
        piece += 6;
    }

    return piece / 6;
}