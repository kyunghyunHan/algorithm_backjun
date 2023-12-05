import sys 

reader= sys.stdin.readline

t= int(reader())

def dfs(x,y):
    if x<0 or y<0 or x >=m or y>=n:
        return False
    

    if array[x][y]==1:
        array[x][y]=0
        dfs(x - 1, y)
        dfs(x + 1, y)
        dfs(x, y + 1)
        dfs(x, y - 1)
        return True
    return False

for i in range(0,t):
    m,n,k=map(int,sys.stdin.readline().split())

    array = [[0] * n for i in range(m)]


    for j in range(0,k):
        x,y=map(int,sys.stdin.readline().split())
        array[x][y]=1
    
    total= 0
    for i in range(0,m):
        for j in range(0,n):
            if dfs(i,j) == True:
                total+=1
    sys.stdout.write(f"{total}\n")

