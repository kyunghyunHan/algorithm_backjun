import sys

t = int(sys.stdin.readline().strip())

for _ in range(t):
    n = int(sys.stdin.readline().strip())
    clothes = {}

    for _ in range(n):
        line = sys.stdin.readline().strip().split()
        c_t = line[1]

        if c_t not in clothes:
            clothes[c_t] = 1
        else:
            clothes[c_t] += 1

    ans = 1
    for val in clothes.values():
        ans *= (val + 1)

    print(ans - 1)
