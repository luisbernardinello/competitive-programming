package kata

import (
	"fmt"
	"strings"
)

func FormatDuration(seconds int64) string {
	if seconds == 0 {
		return "now"
	}

	years := seconds / (365 * 24 * 3600)
	seconds %= 365 * 24 * 3600
	days := seconds / (24 * 3600)
	seconds %= 24 * 3600
	hours := seconds / 3600
	seconds %= 3600
	minutes := seconds / 60
	seconds %= 60

	components := []string{}

	if years > 0 {
		components = append(components, formatComponent(years, "year"))
	}
	if days > 0 {
		components = append(components, formatComponent(days, "day"))
	}
	if hours > 0 {
		components = append(components, formatComponent(hours, "hour"))
	}
	if minutes > 0 {
		components = append(components, formatComponent(minutes, "minute"))
	}
	if seconds > 0 {
		components = append(components, formatComponent(seconds, "second"))
	}

	if len(components) == 1 {
		return components[0]
	}

	return strings.Join(components[:len(components)-1], ", ") + " and " + components[len(components)-1]
}

func formatComponent(value int64, unit string) string {
	if value == 1 {
		return fmt.Sprintf("%d %s", value, unit)
	}
	return fmt.Sprintf("%d %s", value, unit+"s")
}
