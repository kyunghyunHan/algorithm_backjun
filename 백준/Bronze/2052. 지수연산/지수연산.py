n=int(input())
s= "%.250f"% (2**(-n))
last=len(s)
for i in range(last-1, 1, -1):
    if s[i]!='0':
        last=i
        break
print(s[:last+1])