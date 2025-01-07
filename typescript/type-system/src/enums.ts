enum DoorState {
	Open = 1,
	Closed = 2,
}

function checkDoorState(state: DoorState) {
	switch (state) {
		case DoorState.Open:
			console.log(`Door Open = ${state}`);
			break;
		case DoorState.Closed:
			console.log(`Door Closed = ${state}`);
			break;
	}
}

checkDoorState(DoorState.Closed);
