class Mover:
    def __init__(self, pos, dir):
        self.x = pos[0]
        self.y = pos[1]
        self.dx = dir[0]
        self.dy = dir[1]

    def turn_right(self):
        if self.dx == 0:
            self.dx, self.dy = -self.dy, 0
        else:
            self.dx, self.dy = 0, self.dx

    def move(self):
        self.x += self.dx
        self.y += self.dy
        return [self.x, self.y]

    def next(self):
        return [self.x + self.dx, self.y + self.dy]

    def pos(self):
        return [self.x, self.y]

class Game:
    def __init__(self):
        self.count = 0
        self.starters = []
        self.directions = [
            [0, -1, 'Up'], [1, 0, 'Right'], [0, 1, 'Down'], [-1, 0, 'Left']
        ]

    def prepare(self, game_map_string):
        self.gamemap = [list(row) for row in game_map_string.split('\n')]
        for y, row in enumerate(self.gamemap):
            for x, value in enumerate(row):
                if value == '.':
                    self.count += 1
                elif value == 'B':
                    self.starters.append([x, y])

    def get(self, x, y):
        return self.gamemap[y][x]

    def put(self, x, y, value):
        self.gamemap[y][x] = value

    def swap(self, i, j):
        self.starters[i], self.starters[j] = self.starters[j], self.starters[i]

    def is_free(self, pos):
        return self.get(pos[0], pos[1]) == '.'

    def can_move(self, mover):
        next_pos = mover.next()
        return 0 <= next_pos[1] < len(self.gamemap) and 0 <= next_pos[0] < len(self.gamemap[0]) and self.is_free(next_pos)

    def movecolor(self, starter, direction, marker):
        steps = []
        mover = Mover(starter, direction)
        while self.count > 0 and self.can_move(mover):
            steps.append(mover.move())
            self.put(mover.x, mover.y, marker)
            self.count -= 1
            if not self.can_move(mover):
                mover.turn_right()
        return steps

    def restore(self, path):
        for pos in path:
            self.put(pos[0], pos[1], '.')
        self.count += len(path)

    def to_string(self):
        return '\n'.join([''.join(row) for row in self.gamemap])

    def print(self):
        print(self.to_string())

    def permutate(self, index):
        if index == len(self.starters) - 1:
            return self.try_(0)
        else:
            for i in range(index, len(self.starters)):
                self.swap(index, i)
                if self.permutate(index + 1):
                    return True
                self.swap(index, i)
        return False

    def try_(self, depth):
        for i in range(4):
            self.starters[depth].append(self.directions[i])
            steps = self.movecolor(self.starters[depth], self.directions[i], i)
            if steps:
                if self.count == 0 and depth == len(self.starters) - 1:
                    self.solution = [[st[1] - 1, st[0] - 1, st[2][2]] for st in self.starters]
                    return True
                elif self.count > 0 and depth < len(self.starters) - 1:
                    if self.try_(depth + 1):
                        return True
                self.restore(steps)
            self.starters[depth].pop()
        return False

def play_flou(game_map):
    game = Game()
    game.prepare(game_map)
    game.permutate(0)
    return game.solution if hasattr(game, 'solution') else False
