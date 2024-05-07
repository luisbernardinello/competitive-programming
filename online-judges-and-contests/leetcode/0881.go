
import (
	"sort"
	"testing"
)

func minRescueBoats(peopleWeights []int, weightLimit int) int {
	sort.Ints(peopleWeights)
	i, j := 0, len(peopleWeights)-1
	boats := 0
	for i <= j {
		if peopleWeights[i]+peopleWeights[j] <= weightLimit {
			i++
		}
		j--
		boats++
	}
	return boats
}

func TestMinRescueBoats(t *testing.T) {
	tests := []struct {
		peopleWeights []int
		weightLimit   int
		expected      int
	}{
		{
			peopleWeights: []int{1, 2},
			weightLimit:   3,
			expected:      1,
		},
		{
			peopleWeights: []int{3, 2, 2, 1},
			weightLimit:   3,
			expected:      3,
		},
		{
			peopleWeights: []int{3, 5, 3, 4},
			weightLimit:   5,
			expected:      4,
		},
	}

	for _, test := range tests {
		result := minRescueBoats(test.peopleWeights, test.weightLimit)
		if result != test.expected {
			t.Errorf("For people weights %v with weight limit %d, expected %d, but got %d", test.peopleWeights, test.weightLimit, test.expected, result)
		}
	}
}
