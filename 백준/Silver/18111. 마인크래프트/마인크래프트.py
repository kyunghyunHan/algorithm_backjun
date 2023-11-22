import sys

# 입력 받기
n, m, b = map(int, sys.stdin.readline().split())

matrix = []

t = sys.maxsize
idx = 0
for _ in range(n):
    matrix.append([int(x) for x in sys.stdin.readline().split()])

for target in range(0, 257):
    max_target = 0
    min_target = 0
    for i in range(0, n):
        for j in range(0, m):
            if matrix[i][j] >= target:
                max_target += matrix[i][j] - target
            else:
                min_target += target - matrix[i][j]
    
    if max_target + b >= min_target:
        if min_target + (max_target * 2) <= t:
            t = min_target + (max_target * 2)
            idx = target

sys.stdout.write(f"{t} {idx}")
sys.stdout.flush()
