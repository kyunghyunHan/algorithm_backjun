import sys

def my_round(val):
    return int(val) + 1 if val - int(val) >= 0.5 else int(val)


input = sys.stdin.readline
n = int(input())
#n이면 종료
if n:
#배열에 담고
    arr = [int(input()) for _ in range(n)]
    arr.sort()
    nn = my_round(n * 0.15)
    print(my_round(sum(arr[nn:-nn] if nn else arr) / (n - 2 * nn)))
else:
    print(0)
 
