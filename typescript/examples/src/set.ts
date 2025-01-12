namespace MySet {
	const books: Set<string> = new Set();
	books.add("A Dance With Dragons");

	if (!books.has("The Winds of Winter")) {
		console.log(
			`We have ${books.size} books, but The Winds of Winter ain't one`,
		);
	}

	books.add("The Odyssey");
	books.delete("The Odyssey");

	for (const book of books) {
		console.log(book);
	}

	function getOrInsert(set: Set<string>, value: string): string {
		if (!set.has(value)) {
			set.add(value);
		}
		return value;
	}
}
