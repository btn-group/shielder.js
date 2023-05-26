/* eslint-disable @typescript-eslint/no-var-requires */
// import * as shielderwasm from '../../shielder.js-core/shielder-zk/pkg/shielder_zk';
// eslint-disable-next-line @typescript-eslint/no-var-requires
// const shielderwasm = require('../../shielder.js-core/shielder-zk/pkg/shielder_zk');
// import * as shielderwasm from 'shielder-zk';
const shielderwasm = require('shielder-zk');

export const delayMillis = (delayMs: number): Promise<void> => new Promise(resolve => setTimeout(resolve, delayMs));

export const greet = (name: string): string => `Hello ${name}`

export const foo = async (): Promise<boolean> => {
  console.log(greet('World'))
  await delayMillis(1000)
  console.log('done')
  return true
}

export const bar = () => {
  const test = {
    num: 10,
    arr: [1,2,3,4],
    my_vec: [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]],
    test_json: {
      num: 10,
      word: 'test-typescript'
    }
  }

  const res = shielderwasm.json_test_string(JSON.stringify(test));
  return res;
}
