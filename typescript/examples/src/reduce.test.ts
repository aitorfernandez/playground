import { describe, it, expect } from "vitest";

import { calculateSummary, type Transaction } from "./reduce";

describe("calculateSummary", () => {
	it("should calculate total deposits in the specific base currency", () => {
		// Arrange
		const transactions: Transaction[] = [
			{
				id: "1",
				type: "deposit",
				amount: 100,
				currency: "USD",
			},
			{
				id: "2",
				type: "deposit",
				amount: 50,
				currency: "GBP",
			},
			{
				id: "3",
				type: "withdrawal",
				amount: 50,
				currency: "EUR",
			},
		];
		// Act
		const summary = calculateSummary(transactions, "USD");
		// Assert
		expect(summary.totalDeposits).toBeCloseTo(138.46); // 100 + (50/1.3)
		expect(summary.totalWithdrawals).toBeCloseTo(45.45); // 50 / 1.1
		expect(summary.netBalance).toBeCloseTo(93.0069); // deposits - withdrawals
	});
});
