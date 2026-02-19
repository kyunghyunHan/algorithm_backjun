#include <stdio.h>

int main(){
    int a,b,c,d;
    
    scanf("%d %d",&a,&b);
    scanf("%d %d",&c,&d);
    
    if (a + c < b + d){
         printf("%s","Hanyang Univ.");
    }
   
    else if (a + c > b + d){
    printf("%s","Yongdap");
        
    }
    else{
    printf("%s","Either");    
    }
    

    return 0;
}