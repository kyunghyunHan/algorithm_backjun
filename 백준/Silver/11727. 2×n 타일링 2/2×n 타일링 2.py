import sys

n = int(sys.stdin.readline())

dp = [0] * 1001

dp[1]= 1
dp[2]= 3
dp[3]= 5

for i in range(4,n+1):
    dp[i]=(dp[i-2]*2)+dp[i-1]
    dp[i]%=10007

sys.stdout.write(f"{dp[n]}")

