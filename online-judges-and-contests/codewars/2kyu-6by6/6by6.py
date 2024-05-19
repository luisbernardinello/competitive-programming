from itertools import permutations
from typing import List, Tuple

class ViewClue:
    def __init__(self, front: int, back: int):
        self.front = front
        self.back = back

class BuildingRow(list):
    pass

grid_size = 6

class PuzzleSolver:
    def __init__(self, clues: List[int]):
        self.column_clues, self.row_clues = self.transform_clues(clues)
        self.all_perms = list(permutations(range(1, grid_size + 1)))
        self.perms_map = {tuple(perm): True for perm in self.all_perms}

    def solve(self) -> List[List[int]]:
        self.candidate_rows = self.filter_rows(self.row_clues, self.all_perms)
        self.candidate_cols = self.filter_rows(self.column_clues, self.all_perms)

        while not self.is_unique_solution():
            self.candidate_rows = [
                self.filter_valid_rows(rows, self.candidate_cols, i)
                for i, rows in enumerate(self.candidate_rows)
            ]
            self.candidate_cols = [
                self.filter_valid_rows(cols, self.candidate_rows, i)
                for i, cols in enumerate(self.candidate_cols)
            ]

        return [list(rows[0]) for rows in self.candidate_rows]

    def filter_valid_rows(self, rows: List[BuildingRow], opposing_rows: List[List[BuildingRow]], position: int) -> List[BuildingRow]:
        return [row for row in rows if self.is_row_valid(row, position, opposing_rows)]

    def is_row_valid(self, row: BuildingRow, position: int, opposing_rows: List[List[BuildingRow]]) -> bool:
        for i, value in enumerate(row):
            if not self.contains_value_at_position(value, position, opposing_rows[i]):
                return False
        return True

    def contains_value_at_position(self, value: int, position: int, rows: List[BuildingRow]) -> bool:
        return any(row[position] == value for row in rows)

    def is_unique_solution(self) -> bool:
        return all(len(rows) == 1 for rows in self.candidate_rows) and all(len(cols) == 1 for cols in self.candidate_cols)

    def filter_rows(self, clues: List[ViewClue], rows: List[BuildingRow]) -> List[List[BuildingRow]]:
        return [
            [row for row in rows if self.matches_clue(row, clue)]
            for clue in clues
        ]

    def transform_clues(self, clues: List[int]) -> Tuple[List[ViewClue], List[ViewClue]]:
        column_clues = [
            ViewClue(clues[i], clues[grid_size * 3 - (i % grid_size) - 1])
            for i in range(grid_size)
        ]
        row_clues = [
            ViewClue(clues[grid_size * 2 + i], clues[grid_size + i])
            for i in range(grid_size)
        ]
        return column_clues, row_clues

    def matches_clue(self, row: BuildingRow, clue: ViewClue) -> bool:
        return (clue.front == 0 or self.count_visible_buildings(row) == clue.front) and (
            clue.back == 0 or self.count_visible_buildings(row[::-1]) == clue.back
        )

    def count_visible_buildings(self, row: BuildingRow) -> int:
        max_height = 0
        visible_count = 0
        for height in row:
            if height > max_height:
                visible_count += 1
                max_height = height
        return visible_count

def solve_puzzle(clues: List[int]) -> List[List[int]]:
    solver = PuzzleSolver(clues)
    return solver.solve()
