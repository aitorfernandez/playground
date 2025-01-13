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

type URLObject = { [k: string]: string };

type VideoFormat = {
	format360p: string;
	format480p: string;
	format720p: string;
};

type Subtitle = {
	english: string;
	german: string;
};

type LoadedKey<T> = {
	format: T;
	loaded: boolean;
};

async function isAvailable<T extends URLObject, K extends keyof T>(
	obj: T,
	format: K,
): Promise<LoadedKey<K>> {
	const data = await fetch(obj[format]);
	return {
		format,
		loaded: data.status === 200,
	};
}

const videos: VideoFormat = {
	format360p: "format360",
	format480p: "format480",
	format720p: "format720",
};

(async () => {
	const result = await isAvailable(videos, "format360p");
	if (result.format === "format360p") {
		//
	}
})();
