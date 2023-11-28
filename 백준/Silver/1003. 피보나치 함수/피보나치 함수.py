import sys

t = int(sys.stdin.readline())

for _ in range(0,t) :
    n = int(sys.stdin.readline())
    zero=1
    one=0

    for _ in range(0,n):
        temp=zero
        zero=one
        one=zero+temp
    
    sys.stdout.write(f"{zero} {one}\n")

sys.stdout.flush() 