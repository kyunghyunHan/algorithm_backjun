import sys

n = int(sys.stdin.readline().strip())
m = int(sys.stdin.readline().strip())
s = sys.stdin.readline().strip()

answer, i, count = 0, 0, 0

while i < (m - 2):
    if i + 2 < len(s) and s[i:i+3] == 'IOI':
        i += 2
        count += 1
        if count == n:
            answer += 1
            count -= 1
    else:
        i += 1
        count = 0

print(answer)