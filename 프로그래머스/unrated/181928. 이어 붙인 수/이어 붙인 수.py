def solution(num_list):
    first= ''
    second=''
    for i in num_list:
        if i%2==0:
            second+=str(i)
        else:
            first+=str(i)

    answer = int(first)+int(second)
    return answer