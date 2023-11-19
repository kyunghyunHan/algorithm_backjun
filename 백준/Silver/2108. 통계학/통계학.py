from collections import Counter
import sys


n = int(sys.stdin.readline())
v = []
for _ in range(n):
    m = int(sys.stdin.readline())
    v.append(m)

if n == 1:
    for _ in range(3):
        print(v[0])
    print(0)
    sys.stdout.flush()
    sys.exit()

v.sort()


a = round(sum(v) / n)


b = v[n // 2]


count = Counter(v)
max_freq = max(count.values())
mode_candidates = [num for num, freq in count.items() if freq == max_freq]
c = min(mode_candidates) if len(mode_candidates) == 1 else mode_candidates[1]


d = v[-1] - v[0]

# Printing output
print(a)
print(b)
print(c)
print(d)
sys.stdout.flush()
