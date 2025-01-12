import type { Application, Request, Response } from "express";

import { getPokemonSpecie } from "./pokemon_specie";
import { transformPokemon } from "./pokemon";

export function config(app: Application) {
	app.get("/pokemon-species/:idOrName", async (req: Request, res: Response) => {
		try {
			const idOrName = req.params.idOrName;
			const pokemonSpecie = await getPokemonSpecie(idOrName);

			res.json(transformPokemon(pokemonSpecie));
		} catch (error: any) {
			console.error(error);
			res.status(500).json({ error: error.message });
		}
	});

	app.post("/pokemon", async (req: Request, res: Response) => {
		try {
			// const body = req.body;
			// const newPokemon = await createPokemon(body);
		} catch (error: any) {
			//
		}
	});
}
