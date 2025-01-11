import { test, expect } from "vitest";

import { greet } from "./example";

test("greet return correct message", () => {
	expect(greet("World")).toBe("Hello, World");
});
