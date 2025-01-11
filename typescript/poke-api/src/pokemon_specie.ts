import { ApiClient } from "./api_client";

const BASE_PATH = "https://pokeapi.co";
const PATH = "/api/v2/pokemon-species";

interface Language {
	name: string;
	url: string;
}

interface FlavorTextEntries {
	flavorText: string;
	language: Language;
}

export interface PokemonSpecie {
	id: string;
	name: string;
	flavorTextEntries: FlavorTextEntries[];
}

export async function getPokemonSpecie(
	idOrName: string,
): Promise<PokemonSpecie> {
	const url = `${BASE_PATH}${PATH}/${idOrName}`;
	const apiClient = ApiClient.getInstance();

	const res = await apiClient.sendGet<PokemonSpecie>(url);

	const pokemonSpecie = {
		id: res.id,
		name: res.name,
		flavorTextEntries: res.flavor_text_entries.map((entry: any) => ({
			flavorText: entry.flavor_text,
			language: {
				name: entry.language.name,
				url: entry.language.url,
			},
		})),
	};

	return pokemonSpecie;
}
