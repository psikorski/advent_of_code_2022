def ParseDir(sizes, file, size = 0):
    for line in file:
        if line[:4] == '$ cd':
            if line.split()[2] == "..": break
            size += ParseDir(sizes, file)[-1]
        fileSize = line.split()[0]
        if fileSize.isnumeric(): size += int(fileSize)
    sizes.append(size)
    return sizes

sizes = sorted(ParseDir([], open("input.txt"))[:-1])
print(len(sizes))
for s in sizes:
    if s < 100000:
        print(s)
print(sum([size for size in sizes if size < 100000]))
print(next(x for x in sizes if x > sizes[-1] - 40000000))