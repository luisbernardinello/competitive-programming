package kata

func CountContiguousDistinct(k int, arr []int) []int {
	if len(arr) < k {
		return nil
	}

	var result []int
	countMap := make(map[int]int)

	for i := 0; i < k; i++ {
		countMap[arr[i]]++
	}
	result = append(result, len(countMap))

	for i := k; i < len(arr); i++ {
		countMap[arr[i-k]]--
		if countMap[arr[i-k]] == 0 {
			delete(countMap, arr[i-k])
		}
		countMap[arr[i]]++
		result = append(result, len(countMap))
	}

	return result
}
