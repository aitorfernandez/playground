function testArguments(...args: number[]) {
	for (let i = 0; i < args.length; i++) {
		console.log(`argument[${i}] = ${args[i]}`);
	}
}

testArguments(1, 2);
