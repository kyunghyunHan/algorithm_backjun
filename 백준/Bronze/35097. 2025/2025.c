#include <stdio.h>

int main() {
    int n;
    
    while (1) {
        scanf("%d", &n);
        
        if (n == 0)
            break;
        
        long long sum = (long long)n * (n + 1) / 2;
        long long result = sum * sum;
        
        printf("%lld\n", result);
    }
    
    return 0;
}