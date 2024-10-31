use serde::Serialize;

use crate::pokemon_specie::PokemonSpecie;

#[derive(Serialize)]
pub struct Pokemon {
    id: u32,
    name: String,
}

impl From<PokemonSpecie> for Pokemon {
    fn from(ps: PokemonSpecie) -> Self {
        let PokemonSpecie { id, name, .. } = ps;

        Self { id, name }
    }
}
