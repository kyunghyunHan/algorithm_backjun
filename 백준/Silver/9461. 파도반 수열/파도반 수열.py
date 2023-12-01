import sys

n = int(sys.stdin.readline())

dp = [0] * 100

dp[0]= 1
dp[1]= 1
dp[2]= 1
dp[3]= 2
dp[4]= 2

for i in range(5,100):
    dp[i]=dp[i-1] + dp[i -5]

for i in range(0,n):
    m = int(sys.stdin.readline())
    sys.stdout.write(f"{dp[m-1]}\n")


