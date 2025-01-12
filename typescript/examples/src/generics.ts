function printGeneric<T>(value: T) {
	console.log(`Generic is ${typeof value}`);
}

printGeneric(1);
printGeneric("test");
printGeneric(true);
printGeneric(() => {});
printGeneric({ id: 1 });

function usingTwoTypes<A, B>(first: A, second: B) {
	//...
}

usingTwoTypes<string, number>("a", 2);

interface PrintId {
	id: number;
	print(): void;
}

interface PrintName {
	name: string;
	print(): void;
}

function use<T extends PrintId | PrintName>(value: T) {
	value.print();
}

function printProperty<T, K extends keyof T>(o: T, k: K) {
	const property = o[k];
	console.log(`object[${String(k)}] = ${property}`);
}

const obj1 = {
	id: 1,
	name: "myName",
	print() {
		console.log(`${this.id}`);
	},
};

printProperty(obj1, "id");
