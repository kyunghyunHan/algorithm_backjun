#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int solution(const char* myString, const char* pat) {
    int answer = 0;

    int mlen = strlen(myString);
    int plen = strlen(pat);

    for (int i = 0; i <= mlen - plen; i++) {
        int cnt = 0;

        for (int j = 0; j < plen; j++) {
            if (myString[i + j] == pat[j]) {
                cnt++;
            } else {
                break;
            }
        }

        if (cnt == plen) {
            answer++;
        }
    }

    return answer;
}