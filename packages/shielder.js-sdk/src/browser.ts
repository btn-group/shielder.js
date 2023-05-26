import init, { json_test_string} from 'shielder-zk';

async function initWasm() {
  await init();
}

async function testMethod() {
  const test = {
    num: 10,
    arr: [1,2,3,4],
    my_vec: [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]],
    test_json: {
      num: 10,
      word: 'test-typescript'
    }
  }

  const result = json_test_string(JSON.stringify(test));
  console.log(result);
}

export {
  initWasm,
  testMethod
}
