import axios from 'axios'
import { useQuery } from '@tanstack/react-query'

const apiClient = axios.create({
  baseURL: 'http://localhost:8080',
  headers: {
    'Content-Type': 'application/json',
  },
})

type PokemonSpecie = {
  id: number
  name: string
}

async function fetchPokemonSpecie(
  id_or_name: number | string
): Promise<PokemonSpecie> {
  const response = await apiClient.get(`/pokemon-species/${id_or_name}`)
  return response.data
}

export function usePokemonSpecie(id_or_name: number | string) {
  return useQuery({
    queryKey: ['pokemon-specie', id_or_name],
    queryFn: () => fetchPokemonSpecie(id_or_name),
    enabled: !!id_or_name,
  })
}
