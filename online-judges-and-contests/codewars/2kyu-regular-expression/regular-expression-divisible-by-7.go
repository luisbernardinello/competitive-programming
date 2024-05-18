package kata

import "regexp"

var Solution = regexp.MustCompile(`^(0|1(01*0)*1)*$`)
