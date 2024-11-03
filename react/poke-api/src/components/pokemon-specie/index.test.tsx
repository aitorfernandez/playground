import { render, screen } from '@testing-library/react'
import { vi } from 'vitest'

import { PokemonSpecie } from '.'
import { usePokemonSpecie } from '../../api'

vi.mock('../../api', () => ({
  usePokemonSpecie: vi.fn(),
}))

describe('PokemonSpecie Component', () => {
  it('renders loading state', () => {
    ;(usePokemonSpecie as jest.Mock).mockReturnValue({
      data: null,
      isLoading: true,
      error: null,
    })

    render(<PokemonSpecie />)

    expect(screen.getByText(/loading.../i)).toBeInTheDocument()
  })

  it('renders error state', () => {
    ;(usePokemonSpecie as jest.Mock).mockReturnValue({
      data: null,
      isLoading: false,
      error: new Error('Failed to fetch data'),
    })

    render(<PokemonSpecie />)

    expect(screen.getByText(/error: failed to fetch data/i)).toBeInTheDocument()
  })

  it('renders pokemon details successfully', () => {
    // Mock success state
    ;(usePokemonSpecie as jest.Mock).mockReturnValue({
      data: { name: 'Pikachu', id: 100 },
      isLoading: false,
      error: null,
    })

    render(<PokemonSpecie />)

    expect(screen.getByText(/pokemon details/i)).toBeInTheDocument()
    expect(screen.getByText(/name: pikachu with id: 100/i)).toBeInTheDocument()
  })
})
