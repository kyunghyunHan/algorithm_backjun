from collections import deque
import sys

input = sys.stdin.readline

t = int(input().strip())

for _ in range(t):
    n, m = map(int, input().split())
    queue = deque(enumerate(map(int, input().split())))
    
    count = 0
    while True:
        max_value = max(queue, key=lambda x: x[1])[1]
        front = queue.popleft()
        
        if front[1] == max_value:
            count += 1
            if front[0] == m:
                print(count)
                break
        else:
            queue.append(front)
