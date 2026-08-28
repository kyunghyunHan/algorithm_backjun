#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

int solution(const char* myString, const char* pat) {
    int my_len = strlen(myString);
    int pat_len = strlen(pat);

    for (int i = 0; i <= my_len - pat_len; i++) {
        int t = 1;

        for (int j = 0; j < pat_len; j++) {

            if (tolower((unsigned char)myString[i + j]) !=
                tolower((unsigned char)pat[j])) {
                t = 0;
                break;
            }
        }

        if (t == 1) {
            return 1;
        }
    }

    return 0;
}