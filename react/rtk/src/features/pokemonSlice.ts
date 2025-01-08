import {
  createSlice,
  createAsyncThunk,
  type PayloadAction,
} from '@reduxjs/toolkit'

interface PokemonState {
  data: PokemonData | null
  loading: boolean
  error: string | null
}

interface PokemonData {
  name: string
  height: string
  base_experience: number
}

const initialState: PokemonState = {
  data: null,
  loading: false,
  error: null,
}

export const fetchPokemon = createAsyncThunk<PokemonData, string>(
  'pokemon/fetchPokemon',
  async (name: string, thunkAPI) => {
    const response = await fetch(`https://pokeapi.co/api/v2/pokemon/${name}`)
    if (!response.ok) {
      throw new Error('Failed to fetch Pokemon data')
    }
    return response.json()
  },
)

const pokemonSlice = createSlice({
  name: 'pokemon',
  initialState,
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(fetchPokemon.pending, (state) => {
        console.log('pending')
        state.loading = true
      })
      .addCase(
        fetchPokemon.fulfilled,
        (state, action: PayloadAction<PokemonData>) => {
          console.log('fulfilled')
          state.data = action.payload
          state.loading = false
        },
      )
      .addCase(fetchPokemon.rejected, (state, action) => {
        console.log('rejected', state, action)
        state.loading = false
        state.error = action.error.message || 'Something went wrong'
      })
  },
})

export default pokemonSlice.reducer
