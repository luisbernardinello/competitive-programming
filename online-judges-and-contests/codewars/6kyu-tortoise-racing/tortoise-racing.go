package kata

func Race(v1, v2, g int) [3]int {
	if v1 >= v2 {
		return [3]int{-1, -1, -1}
	}

	catch := float64(g) / float64(v2-v1)

	hours := int(catch)
	minutes := int(catch*60) % 60
	seconds := int(catch*3600) % 60
	return [3]int{hours, minutes, seconds}
}
