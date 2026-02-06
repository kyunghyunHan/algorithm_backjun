#include <stdio.h>

int main(){
    
    int a,b,c;
    
    scanf("%d",&a);
    scanf("%d",&b);
    scanf("%d",&c);
    
    if (a <= b * c){
        printf("%s","yes");
    }else{
        printf("%s","no");

    }

    return 0;
}