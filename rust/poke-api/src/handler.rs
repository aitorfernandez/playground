use crate::{errors::PokemonError, pokemon::Pokemon, pokemon_specie::PokemonSpecie};
use actix_web::{
    get,
    web::{Path, ServiceConfig},
    HttpResponse,
};

#[get("pokemon-species/{id_or_name}")]
async fn pokemon_species(id_or_name: Path<(String,)>) -> Result<HttpResponse, PokemonError> {
    let (id_or_name,) = id_or_name.into_inner();
    let pokemon_specie = PokemonSpecie::get(&id_or_name, None).await?;

    println!("{pokemon_specie:?}");

    Ok(HttpResponse::Ok().json(Pokemon::from(pokemon_specie)))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(pokemon_species);
}
