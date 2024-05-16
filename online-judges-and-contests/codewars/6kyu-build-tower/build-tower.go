package kata

import "strings"

func TowerBuilder(nFloors int) []string {
	tower := make([]string, nFloors)
	maxWidth := 2*nFloors - 1
	for i := 0; i < nFloors; i++ {
		numAsterisks := 2*i + 1
		numSpaces := (maxWidth - numAsterisks) / 2
		tower[i] = strings.Repeat(" ", numSpaces) + strings.Repeat("*", numAsterisks) + strings.Repeat(" ", numSpaces)
	}
	return tower
}
