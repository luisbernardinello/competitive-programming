package main

import (
	"container/list"
	"fmt"
)

const INF = 1e9

type Aresta struct {
	destino   int
	capacidade int
}

type Grafo struct {
	numVertices  int
	adjacencias [][]Aresta
}

func novoGrafo(n int) *Grafo {
	return &Grafo{
		numVertices:  n,
		adjacencias: make([][]Aresta, n),
	}
}

func (g *Grafo) adicionarAresta(verticeOrigem, verticeDestino, capacidade int) {
	g.adjacencias[verticeOrigem] = append(g.adjacencias[verticeOrigem], Aresta{verticeDestino, capacidade})
	g.adjacencias[verticeDestino] = append(g.adjacencias[verticeDestino], Aresta{verticeOrigem, capacidade})
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func algFordFulkerson(grafo *Grafo, fonte, escoadouro int) int {
	n := grafo.numVertices
	matrizResidual := make([][]int, n)
	for i := range matrizResidual {
		matrizResidual[i] = make([]int, n)
	}
	paiVertices := make([]int, n)
	fluxoMax := 0

	for {
		for i := range paiVertices {
			paiVertices[i] = -1
		}

		fila := list.New()
		fila.PushBack([2]int{fonte, INF})

		for fila.Len() > 0 {
			elem := fila.Front()
			fila.Remove(elem)
			atual := elem.Value.([2]int)[0].(int)
			fluxo := elem.Value.([2]int)[1].(int)

			for _, aresta := range grafo.adjacencias[atual] {
				prox := aresta.destino
				if paiVertices[prox] == -1 && aresta.capacidade-matrizResidual[atual][prox] > 0 {
					paiVertices[prox] = atual
					novoFluxo := min(fluxo, aresta.capacidade-matrizResidual[atual][prox])
					if prox == escoadouro {
						fluxoMax += novoFluxo
						esc := prox
						for esc != fonte {
							ant := paiVertices[esc]
							matrizResidual[ant][esc] += novoFluxo
							matrizResidual[esc][ant] -= novoFluxo
							esc = ant
						}
						break
					}
					fila.PushBack([2]int{prox, novoFluxo})
				}
			}
		}

		if paiVertices[escoadouro] == -1 {
			break
		}
	}

	return fluxoMax
}

func main() {
	var T int
	fmt.Scan(&T)

	for t := 0; t < T; t++ {
		var n, m int
		fmt.Scan(&n, &m)

		grafo := novoGrafo(n)

		for i := 0; i < m; i++ {
			var u, v, c int
			fmt.Scan(&u, &v, &c)
			grafo.adicionarAresta(u-1, v-1, c)
		}

		fonte := 0
		minCut := INF

		for u := 1; u < n; u++ {
			matrizResidualGrafo := *grafo
			fluxoMax := algFordFulkerson(&matrizResidualGrafo, fonte, u)
			minCut = min(minCut, fluxoMax)
		}

		fmt.Println(minCut)
	}
}
