#include <stdio.h>
#include <string.h>

int main() {
    while (1) {
        char s[10];

        scanf("%s", s);

        if (strcmp(s, "end") == 0) {
            break;
        } 
        else if (strcmp(s, "animal") == 0) {
            printf("Panthera tigris\n");
        } 
        else if (strcmp(s, "tree") == 0) {
            printf("Pinus densiflora\n");
        } 
        else if (strcmp(s, "flower") == 0) {
            printf("Forsythia koreana\n");
        }
    }

    return 0;
}
