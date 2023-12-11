def dfs(v, visited, graph):
    visited[v] = True
    for i in graph[v]:
        if not visited[i]:
            dfs(i, visited, graph)

input_values = input().split()
n = int(input_values[0])
m = int(input_values[1])

graph = [[] for _ in range(n + 1)]
visited = [False for _ in range(n + 1)]

for _ in range(m):
    s, e = map(int, input().split())
    graph[s].append(e)
    graph[e].append(s)

count = 0
for i in range(1, n + 1):
    if not visited[i]:
        count += 1
        dfs(i, visited, graph)

print(count)


