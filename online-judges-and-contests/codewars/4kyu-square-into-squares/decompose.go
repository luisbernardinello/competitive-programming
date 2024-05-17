package kata

func Decompose(n int64) []int64 {
	result := decomposer(n*n, n)
	if len(result) == 0 {
		return nil
	}
	return result
}

func decomposer(target, current int64) []int64 {
	if target == 0 {
		return []int64{}
	}
	for i := current - 1; i > 0; i-- {
		if remainder := target - i*i; remainder >= 0 {
			if result := decomposer(remainder, i); result != nil || remainder == 0 {
				return append(result, i)
			}
		}
	}
	return nil
}
