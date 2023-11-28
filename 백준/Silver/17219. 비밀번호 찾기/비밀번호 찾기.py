import sys

n, m = map(int, sys.stdin.readline().split())

database = {}


for _ in range(n):
    site, password = input().split()
    database[site] = password


for _ in range(m):
    site = input().strip()
    if site in database:
        print(database[site])
    else:
        print("Site not found in the database")
