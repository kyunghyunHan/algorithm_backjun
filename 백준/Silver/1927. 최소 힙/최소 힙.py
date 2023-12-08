import sys
import heapq

n = int(sys.stdin.readline())
heap = [] 

for i in range(n):
    x = int(sys.stdin.readline())

    
    if x == 0:

        if heap:
            sys.stdout.write(f"{heapq.heappop(heap)}\n")
        else:
            sys.stdout.write(f"{0}\n")

   
    else:
        heapq.heappush(heap, x)