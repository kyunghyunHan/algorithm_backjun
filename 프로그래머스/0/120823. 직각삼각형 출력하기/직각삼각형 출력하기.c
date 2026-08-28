#include <stdio.h>

int main(void) {
    int n;
    scanf("%d", &n);
    char *s = (char*) malloc(sizeof(char)*1000);
    int index = 0;
    for (int i = 0;i<n;i++){
        for (int j = 0;j<=i;j++){
            s[index++] = '*';
        }
        s[index++] = '\n';
    }
    
    s[index] = '\0'; 
    printf("%s",s);
    return 0;
}