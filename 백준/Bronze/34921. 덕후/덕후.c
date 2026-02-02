#include <stdio.h>

int main(){
    int a,t;
    int res;
    
    scanf("%d %d",&a,&t);
    res = 10 + 2 * ( 25-a+t);
    if (res < 0){
           printf("%d",0);

    }else{
            printf("%d",res);

    }  
    return 0;
}