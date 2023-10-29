def solution(num_list):

    odd_num, even_num = 0, 0

    for  i in num_list:
        if i % 2 == 0:
            odd_num+=1
        elif i % 2 != 0:
            even_num+=1
            
            
    answer = [odd_num,even_num]   
    return answer
