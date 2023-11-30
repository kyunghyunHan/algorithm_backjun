import sys
from collections import deque
n = int(sys.stdin.readline())

dp= [0]*1001

dp[1]=1
dp[2]=2
dp[3]=4
for j in range(4,11):
    dp[j]=dp[j-1]+dp[j-2]+dp[j-3]

for _ in range(0,n):
    n = int(sys.stdin.readline())
    sys.stdout.write(f"{dp[n]}\n")


sys.stdout.flush() 