import axios from "axios";

import { ApiClientError } from "./errors";

export class ApiClient {
	private static instance: ApiClient;

	private constructor() {}

	static getInstance(): ApiClient {
		if (!ApiClient.instance) {
			ApiClient.instance = new ApiClient();
		}

		return ApiClient.instance;
	}

	async sendGet<T>(url: string): Promise<T> {
		try {
			const response = await axios.get<T>(url);
			return response.data;
		} catch (error: any) {
			if (axios.isAxiosError(error)) {
				throw new ApiClientError(`Request failed ${error.response?.status}`);
			}

			throw new ApiClientError(error.message || "Request failed");
		}
	}
}
