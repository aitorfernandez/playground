type AllowedStringValues = "one" | "two" | "three";

type AllowedNumericValues = 1 | 2 | 3;

function withLiteral(input: AllowedStringValues | AllowedNumericValues) {
	console.log(`called with: ${input}`);
}
