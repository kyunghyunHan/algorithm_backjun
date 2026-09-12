#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int compare(const void *a, const void *b)
{
    const char *s1 = *(const char **)a;
    const char *s2 = *(const char **)b;

    return strcmp(s1, s2);
}

char** solution(const char* my_string)
{
    int len = strlen(my_string);

    char **answer = malloc(sizeof(char *) * len);

    for (int i = 0; i < len; i++)
    {
        answer[i] = (char *)(my_string + i);
    }

    qsort(answer, len, sizeof(char *), compare);

    return answer;
}