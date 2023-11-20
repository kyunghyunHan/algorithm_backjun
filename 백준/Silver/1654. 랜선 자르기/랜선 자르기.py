import sys

# 최대 랜선수
N_MAX = 1_000_000
# 최대 길이
K_MAX = 10_000

# 입력 받기
k, n = map(int, input().split())

nums = [0] * K_MAX

# 각 랜선의 길이 입력
for i in range(k):
    nums[i] = int(input())

# 랜선 길이의 총합 계산
sum_length = sum(nums[:k])

# 시작, 끝, 최대 길이 초기화
start = 1
end = sum_length // n + 1
max_len = float('-inf')

# 이진 탐색을 이용한 최적의 랜선 길이 찾기
while start < end:
    # 중간 길이
    middle_len = start + (end - start) // 2
    # 남은 길이
    leftover = sum_length - (middle_len * n)
    # 남은 값들의 합
    leftover_sum = 0

    # 각 랜선의 길이를 이용하여 남은 값들의 합 계산
    for num in nums[:k]:
        leftover_sum += num % middle_len
        # 초과하면
        if leftover_sum > leftover:
            end = middle_len
            # 바깥쪽 반복문으로 이동
            break

    else:  # 만약 break가 발생하지 않으면 (모든 랜선이 해당 조건에 부합)
        # max_len 값을 업데이트
        max_len = max(middle_len, max_len)
        # 다음 반복을 위해 start를 조정
        start = middle_len + 1

# 최적의 랜선 길이 출력
print(max_len)