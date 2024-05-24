def validate_battlefield(field):
    def is_valid_position(x, y):
        return 0 <= x < 10 and 0 <= y < 10

    def mark_ship(x, y):
        positions = [(x, y)]
        field[x][y] = 2
        size = 1

        # Check horizontally and vertically
        for dx, dy in [(1, 0), (0, 1)]:
            length = 1
            while is_valid_position(x + length * dx, y + length * dy) and field[x + length * dx][y + length * dy] == 1:
                field[x + length * dx][y + length * dy] = 2
                positions.append((x + length * dx, y + length * dy))
                length += 1
            size = max(size, length)
        
        return size, positions

    def check_adjacent(positions):
        for x, y in positions:
            for dx in [-1, 0, 1]:
                for dy in [-1, 0, 1]:
                    if dx == 0 and dy == 0:
                        continue
                    nx, ny = x + dx, y + dy
                    if is_valid_position(nx, ny) and field[nx][ny] == 1:
                        return False
        return True

    ships = {4: 1, 3: 2, 2: 3, 1: 4}
    found_ships = {4: 0, 3: 0, 2: 0, 1: 0}
    
    for i in range(10):
        for j in range(10):
            if field[i][j] == 1:
                size, positions = mark_ship(i, j)
                if not check_adjacent(positions):
                    return False
                if size in ships:
                    found_ships[size] += 1
                else:
                    return False
    
    return found_ships == ships