def determinant(matrix):
    n = len(matrix)
    if n == 1:
        return matrix[0][0]
    if n == 2:
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    det = 0
    for i in range(n):
        det += matrix[0][i] * cofactor(matrix, 0, i)
    return det

def cofactor(matrix, row, col):
    minor_matrix = minor(matrix, row, col)
    sign = 1 if (row + col) % 2 == 0 else -1
    return sign * determinant(minor_matrix)

def minor(matrix, row, col):
    n = len(matrix)
    minor_matrix = [[0] * (n - 1) for _ in range(n - 1)]
    r = 0
    for i in range(n):
        if i == row:
            continue
        c = 0
        for j in range(n):
            if j == col:
                continue
            minor_matrix[r][c] = matrix[i][j]
            c += 1
        r += 1
    return minor_matrix