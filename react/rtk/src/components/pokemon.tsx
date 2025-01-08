import React, { useState } from 'react'
import { useSelector, useDispatch } from 'react-redux'

import { fetchPokemon } from '../features/pokemonSlice'
import type { RootState, AppDispatch } from '../store'

export function Pokemon() {
  const [name, setName] = useState<string>('')

  const dispatch = useDispatch<AppDispatch>()
  const { data, loading, error } = useSelector(
    (state: RootState) => state.pokemon,
  )

  function handleFetch() {
    if (name.trim() !== '') {
      dispatch(fetchPokemon(name.toLowerCase()))
    }
  }

  return (
    <div>
      <h2>Pokemon</h2>
      <input
        type="text"
        placeholder="Enter name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
      <button type="button" onClick={handleFetch} disabled={loading}>
        {loading ? 'Loading...' : 'Fetch Pokémon'}
      </button>
      {error && <p style={{ color: 'red' }}>Error: {error}</p>}
      {data && (
        <div>
          <h3>{data.name.toUpperCase()}</h3>
          <p>Height: {data.height}</p>
          <p>Base Experience: {data.base_experience}</p>
        </div>
      )}
    </div>
  )
}
