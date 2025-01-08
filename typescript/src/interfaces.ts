interface Person {
	id: number;
	name: string;
}

type PersonProperty = keyof Person;

interface Value {
	descr: string;
}

function printPersonOrValue(obj: Person | Value): void {
	if ("id" in obj) {
		console.log(`obj.id ${obj.id}`);
	}

	if ("descr" in obj) {
		console.log(`obj.descr ${obj.descr}`);
	}
}

printPersonOrValue({ id: 1, name: "Yo!" });
printPersonOrValue({ descr: "description" });

function printProperty(key: PersonProperty, person: Person) {
	console.log(`${key} = ${person[key]}`);
}

printProperty("id", { id: 1, name: "Yo!" });
