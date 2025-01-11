import type { PokemonSpecie } from "./pokemon_specie";

interface Pokemon {
	id: string;
	name: string;
	flavorText: string[];
}

export function transformPokemon(pokemonSpecie: PokemonSpecie): Pokemon {
	const flavorTextEntries = pokemonSpecie.flavorTextEntries.filter(
		(entry) => entry.language.name === "en",
	);

	const flavorText: string[] = [];
	for (const flavor of flavorTextEntries) {
		flavorText.push(flavor.flavorText);
	}

	const pokemon = {
		id: pokemonSpecie.id,
		name: pokemonSpecie.name,
		flavorText,
	};

	return pokemon;
}
