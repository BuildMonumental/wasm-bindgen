const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.pair_f64_js = (p) => {
    assert.equal(p.first, 1);
    assert.equal(p.second, 2);
    p.first = 10;
    p.second = 20;
    return p;
};

exports.pair_i32_js = (p) => {
    assert.equal(p.first, 3);
    assert.equal(p.second, 4);
    p.first = 30;
    p.second = 40;
    return p;
};

exports.check_distinct = (f, i) => {
    assert.ok(f instanceof wasm.PairF64);
    assert.ok(i instanceof wasm.PairI32);
    assert.ok(!(f instanceof wasm.PairI32));
    assert.ok(!(i instanceof wasm.PairF64));
    f.free();
    i.free();
};
