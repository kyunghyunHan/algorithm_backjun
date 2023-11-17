import sys
def find_prime_num(m):
    i=2
    if m<2:
        return 0
    else:
        while i<=m/i:
            if m%i==0:
                return 0
            i+=1
        
        return 1

input = sys.stdin.readline
m,n=map(int,input().split())

while m<=n:
    if find_prime_num(m)==1:
        print(m)
    m+=1





 
