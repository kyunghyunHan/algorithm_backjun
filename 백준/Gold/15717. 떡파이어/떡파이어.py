a,b,c = 2,int(input())-1,1000000007
def power(a,b,c):
    if b<2: return (a**b)%c
    else:
        d = b//2
        return ((pow(a,d,c))**2)%c if b%2 == 0 else (a*(pow(a,d,c))**2)%c
print(power(a, b, c) if b!= -1 else 1)