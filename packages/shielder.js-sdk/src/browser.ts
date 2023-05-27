import init, {bar, 
  deposit as depositWasm, 
  // withdraw as withdrawWasm
  run_prover
} from 'shielder-zk';

async function initWasm() {
  await init();
}

async function foo() {
  const res = bar();
  console.log('foo', res);
}


interface Deposit {
  deposit_id: number;
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

async function deposit() {
  console.log('deposit');


  console.log(JSON.stringify([[0,0], [0,1], [0,3]]));

  const res = await fetch('https://bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u.ipfs.w3s.link/ipfs/bafybeifjsmmivrq2hxmujww7t2t3prbu3r2vffh7bq7pf6su2v5qgmzd4u/deposit.pk.bytes')
  const chunks = (await readAllChunks(res.body));
  
  let sum = 0;
  const flatten: number[] = [];

  chunks.forEach(chunk => {
    sum += chunk.length;
    flatten.push(...chunk);
  });

  console.log(sum, flatten);

  const dep: Deposit = {
    deposit_id: 0,
    token_id: 0,
    token_amount: 100
  }

  console.log('parsed deposit', JSON.stringify(dep))
  console.log('chunks json', JSON.stringify({
    nested: flatten
  }))
  const depositWasmResult = await depositWasm(JSON.stringify(dep), JSON.stringify({
    nested: flatten
  }));

  console.log({depositWasmResult})
  console.log('finish deposit')
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
  run_prover
}
