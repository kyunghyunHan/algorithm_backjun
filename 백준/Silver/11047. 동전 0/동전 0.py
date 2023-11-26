import sys

n, k = map(int, sys.stdin.readline().split())
arr = []

for i in range(n):
    price = int(sys.stdin.readline())
    arr.append(price)

total = 0

for i in range(n - 1, -1, -1):
    total += k // arr[i]
    k = k % arr[i]

print(total)
