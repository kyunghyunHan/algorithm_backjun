from collections import deque

import sys

n= int(input())
dq = deque(range(1, n+1))

result= []
while 1 < len(dq) :
     if  len(dq)<1 :
         print(dq[0])
         break
     else :
         result.append(dq.popleft()) 
         dq.append(dq.popleft())
result.append(dq[0])
for i in result:
    print(i,end=" ")