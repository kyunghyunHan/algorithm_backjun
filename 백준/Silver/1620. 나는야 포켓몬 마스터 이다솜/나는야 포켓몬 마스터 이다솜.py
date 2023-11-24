inputs = input().split()
n = int(inputs[0])
m = int(inputs[1])

shhm = {}
nshm = {}

for i in range(n):
    name = input().strip()
    shhm[name] = i + 1
    nshm[i + 1] = name

results = []

for _ in range(m):
    query = input().strip()

    try:
        k = int(query)
        if k in nshm:
            results.append(nshm[k])
    except ValueError:
        if query in shhm:
            results.append(str(shhm[query]))

print("\n".join(results))
