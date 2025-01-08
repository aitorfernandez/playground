function testNullOperands(a: number, b: number | null | undefined): number {
	return a + (b ?? 0);
}
