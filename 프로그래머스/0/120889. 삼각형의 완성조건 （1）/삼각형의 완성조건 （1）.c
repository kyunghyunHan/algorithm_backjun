#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int compare(const void *a, const void *b)
{
    int x = *(const int *)a;
    int y = *(const int *)b;

    return x - y;
}
int solution(int sides[], size_t sides_len) {
    int answer = 1;
    qsort(sides, sides_len, sizeof(sides[0]), compare); 
    
    if (sides[0]+sides[1]<=sides[2]){
        answer = 2;
    }
    return answer;
}

