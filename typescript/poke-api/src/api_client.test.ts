import { describe, it, expect, beforeEach, afterEach } from "vitest";
import axios from "axios";
import axiosMockAdapter from "axios-mock-adapter";

import { ApiClient } from "./api_client";

describe("ApiClient", () => {
	let mock: axiosMockAdapter;
	const apiClient = ApiClient.getInstance();

	beforeEach(() => {
		mock = new axiosMockAdapter(axios);
	});

	afterEach(() => {
		mock.reset();
		mock.restore();
	});

	it("should fetch data successfully", async () => {
		const url = "https://pokeapi.co/api/v2/pokemon-species/1";
		const mockResponse = { id: 1, name: "bulbasur" };

		mock.onGet(url).reply(200, mockResponse);

		const data = await apiClient.sendGet<typeof mockResponse>(url);
		expect(data).toEqual(mockResponse);
	});

	it("should send data by post successfully", async () => {
		const url = "https://example.com/api/users";
		const body = { name: "Foo", email: "foo@foo.com" };
		const mockResponse = { id: 1, name: "Foo", email: "foo@foo.com" };

		mock.onPost(url, body).reply(201, mockResponse);

		const data = await apiClient.sendPost<typeof body, typeof mockResponse>(
			url,
			body,
		);
		expect(data).toEqual(mockResponse);
	});
});
