package kata

const N = 6

type Line []int

type Clue struct {
	First int
	Last  int
}

type Solver struct {
	possibleCols [][]Line
	possibleRows [][]Line
	colClues     []Clue
	rowClues     []Clue
	permutations []Line
}

func NewSolver(clues []int) *Solver {
	colClues, rowClues := prepareClues(clues)

	return &Solver{
		permutations: generateAllLinePermutations(possibleFloors(N)),
		colClues:     colClues,
		rowClues:     rowClues,
	}
}

func SolvePuzzle(clues []int) [][]int {
	return NewSolver(clues).Solve()
}

func (s *Solver) Solve() [][]int {
	s.possibleRows = possibleLines(s.rowClues, s.permutations)
	s.possibleCols = possibleLines(s.colClues, s.permutations)

	return s.buildSolution()
}

func (s *Solver) buildSolution() [][]int {
	for !s.isOneLinePerClue() {
		for i, rows := range s.possibleRows {
			s.possibleRows[i] = filterMatchingOpposites(rows, s.possibleCols, i)
		}

		for i, cols := range s.possibleCols {
			s.possibleCols[i] = filterMatchingOpposites(cols, s.possibleRows, i)
		}
	}

	result := make([][]int, N)
	for i, rows := range s.possibleRows {
		result[i] = rows[0]
	}

	return result
}

func filterMatchingOpposites(lines []Line, oppositeLines [][]Line, position int) []Line {
	var result []Line
	for _, line := range lines {
		isValidLine := true
		for i, elem := range line {
			if !isElementInPosition(elem, position, oppositeLines[i]) {
				isValidLine = false
				break
			}
		}
		if isValidLine {
			result = append(result, line)
		}
	}
	return result
}

func isElementInPosition(elem, position int, lines []Line) bool {
	for _, line := range lines {
		if line[position] == elem {
			return true
		}
	}
	return false
}

func (s *Solver) isOneLinePerClue() bool {
	for _, cols := range s.possibleCols {
		if len(cols) > 1 {
			return false
		}
	}

	for _, rows := range s.possibleRows {
		if len(rows) > 1 {
			return false
		}
	}

	return true
}

func possibleLines(clues []Clue, lines []Line) [][]Line {
	result := make([][]Line, len(clues))
	for i, clue := range clues {
		var cluePossibleLines []Line
		for _, line := range lines {
			if line.matchesClue(clue) {
				cluePossibleLines = append(cluePossibleLines, line)
			}
		}
		result[i] = cluePossibleLines
	}
	return result
}

func prepareClues(clues []int) ([]Clue, []Clue) {
	var colClues, rowClues []Clue
	for i := 0; i < len(clues)/2; i++ {
		clue := Clue{
			First: clues[i],
			Last:  clues[N*(3+i/N)-(i%N+1)],
		}

		if i < N {
			colClues = append(colClues, clue)
		} else {
			clue.First, clue.Last = clue.Last, clue.First
			rowClues = append(rowClues, clue)
		}
	}
	return colClues, rowClues
}

func generateAllLinePermutations(src []int) []Line {
	var result []Line
	for p := make([]int, len(src)); p[0] < len(p); nextPermutation(p) {
		result = append(result, permute(src, p))
	}
	return result
}

func nextPermutation(p []int) {
	for i := len(p) - 1; i >= 0; i-- {
		if i == 0 || p[i] < len(p)-i-1 {
			p[i]++
			return
		}
		p[i] = 0
	}
}

func permute(src, p []int) []int {
	result := make([]int, len(src))
	copy(result, src)
	for i, v := range p {
		result[i], result[i+v] = result[i+v], result[i]
	}
	return result
}

func possibleFloors(size int) []int {
	result := make([]int, size)
	for i := 0; i < size; i++ {
		result[i] = i + 1
	}
	return result
}

func (l Line) countVisible() int {
	var max, count int
	for _, val := range l {
		if val > max {
			count++
			max = val
		}
	}
	return count
}

func (l Line) reverse() Line {
	size := len(l)
	result := make([]int, size)
	for i := len(l) - 1; i >= 0; i-- {
		result[i] = l[size-1-i]
	}
	return result
}

func (l Line) matchesClue(clue Clue) bool {
	return (clue.First == 0 || l.countVisible() == clue.First) &&
		(clue.Last == 0 || l.reverse().countVisible() == clue.Last)
}
