def solution(my_string):
    name_list = list(my_string)  
    name_list.reverse()
    answer = ''.join(name_list)
    return answer