B = 15
POS = {}

for n in range(1 << B):
    s = bin(n)[2:].zfill(B)
    ones_lengths = []
    length = 0

    for char in s:
        if char == '1':
            length += 1
        elif length > 0:
            ones_lengths.append(length)
            length = 0

    if length > 0:
        ones_lengths.append(length)
        
    key = tuple(ones_lengths)
    if key not in POS:
        POS[key] = set()
    POS[key].add(tuple(int(c) for c in s))

def solve(clues):
    clues = {'V': clues[0], 'H': clues[1]}
    grid = { (d, z): list(POS[clues[d][z]]) for d in 'VH' for z in range(B) }

    changed = True
    while changed:
        changed = False

        for x in range(B):
            for y in range(B):
                tupH, iH, tupV, iV = ('H', x), y, ('V', y), x
                if len(grid[tupH]) == 1 and len(grid[tupV]) == 1:
                    continue

                vH = { v[iH] for v in grid[tupH] }
                vV = { v[iV] for v in grid[tupV] }
                target = vH & vV

                if len(vH) == 2 and len(target) == 1:
                    changed = True
                    grid[tupH] = [t for t in grid[tupH] if t[iH] in target]

                if len(vV) == 2 and len(target) == 1:
                    changed = True
                    grid[tupV] = [t for t in grid[tupV] if t[iV] in target]

    return tuple( grid[('H', n)][0] for n in range(B) )
