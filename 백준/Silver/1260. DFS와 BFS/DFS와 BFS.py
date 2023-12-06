import sys

n,m,v = map(int,sys.stdin.readline().split())

graph = [[0]*(n+1) for _ in range(n+1)]
for i in range (0,m):
    a,b = map(int,sys.stdin.readline().split())
    graph[a][b] = graph[b][a] = 1

visited1 = [0]*(n+1)
visited2 = [0]*(n+1)

def dfs(v):
    visited1[v] = 1 
    sys.stdout.write(f"{v} ")
    for i in range(1, n+1):
        if graph[v][i] == 1 and visited1[i] == 0:
            dfs(i)

def bfs(v):
    queue = [v]
    visited2[v] = 1 #방문처리
    while queue:
        v = queue.pop(0) #방문 노드 제거
        sys.stdout.write(f"{v} ")
        for i in range(1, n+1):
            if(visited2[i] == 0 and graph[v][i] == 1):
                queue.append(i)
                visited2[i] = 1 # 방문처리

dfs(v)
sys.stdout.write(f"\n")
bfs(v)