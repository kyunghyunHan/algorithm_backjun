from collections import deque
import sys

def main():
    n, m = map(int, input().split())

    campus = []
    start = (0, 0)

    for i in range(n):
        row = list(input().strip())
        for j in range(m):
            if row[j] == 'I':
                start = (i, j)
        campus.append(row)

    directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    visited = [[False for _ in range(m)] for _ in range(n)]
    res = 0
    queue = deque([start])
    visited[start[0]][start[1]] = True

    while queue:
        x, y = queue.popleft()

        for dx, dy in directions:
            nx = x + dx
            ny = y + dy

            if 0 <= nx < n and 0 <= ny < m:
                if campus[nx][ny] != 'X' and not visited[nx][ny]:
                    queue.append((nx, ny))
                    visited[nx][ny] = True

                    if campus[nx][ny] == 'P':
                        res += 1

    if res > 0:
        print(res)
    else:
        print("TT")

if __name__ == "__main__":
    main()
