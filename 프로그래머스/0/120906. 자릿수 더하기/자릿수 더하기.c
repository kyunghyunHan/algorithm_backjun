#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>


int solution(int n) {
    int answer = 0;
    char s[20]; 
    sprintf(s, "%d", n);
    
    for (int i = 0; i<strlen(s);i++){
        answer += n % 10; 
        n /= 10;   
    }
    
    return answer;
}