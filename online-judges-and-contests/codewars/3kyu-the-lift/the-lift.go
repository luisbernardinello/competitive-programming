package kata

type TheLiftSystem struct {
	queues        [][]int
	capacity      int
	elevator      []int
	peopleWaiting bool
}

func NewTheLiftSystem(queues [][]int, capacity int) *TheLiftSystem {
	return &TheLiftSystem{
		queues:        queues,
		capacity:      capacity,
		peopleWaiting: true,
	}
}

func TheLift(queues [][]int, capacity int) []int {
	d := NewTheLiftSystem(queues, capacity)
	return d.TheLift()
}

func (d *TheLiftSystem) TheLift() []int {
	stops := []int{0}

	for d.peopleWaiting {
		d.peopleWaiting = false

		stops = d.runElevator(stops, "up")
		stops = d.runElevator(stops, "down")
	}

	if stops[len(stops)-1] != 0 {
		stops = append(stops, 0)
	}

	return stops
}

func (d *TheLiftSystem) runElevator(stops []int, direction string) []int {
	var floorOrder []int
	if direction == "down" {
		for i := len(d.queues) - 1; i >= 0; i-- {
			floorOrder = append(floorOrder, i)
		}
	} else {
		for i := 0; i < len(d.queues); i++ {
			floorOrder = append(floorOrder, i)
		}
	}

	for _, floor := range floorOrder {
		stopToLetPeopleOff := d.peopleGettingOff(floor)
		stopToLetPeopleOn := d.peopleGettingOn(floor, direction)

		if stopToLetPeopleOff || stopToLetPeopleOn && stops[len(stops)-1] != floor {
			stops = append(stops, floor)
		}
	}

	return stops
}

func (d *TheLiftSystem) peopleGettingOff(floor int) bool {
	stopAtFloor := false

	for i := 0; i < len(d.elevator); i++ {
		if d.elevator[i] == floor {
			d.elevator = append(d.elevator[:i], d.elevator[i+1:]...)
			stopAtFloor = true
			i-- // Adjust the index since we removed an element
		}
	}

	return stopAtFloor
}

func (d *TheLiftSystem) peopleGettingOn(floor int, direction string) bool {
	stopAtFloor := false

	for i := 0; i < len(d.queues[floor]); i++ {
		person := d.queues[floor][i]
		personIsGettingOn := (direction == "down" && person < floor) || (direction == "up" && person > floor)

		if personIsGettingOn {
			stopAtFloor = true

			if d.capacity > len(d.elevator) {
				d.elevator = append(d.elevator, person)
				d.queues[floor] = append(d.queues[floor][:i], d.queues[floor][i+1:]...)
				i-- // Adjust the index since we removed an element
			} else {
				d.peopleWaiting = true
			}
		}
	}

	return stopAtFloor
}
