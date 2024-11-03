import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

import { PokemonSpecie } from './components/pokemon-specie'

const client = new QueryClient()

export function App() {
  return (
    <QueryClientProvider {...{ client }}>
      <PokemonSpecie />
    </QueryClientProvider>
  )
}
