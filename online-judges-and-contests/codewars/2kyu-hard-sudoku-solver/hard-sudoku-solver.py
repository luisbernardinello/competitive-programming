from collections import Counter

BASE_SET = set(range(10))

def has_no_duplicates(blocks):
    return all(set(block) <= BASE_SET and (block.__delitem__(0) or set(block.values()) <= {1}) for block in map(Counter, blocks))

def grid_is_valid(grid):
    return (len(grid) == 9 and all(len(row) == 9 for row in grid)
            and has_no_duplicates(grid)
            and has_no_duplicates(zip(*grid))
            and has_no_duplicates([[grid[x + dx][y + dy] for dx in range(3) for dy in range(3)]
                                   for x in range(0, 9, 3) for y in range(0, 9, 3)]))

def sudoku_solver(board):
    if not grid_is_valid(board):
        raise ValueError("Invalid Sudoku grid")
    
    grid = [[{board[row][col]} if board[row][col] != 0 else {1, 2, 3, 4, 5, 6, 7, 8, 9} for col in range(9)] for row in range(9)]
    solutions = set(depth_first_search(grid, 0, [[0] * 9 for _ in range(9)]))
    
    if len(solutions) > 1:
        raise ValueError("Multiple solutions found")
    if not solutions:
        raise ValueError("No solution exists")
    
    return [list(row) for row in solutions.pop()]

def depth_first_search(grid, count, solution):
    def update_grid(i, j, value, count):
        block_row, block_col = i // 3 * 3, j // 3 * 3
        for index in range(9):
            for x, y in [(i, index), (index, j), (block_row + index // 3, block_col + index % 3)]:
                grid[x][y].discard(value)
                if (x, y) != (i, j) and not (len(grid[x][y]) ^ solution[x][y]):
                    return 0
        
        solution[i][j] = value
        return count + 1

    progress = True
    while count != 81 and progress:
        progress = False
        for row in range(9):
            for col in range(9):
                if len(grid[row][col]) == 1:
                    progress = True
                    count = update_grid(row, col, grid[row][col].pop(), count)
                    if not count:
                        return

    if count == 81:
        yield tuple(map(tuple, solution))
    else:
        _, row, col = min((len(cell), row, col) for row, line in enumerate(grid) for col, cell in enumerate(line) if cell)
        for value in grid[row][col]:
            new_grid = [[cell.copy() for cell in line] for line in grid]
            new_grid[row][col] = {value}
            yield from depth_first_search(new_grid, count, [line[:] for line in solution])