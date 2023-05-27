import init, {
  bar,
  deposit as depositWasm,
  // withdraw as withdrawWasm
} from 'shielder-zk';

async function initWasm() {
  await init();
}

async function foo() {
  const res = bar();
  console.log('foo', res);
}


interface Deposit {
  token_id: number;
  token_amount: number;
  lead_idx?: number;
  trapdoor?: [number, number, number, number];
  nullifier?: [number, number, number, number];
  note?: [number, number, number, number];
  proof?: string
}

async function readAllChunks(readableStream: any) {
  const reader = readableStream.getReader();
  const chunks = [];

  let done, value;
  while (!done) {
    ({ value, done } = await reader.read());
    if (done) {
      return chunks;
    }
    chunks.push(value);
  }

  return chunks;
}

// function flatten(arr: any) {
//   return arr.reduce(function (flat: any, toFlatten: any) {
//     return flat.concat(Array.isArray(toFlatten) ? flatten(toFlatten) : toFlatten);
//   }, []);
// }

async function deposit(args: Deposit) {
  console.log('deposit');


  console.log(JSON.stringify([[0, 0], [0, 1], [0, 3]]));

  const res = await fetch('https://bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u.ipfs.w3s.link/ipfs/bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u/deposit.pk.bytes')
  const chunks = (await readAllChunks(res.body));
  
  // const flatten: number[] = [];

  // chunks.forEach(chunk => {
  //   sum += chunk.length;
  //   flatten.push(...chunk);
  // });
  const flatten = chunks.reduce((acc, curr) => {
    return [...acc, ...curr]
  })

  // for (let index = 0; index < chunks.length; index++) {
  //   const element = chunks[index];
  //   flatten.push(...element)
  // }

  console.log('parsed deposit', JSON.stringify(args))
  const depositWasmResult = await depositWasm(JSON.stringify(args), JSON.stringify({
    nested: flatten
  }));

  console.log({ depositWasmResult })
  console.log('finish deposit')

  return depositWasmResult;
}

// async function testMethod() {
//   const test = {
//     num: 10,
//     arr: [1,2,3,4],
//     my_vec: [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]],
//     test_json: {
//       num: 10,
//       word: 'test-typescript'
//     }
//   }

//   const result = json_test_string(JSON.stringify(test));
//   console.log(result);
// }

export {
  initWasm,
  // testMethod
  foo,
  deposit,
  Deposit
}
