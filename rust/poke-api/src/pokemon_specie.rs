use crate::{api_client::ApiClient, errors::PokemonError};
use serde::{Deserialize, Serialize};

const PATH: &str = "/api/v2/pokemon-species";
const BASE_PATH: &str = "https://pokeapi.co";

#[derive(Debug, Deserialize, Serialize)]
struct Language {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FlavorTextEntries {
    flavor_text: String,
    language: Language,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PokemonSpecie {
    pub id: u32,
    pub name: String,
    flavor_text_entries: Vec<FlavorTextEntries>,
}

impl PokemonSpecie {
    pub async fn get(
        id_or_name: &str,
        base_url: Option<&str>,
    ) -> Result<PokemonSpecie, PokemonError> {
        let url = match base_url {
            Some(base) => format!("{base}/{PATH}/{id_or_name}"),
            None => format!("{BASE_PATH}/{PATH}/{id_or_name}"),
        };
        let pokemon_specie_value = ApiClient::new().send_get(&url).await?;
        let pokemon_specie = serde_json::from_value(pokemon_specie_value)?;

        Ok(pokemon_specie)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_pokemon_specie_get() {
        // Arrange
        let response = r#"{
            "id": 1,
            "name": "test"
        }"#;

        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.path(format!("{PATH}/1"));
            then.status(200)
                .header("content-type", "application/json")
                .body(response);
        });

        // Act
        let res = PokemonSpecie::get("1", Some(&server.url("")))
            .await
            .unwrap();

        // Assert
        mock.assert();

        let PokemonSpecie { id, name, .. } = res;
        assert_eq!(id, 1);
        assert_eq!(name, "test".to_string());
    }
}
