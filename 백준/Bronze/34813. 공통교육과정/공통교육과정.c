#include <stdio.h>

int main(){
    char s[100];
    scanf("%s", s);
    
    char first = s[0];
    if (first == 'F') {
        printf("%s","Foundation");
    }else if (first == 'C'){
        printf("%s","Claves");
    }else if (first == 'V'){
        printf("%s","Veritas");
    }else if (first == 'E'){
        printf("%s","Exploration");
    }
    
    return 0;

}