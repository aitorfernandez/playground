interface Data {
	id: number;
}

async function fetchData(id: number): Promise<Data> {
	return new Promise((resolve, reject) => {
		setTimeout(() => {
			if (id === 0) {
				reject(new Error("data doesn't found"));
			} else {
				resolve({ id });
			}
		}, 1000);
	});
}

async function fetch() {
	try {
		const pending_data = [0, 1, 2, 3, 4, 5].map((id) => fetchData(id));
		const data = await Promise.allSettled(pending_data);
		// {
		//     status: 'rejected',
		//     reason: Error: data doesn't found
		//         at Timeout._onTimeout (/Users/aitorfernandez/af/playground/typescript/examples/src/promises.ts:9:12)
		//         at listOnTimeout (node:internal/timers:573:17)
		//         at process.processTimers (node:internal/timers:514:7)
		//   },
		//   { status: 'fulfilled', value: { id: 1 } },
		//   { status: 'fulfilled', value: { id: 2 } },
		//   { status: 'fulfilled', value: { id: 3 } },

		console.log(data);
	} catch (error) {
		console.log(error);
		throw error;
	}
}

fetch();
