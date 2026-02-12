#include <stdio.h>

int  main(){
    int n ;
    int res;
    scanf("%d",&n);
    if (n>12){
        res = n+1;
    }else{
        res= n;
    }
    printf("%d",res);
    return 0;
}