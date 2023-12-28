import sys

n = int(sys.stdin.readline())

time = [[0]*2 for _ in range(n)]
for i in range(0,n):
    k,m= map(int,sys.stdin.readline().split())
    time[i][0] = k
    time[i][1] = m
time.sort(key = lambda x: (x[1], x[0]))
count = 1

end_time = time[0][1]
for i in range(1, n):
    if time[i][0] >= end_time:
        count += 1
        end_time = time[i][1]


sys.stdout.write(f"{count}")