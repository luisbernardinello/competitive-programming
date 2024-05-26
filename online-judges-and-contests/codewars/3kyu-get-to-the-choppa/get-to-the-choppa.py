from collections import deque

def find_shortest_path(grid, start_node, end_node):
    if not grid or not start_node or not end_node:
        return []


    directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    
    #  BFS structures
    queue = deque([(start_node, [start_node])])
    visited = set([start_node])
    
    while queue:
        current_node, path = queue.popleft()
        
        if current_node == end_node:
            return path
        
        for direction in directions:
            neighbor_x = current_node.position.x + direction[0]
            neighbor_y = current_node.position.y + direction[1]
            
            if 0 <= neighbor_x < len(grid) and 0 <= neighbor_y < len(grid[0]):
                neighbor_node = grid[neighbor_x][neighbor_y]
                if neighbor_node.passable and neighbor_node not in visited:
                    visited.add(neighbor_node)
                    queue.append((neighbor_node, path + [neighbor_node]))
    
    return []