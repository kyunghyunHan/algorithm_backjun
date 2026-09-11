#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

int* solution(const char* keyinput[], size_t keyinput_len,
              int board[], size_t board_len)
{
    int* answer = malloc(sizeof(int) * 2);

    int mx = board[0] / 2;
    int my = board[1] / 2;

    int x = 0;
    int y = 0;

    for (int i = 0; i < keyinput_len; i++)
    {
        if (strcmp(keyinput[i], "left") == 0)
        {
            if (x > -mx)
                x--;
        }
        else if (strcmp(keyinput[i], "right") == 0)
        {
            if (x < mx)
                x++;
        }
        else if (strcmp(keyinput[i], "up") == 0)
        {
            if (y < my)
                y++;
        }
        else if (strcmp(keyinput[i], "down") == 0)
        {
            if (y > -my)
                y--;
        }
    }

    answer[0] = x;
    answer[1] = y;

    return answer;
}