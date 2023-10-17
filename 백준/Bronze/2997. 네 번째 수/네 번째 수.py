def fourth_number(numbers):
    sorted_numbers = sorted(numbers)
    
    diff_1 = sorted_numbers[1] - sorted_numbers[0]
    diff_2 = sorted_numbers[2] - sorted_numbers[1]
    
    if diff_1 == diff_2:
        if sorted_numbers[-1] + diff_1 <= 100:
            answer = sorted_numbers[-1] + diff_1
        else:
            answer = sorted_numbers[0] - diff_1
    elif diff_1 * 2 == diff_2:
        answer = sorted_numbers[1] + diff_1
    elif diff_1 == diff_2 * 2:
        answer = sorted_numbers[0] + diff_2
        
    return answer


if __name__ == "__main__":
    numbers = list(map(int, input().split()))
    print(fourth_number(numbers))