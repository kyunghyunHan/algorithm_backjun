import sys

reader = sys.stdin.readline
writer= sys.stdout
n= int(reader())
nums = list(map(int, reader().split()))

nums.sort()

time =0
tmp = 0

for i in range(n):
    tmp+=nums[i]
    time+=tmp
writer.write(str(time) + '\n')
