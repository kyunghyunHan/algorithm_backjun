#include <stdio.h>

int main() {
    int A, B, C, D;
    scanf("%d %d %d %d", &A, &B, &C, &D);

    int shuttle_time = A + B;
    int walk_time = C;

    int shuttle_ok = (shuttle_time <= D);
    int walk_ok = (walk_time <= D);

    if (shuttle_ok && walk_ok) {
        printf("~.~\n");
    } 
    else if (!shuttle_ok && !walk_ok) {
        printf("T.T\n");
    } 
    else if (shuttle_ok) {
        printf("Shuttle\n");
    } 
    else {
        printf("Walk\n");
    }

    return 0;
}
