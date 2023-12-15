import sys

n = int(sys.stdin.readline())
paper = [list(map(int, sys.stdin.readline().split())) for _ in range(n)] 

result = []

def solution(x, y, n) :
  color = paper[x][y]
  for i in range(x, x+n) :
    for j in range(y, y+n) :
      if color != paper[i][j] :
        solution(x, y, n//2)
        solution(x, y+n//2, n//2)
        solution(x+n//2, y, n//2)
        solution(x+n//2, y+n//2, n//2)
        return
  if color == 0 :
    result.append(0)
  else :
    result.append(1)


solution(0,0,n)
sys.stdout.write(f"{result.count(0)}\n")
sys.stdout.write(f"{result.count(1)}")