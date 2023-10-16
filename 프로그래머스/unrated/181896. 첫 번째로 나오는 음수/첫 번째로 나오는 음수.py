def solution(num_list):
    count= 0
    for index, value in enumerate(num_list):
        if value < 0 :
            count=index
            break
        else :
            count =-1
    return count