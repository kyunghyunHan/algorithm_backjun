#include <stdio.h>
#define LEN_INPUT1 11
#define LEN_INPUT2 11

int main(void) {
    char s1[LEN_INPUT1];
    char s2[LEN_INPUT2];
    scanf("%s %s", s1, s2);
    for (int i = 0;i<strlen(s1);i++){
        printf("%c",s1[i]);
    }
    for (int i = 0;i<strlen(s2);i++){
        printf("%c",s2[i]);
    }
    return 0;
}