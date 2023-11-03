import sys

def main():
    input_lines = sys.stdin.readlines()
    n = int(input_lines[0])

    points = []
    for line in input_lines[1:]:
        x, y = map(int, line.strip().split())
        points.append((x, y))

    points.sort(key=lambda p: (p[1], p[0]))

    for a, b in points:
        print(a, b)

if __name__ == "__main__":
    main()