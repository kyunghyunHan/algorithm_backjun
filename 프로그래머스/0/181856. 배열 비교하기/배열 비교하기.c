#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

int sum(int arr[], size_t arr_len) {
    int sum = 0;

    for (int i = 0; i < arr_len; i++) {
        sum += arr[i];
    }

    return sum;
}

int solution(int arr1[], size_t arr1_len,
             int arr2[], size_t arr2_len) {

    if (arr1_len > arr2_len) {
        return 1;
    }
    else if (arr1_len < arr2_len) {
        return -1;
    }

    int sum1 = sum(arr1, arr1_len);
    int sum2 = sum(arr2, arr2_len);

    if (sum1 > sum2) {
        return 1;
    }
    else if (sum1 < sum2) {
        return -1;
    }

    return 0;
}