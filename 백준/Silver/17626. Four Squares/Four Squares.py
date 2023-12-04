import sys

input= sys.stdin.readline

dp=[0,1]
n = int(input())
for i in range(2,n+1):
    min_value = 1e9
    j= 1

    while (j**2) <=i:
        min_value = min(min_value,dp[i-(j**2)])
        j+=1

    dp.append(min_value+1)
print(dp[n])