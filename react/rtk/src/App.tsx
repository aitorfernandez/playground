import { Provider } from 'react-redux'

import { store } from './store'
import { Pokemon } from './components/pokemon'

export function App() {
  return (
    <Provider store={store}>
      <Pokemon />
    </Provider>
  )
}
