def solution(s):
    numbers = list(map(int, s.split()))  # 문자열을 공백으로 분리하여 정수 리스트로 변환
    min_num = str(min(numbers))  # 리스트에서 최소값 찾기
    max_num = str(max(numbers))  # 리스트에서 최대값 찾기
    answer = min_num + " " + max_num  # 최소값과 최대값을 문자열로 합치기
    return answer