import sys

reader = sys.stdin.readline
writer = sys.stdout

n = reader().strip()  
c = 0

while len(n)>=2:
    t = sum(map(int, n))  
    n = str(t)
    c += 1

writer.write(str(c) + '\n')

if int(n) % 3 == 0:
    writer.write("YES\n")
else:
    writer.write("NO\n")
