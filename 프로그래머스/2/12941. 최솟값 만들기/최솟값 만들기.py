def solution(A,B):
    A.sort()
    B.sort(reverse=True)
    cnt=0
    for i in range(0,len(A)):
        cnt+=(A[i]*B[i])
    return cnt