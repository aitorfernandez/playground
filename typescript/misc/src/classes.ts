interface Print {
	print(): void;
}

class SimpleClass implements Print {
	private _id: number | undefined;

	readonly name: string; // only in the constructor

	static count = 0;

	constructor(id: number, _name: string) {
		this._id = id;
		this.name = _name;
	}

	get id(): number | undefined {
		return this._id;
	}

	set id(value: number) {
		this._id = value;
	}

	print(): void {
		console.log(`SimpleClass ${this.id} ${this.name} ${SimpleClass.count}`);
	}
}

const myClass = new SimpleClass(100, "Yo!");
myClass.id = 200;

SimpleClass.count++;

myClass.print();
