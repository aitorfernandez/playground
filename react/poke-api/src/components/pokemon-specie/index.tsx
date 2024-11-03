import { usePokemonSpecie } from '../../api'

export function PokemonSpecie() {
  const { data, isLoading, error } = usePokemonSpecie(100)

  if (isLoading) return <>Loading...</>
  if (error instanceof Error) return <>Error: {error.message}</>

  return (
    <>
      <h2>Pokemon details</h2>
      <div>{`Name: ${data?.name} with id: ${data?.id}`}</div>
    </>
  )
}
