type Currency = "USD" | "GBP" | "EUR";

export type Transaction = {
	id: string;
	type: "deposit" | "withdrawal";
	amount: number;
	currency: Currency;
};

type Summary = {
	totalDeposits: number;
	totalWithdrawals: number;
	netBalance: number;
};

const exchangeRates: Record<Currency, number> = {
	USD: 1,
	GBP: 1.3,
	EUR: 1.1,
};

export function calculateSummary(
	transactions: Transaction[],
	baseCurrency: Currency = "USD",
): Summary {
	return transactions.reduce<Summary>(
		(summary, tx) => {
			const rate = exchangeRates[tx.currency] / exchangeRates[baseCurrency];
			const amount = tx.amount / rate;

			if (tx.type === "deposit") {
				summary.totalDeposits += amount;
			} else if (tx.type === "withdrawal") {
				summary.totalWithdrawals += amount;
			}

			summary.netBalance = summary.totalDeposits - summary.totalWithdrawals;
			return summary;
		},
		{
			totalDeposits: 0,
			totalWithdrawals: 0,
			netBalance: 0,
		},
	);
}
