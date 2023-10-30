def solution(array):
    array.sort()
    centerIndex = int(len(array)/2)
    answer = array[centerIndex]
    return answer