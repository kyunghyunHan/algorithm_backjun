#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

// arr_len은 배열 arr의 길이입니다.
// queries_rows는 2차원 배열 queries의 행 길이, queries_cols는 2차원 배열 queries의 열 길이입니다.
int* solution(int arr[], size_t arr_len,
              int** queries,
              size_t queries_rows,
              size_t queries_cols)
{
    for (size_t i = 0; i < queries_rows; i++) {
        int s = queries[i][0];
        int e = queries[i][1];

        for (int j = s; j <= e; j++) {
            arr[j]++;
        }
    }

    return arr;
}