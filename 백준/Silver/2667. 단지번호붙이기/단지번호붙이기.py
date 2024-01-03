from collections import deque

def bfs(dx, dy, a, b, graph):
    n = len(graph)
    queue = deque()
    queue.append((a, b))
    graph[a][b] = 0
    count = 1

    while queue:
        x, y = queue.popleft()
        for i in range(4):
            nx = x + dx[i]
            ny = y + dy[i]
            if 0 <= nx < n and 0 <= ny < n and graph[nx][ny] == 1:
                graph[nx][ny] = 0
                queue.append((nx, ny))
                count += 1

    return count


n = int(input())
graph = []
for _ in range(n):
    row = list(map(int, input().strip()))
    graph.append(row)

dx = [0, 0, 1, -1]
dy = [1, -1, 0, 0]
cnt = []
for i in range(n):
    for j in range(n):
        if graph[i][j] == 1:
            cnt.append(bfs(dx, dy, i, j, graph))

cnt.sort()
print(len(cnt))
for i in cnt:
    print(i)
