import sys

input = sys.stdin.readline

a= int(input())
x= int(input())
b= int(input())
y= int(input())
t= int(input())


if t>30:
    a=a+(t-30)*x*21
else:
    a=a

if t>45:
    b=b+(t-45)*y*21
else:
    b=b

print(a,b)