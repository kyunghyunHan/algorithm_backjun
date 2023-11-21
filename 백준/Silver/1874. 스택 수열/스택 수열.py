import sys



# 입력 받기
n= int(sys.stdin.readline())

cur= 0
stack=[0]
answer=[]

#수열 입력받기

for _ in range(0,n):
    num= int(sys.stdin.readline())

    while cur <num :
        cur+=1
        stack.append(cur)
        answer.append("+")

        if cur ==num:
            stack.pop()
            answer.append("-")
    
    if cur >num:
        if stack[len(stack)-1]<num:
            stack.append(-1)
            break
        
        while stack[len(stack)-1] >=num:
            stack.pop()
            answer.append("-")

if len(stack) != 1:
    sys.stdout.write("NO\n")
else:
    for i in answer:
        sys.stdout.write(f"{i}\n")
    sys.stdout.flush()