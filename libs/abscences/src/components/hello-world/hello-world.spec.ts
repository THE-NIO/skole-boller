import HelloWorld from './HelloWorld.svelte'
import { render } from '@testing-library/svelte'

it('it works', async () => {
  const { getByText } = render(HelloWorld)

  expect(getByText('Hello component!'));
})
