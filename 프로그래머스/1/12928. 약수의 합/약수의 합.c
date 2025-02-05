#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int solution(int n)
{
    int i;
    int count = 0;
    for (i = 1; i < n +1; ++i)
    {
       if (n % i ==0){
        count+=i;
       }
    }
    return count;
}