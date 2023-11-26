from collections import defaultdict
import sys

ma = defaultdict(int)
vt = []

nm = list(map(int, input().strip().split()))
for _ in range(nm[0] + nm[1]):
    name = input().strip()
    ma[name] += 1

    if ma[name] > 1:
        vt.append(name)

vt.sort()

print(len(vt))
for name in vt:
    print(name)


