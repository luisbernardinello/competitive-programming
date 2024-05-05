from collections import deque

def knight_moves(start, end):
    //movimentos do cavalo
    moves = [(1, 2), (2, 1), (2, -1), (1, -2), (-1, -2), (-2, -1), (-2, 1), (-1, 2)]
   
    start_row, start_col = 8 - int(start[1]), ord(start[0]) - ord('a')
    end_row, end_col = 8 - int(end[1]), ord(end[0]) - ord('a')
    
  
    dist = [[-1] * 8 for _ in range(8)]
    dist[start_row][start_col] = 0
    queue = deque([(start_row, start_col)])
    
   //bfs
    while queue:
        current_row, current_col = queue.popleft()
        if current_row == end_row and current_col == end_col:
            return dist[current_row][current_col]
        
        for dr, dc in moves:
            new_row, new_col = current_row + dr, current_col + dc
            if 0 <= new_row < 8 and 0 <= new_col < 8 and dist[new_row][new_col] == -1:
                dist[new_row][new_col] = dist[current_row][current_col] + 1
                queue.append((new_row, new_col))

//caso de teste
while True:
    try:
        start, end = input().split()
        moves = knight_moves(start, end)
        print(f"To get from {start} to {end} takes {moves} knight moves.")
    except EOFError:
        break
