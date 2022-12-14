const Runner = require("./Runner");


// let runner = new Runner();
// runner.parse('plot(volume)');
// // console.log(runner.genIOInfo().inputs[0]);
// console.log(runner.genIOInfo().input_srcs);
//
// //assert.deepEqual(runner.genIOInfo().input_srcs, [{ ticker: null, srcs: ['volume'] }]);
// let result = runner.runWithData(["volume"], 2, new Float64Array([10,13]));
// console.log(result[0].series);
// //assert.deepEqual(result[0].series, [new Float64Array([10.0])]);



const Script = `
//@version=4
study(title="TRIX", shorttitle="TRIX", format=format.price, precision=2)
length = input(2, minval=1)
out = sma(close,length)
plot(out, color=color.maroon, title="TRIX")
`;


    let runner = new Runner();
    runner.parse(Script);
    // console.log(runner.genIOInfo().inputs[0]);
    console.log(runner.genIOInfo().input_srcs);
    //assert.deepEqual(runner.genIOInfo().input_srcs, [{ ticker: null, srcs: ['close'] }]);
    let farray = new Float64Array(100);
    for (let i = 0; i < 10; i += 1) {
        farray[i] = i;
    }
    // 0,1,2,3,4
    let result = runner.runWithData(["close"], 5, farray);
    console.log(result[0].series[0]);
   // assert(!isNaN(result[0].series[0][0]));
    // assert.deepEqual(result[0].series, [new Float64Array([10.0])]);

