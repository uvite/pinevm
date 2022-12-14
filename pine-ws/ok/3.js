
const assert = require('assert');
const Runner = require("./Runner");

const FLScript = `
plot(open)
plot(close)
plot(high)
// plot(close)
// plot(close)
// plot(close)
// plot(close)
// plot(close)
// plot(close)
// plot(close)
// plot(close)
`;

let runner = new Runner();

runner.parse(FLScript);
console.log(runner.genIOInfo().inputs[0]);
//assert.deepEqual(runner.genIOInfo().input_srcs, [{ ticker: null, srcs: ['open', 'close'] }]);

//for (let m = 0; m < 3; m += 1) {
    let farray = new Float64Array(100);
    for (let i = 0; i < 15*3 ; i += 1) {
        farray[i] = i + 100.0;
    }

    let result = runner.runWithData(["close", "open","high"], 10, farray);
    // console.log(result[0].series[0]);
    result.map(d => {
        console.log(d.series)
        //assert.equal(d.series[0].length, 400);
    });
//}