from collections import deque
import sys

n, m = map(int, input().split())
graph = []

for _ in range(n):
    row = list(map(int, input().strip()))
    graph.append(row)

dx = [-1, 1, 0, 0]
dy = [0, 0, -1, 1]

def bfs(x, y, dx, dy, n, m, graph):
    queue = deque()
    queue.append((x, y))

    while queue:
        x, y = queue.popleft()

        for i in range(4):
            nx = x + dx[i]
            ny = y + dy[i]

            if 0 <= nx < n and 0 <= ny < m and graph[nx][ny] == 1:
                queue.append((nx, ny))
                graph[nx][ny] = graph[x][y] + 1
    
    return graph[n - 1][m - 1]

result = bfs(0, 0, dx, dy, n, m, graph)
print(result)
