def solution(s):
    stack = []

    for bracket in s:
        if bracket == '(':
            stack.append(bracket)
        elif bracket == ')':
            if not stack:
                return False  # 괄호가 쌍을 이루지 않는 경우
            stack.pop()

    return len(stack) == 0  # 스택이 비어있으면 올바른 괄호열