let globalString!: string;

setGlobalString("this string is set");

console.log(`globalString = ${globalString}`);

function setGlobalString(value: string) {
	globalString = value;
}
