package main

import (
	"fmt"
)

type disjointSetNode struct {
	data      int
	rank      int
	parent    *disjointSetNode
	frequency int
}

var disjointSet map[int]*disjointSetNode

func makeSet(element int) *disjointSetNode {
	root := &disjointSetNode{
		data:      element,
		rank:      0,
		parent:    nil,
		frequency: 1,
	}
	root.parent = root
	disjointSet[element] = root
	return root
}

func findSet(root *disjointSetNode) *disjointSetNode {
	if root.parent != root {
		root.parent = findSet(root.parent)
	}
	return root.parent
}

func union(element1, element2 int) {
	parent1 := findSet(disjointSet[element1])
	parent2 := findSet(disjointSet[element2])

	if parent1.data == parent2.data {
		return
	}

	if parent1.rank >= parent2.rank {
		parent1.rank = parent1.rank | parent2.rank
		parent2.parent = parent1
		parent1.frequency += parent2.frequency
	} else {
		parent1.parent = parent2
		parent2.frequency += parent1.frequency
	}
}

func elementsInSet(element int) int {
	root := findSet(disjointSet[element])
	return root.frequency
}

func main() {
	disjointSet = make(map[int]*disjointSetNode)

	makeSet(1)
	makeSet(2)
	makeSet(3)
	makeSet(4)
	makeSet(5)
	makeSet(6)
	makeSet(7)

	union(1, 2)
	union(2, 3)
	union(4, 5)
	union(5, 6)
	union(3, 4)

	root := findSet(disjointSet[1])
	fmt.Println(elementsInSet((*root).data))
}
