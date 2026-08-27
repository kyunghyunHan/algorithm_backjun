#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <math.h>
int solution(int n) {
    int answer = 0;
    
    if (n%2!=0){
        for (int i = 1;i<=n;i++){
            if (i%2!=0){
                answer+=i;
            }
            
        }
    }else{
        for (int i = 1;i<=n;i++){
             if(i%2==0){
               answer+=pow(i,2);
             }
        }
    }
    return answer;
}