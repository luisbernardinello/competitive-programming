package kata

import (
	"strconv"
	"strings"
)

func ipToInt(ip string) int {
	parts := strings.Split(ip, ".")
	var result int
	for i, part := range parts {
		num, _ := strconv.Atoi(part)
		result += num << (8 * (3 - i))
	}
	return result
}

func IpsBetween(start, end string) int {
	startInt := ipToInt(start)
	endInt := ipToInt(end)
	return endInt - startInt
}
