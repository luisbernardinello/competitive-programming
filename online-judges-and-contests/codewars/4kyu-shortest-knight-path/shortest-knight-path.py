def algebraic_to_cartesian(pos):
    col = ord(pos[0]) - ord('a')
    row = int(pos[1]) - 1
    return row, col

def knight_moves(row, col):
    moves = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)]
    valid_moves = []
    for dr, dc in moves:
        new_row, new_col = row + dr, col + dc
        if 0 <= new_row < 8 and 0 <= new_col < 8:
            valid_moves.append((new_row, new_col))
    return valid_moves

def knight(p1, p2):
    start_row, start_col = algebraic_to_cartesian(p1)
    target_row, target_col = algebraic_to_cartesian(p2)
    
    queue = [(start_row, start_col)]
    visited = set(queue)
    moves = 0
    
    while queue:
        new_queue = []
        for row, col in queue:
            if row == target_row and col == target_col:
                return moves
            for new_row, new_col in knight_moves(row, col):
                if (new_row, new_col) not in visited:
                    visited.add((new_row, new_col))
                    new_queue.append((new_row, new_col))
        queue = new_queue
        moves += 1

