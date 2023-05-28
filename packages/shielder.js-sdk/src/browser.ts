import init, {
  deposit as depositWasm,
  withdraw as withdrawWasm
} from 'shielder-zk';
import { Deposit, Withdraw } from './interfaces';
import { SHIELDER_DEPOSIT_PK_BYTES_URL, SHIELDER_WITHDRAW_PK_BYTES_URL } from './constants';

async function initWasm() {
  await init();
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

async function deposit(args: Deposit) {
  console.log('[SDK_DEBUG] START_DEPOSIT_SDK');

  const res = await fetch(SHIELDER_DEPOSIT_PK_BYTES_URL);
  const chunks = (await readAllChunks(res.body));

  const flatten = chunks.reduce((acc, curr) => {
    return [...acc, ...curr]
  })

  console.log('[SDK_DEBUG] DEPOSIT_SDK_DATA:', JSON.stringify(args))
  const depositWasmResult = await depositWasm(JSON.stringify(args), JSON.stringify({
    nested: Array.from(flatten)
  }));
  
  console.log('[SDK_DEBUG] FINISH_DEPOSIT_SDK:', depositWasmResult);

  return depositWasmResult;
}

async function withdraw(args: Withdraw) {
  console.log('[SDK_DEBUG] START_WITHDRAW_SDK');

  const res = await fetch(SHIELDER_WITHDRAW_PK_BYTES_URL);
  const chunks = (await readAllChunks(res.body));

  const flatten = chunks.reduce((acc, curr) => {
    return [...acc, ...curr]
  });

  console.log('chunks', flatten.length);

  console.log('[SDK_DEBUG] WITHDRAW_SDK_DATA:', JSON.stringify(args));
  const withdrawWasmResult = await withdrawWasm(JSON.stringify(args), JSON.stringify({
    nested: Array.from(flatten)
  }));
  
  console.log('[SDK_DEBUG] FINISH_WITHDRAW_SDK:', withdrawWasmResult);

  return withdrawWasmResult;
}

export {
  initWasm,
  deposit,
  withdraw,
  Deposit,
  Withdraw
}
