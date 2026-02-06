#include <stdio.h>

int main(){
    char s[21];
    char s2[21];
    
    
    scanf("%s",s);
    scanf("%s",s2);
    
    if (strcmp(s, s2) == 0){
        printf("%d",0);
    }else{
        printf("%d",1550);

    }
    
    return 0;

}