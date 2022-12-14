const Runner = require("./Runner");
const RSIScript = `
src = close
len = 14
m = atr(len)
plot(m)
`;


    let runner = new Runner();
    runner.parse(RSIScript);
    // runner.genIOInfo();

    let farray = new Float64Array(14*3);
    for (let i = 0; i < 14*3; i += 1) {
        farray[i] = i+2 ;
    }
    // for (let i = 0; i < 200; i += 1) {
    //     farray[i] = NaN;
    // }
console.log(farray)
    let result = runner.runWithData(["close","high","low"], 14, farray);
    console.log("result", result[0].series[0]);
