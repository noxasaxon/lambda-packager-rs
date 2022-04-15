import path from 'path'

import test from 'ava'

// import { plus100 } from '../index'

// test('sync function from native code', (t) => {
//   const fixture = 42
//   t.is(plus100(fixture), fixture + 100)
// })

import { readFileAsync } from '../index'

test('async function from native code', async (t) => {
  const thisFilePath = path.join(__dirname, 'index.spec.ts')

  const fixture = await readFileAsync(thisFilePath)

  t.assert(fixture.includes('thisphraserighthere'))
})
