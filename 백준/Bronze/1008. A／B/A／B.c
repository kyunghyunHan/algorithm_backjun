#include <stdio.h>

int main() {
    int A, B;
    scanf("%d %d", &A, &B);

    double result = (double)A / B;
    printf("%.10f\n", result);

    return 0;
}