const assert = require('assert');
const Runner = require("./Runner");


    let runner = new Runner();
    runner.parse("m = input(1, 'hello', 'int')\nplot(close + m)");
    let ioInfo = runner.genIOInfo();
    console.log("io info:", JSON.stringify(ioInfo));
    assert.equal(ioInfo.inputs.length, 1);
    assert.deepEqual(ioInfo.input_srcs, [{ "ticker": null, "srcs": ["close"] }]);

    let output = runner.runWithData(["close", "open", "high", "low"], 1, new Float64Array([10, 90, 0, 0]));

    console.log(output[0].series[0][0])
    assert.equal(output.length, 1);
    assert.deepEqual(output[0].series, [new Float64Array([11.0])]);
    // assert.deepEqual(Array.from(output[0].series), [11.0]);

    output = runner.runWithInput([{ type: "Int", content: 100 }]);
    assert.deepEqual(output[0].series, [new Float64Array([110.0])]);
    console.log(output[0]);
    output = runner.run(
        [{
            type: "Int",
            content: 200
        }],
        ["close", "open", "high", "low"], 1, new Float64Array([10, 90, 0, 0]));
    assert.deepEqual(output[0].series, [new Float64Array([210.0])]);
console.log(output[0].series);
    output = runner.update(["close", "open", "high", "low"], 1, new Float64Array([1, 2, 3, 4]));
    assert.deepEqual(output[0].series, [new Float64Array([201.0])]);

    output = runner.updateFrom(
        ["close", "open", "high", "low"], 0, 1, new Float64Array([10, 2, 3, 4]),
    );
    assert.deepEqual(output[0].series, [new Float64Array([210.0])]);
