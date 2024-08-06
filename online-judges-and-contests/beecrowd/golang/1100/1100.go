package main

import (
	"fmt"
)

type Point struct {
	row, col int
}

var moves = [8]Point{{1, 2}, {2, 1}, {2, -1}, {1, -2}, {-1, -2}, {-2, -1}, {-2, 1}, {-1, 2}}

func knightMoves(start, end string) int {
	startRow, startCol := int(start[1]-'0')-1, int(start[0]-'a')
	endRow, endCol := int(end[1]-'0')-1, int(end[0]-'a')

	dist := make([][]int, 8)
	for i := range dist {
		dist[i] = make([]int, 8)
		for j := range dist[i] {
			dist[i][j] = -1
		}
	}

	dist[startRow][startCol] = 0
	queue := []Point{{startRow, startCol}}

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if current.row == endRow && current.col == endCol {
			return dist[current.row][current.col]
		}

		for _, move := range moves {
			newRow, newCol := current.row+move.row, current.col+move.col
			if newRow >= 0 && newRow < 8 && newCol >= 0 && newCol < 8 && dist[newRow][newCol] == -1 {
				dist[newRow][newCol] = dist[current.row][current.col] + 1
				queue = append(queue, Point{newRow, newCol})
			}
		}
	}

	return -1
}

func main() {
	var start, end string
	for {
		_, err := fmt.Scan(&start, &end)
		if err != nil {
			break
		}
		moves := knightMoves(start, end)
		fmt.Printf("To get from %s to %s takes %d knight moves.\n", start, end, moves)
	}
}
