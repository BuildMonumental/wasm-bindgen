use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/instantiate.js")]
extern "C" {
    fn pair_f64_js(p: Pair<f64>) -> Pair<f64>;
    fn pair_i32_js(p: Pair<i32>) -> Pair<i32>;
    fn check_distinct(f: Pair<f64>, i: Pair<i32>);
}

// A single generic struct monomorphized into two concrete JS classes. Each
// instantiation gets its own exported class with the concrete field types
// substituted in. Methods are out of scope here, so instances cross the
// boundary via free functions and have their public fields read/written
// directly from JS.
#[wasm_bindgen(instantiate(Pair<f64> as PairF64, Pair<i32> as PairI32))]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

#[wasm_bindgen]
pub fn make_pair_f64(first: f64, second: f64) -> Pair<f64> {
    Pair { first, second }
}

#[wasm_bindgen]
pub fn make_pair_i32(first: i32, second: i32) -> Pair<i32> {
    Pair { first, second }
}

#[wasm_bindgen]
pub fn pair_f64_into_sum(p: Pair<f64>) -> f64 {
    p.first + p.second
}

#[wasm_bindgen]
pub fn pair_i32_into_sum(p: Pair<i32>) -> i32 {
    p.first + p.second
}

#[wasm_bindgen_test]
fn roundtrip_f64() {
    let p = make_pair_f64(1.0, 2.0);
    // JS asserts the fields, then overwrites them.
    let p = pair_f64_js(p);
    assert_eq!(pair_f64_into_sum(p), 30.0);
}

#[wasm_bindgen_test]
fn roundtrip_i32() {
    let p = make_pair_i32(3, 4);
    let p = pair_i32_js(p);
    assert_eq!(pair_i32_into_sum(p), 70);
}

#[wasm_bindgen_test]
fn instantiations_are_distinct_classes() {
    check_distinct(make_pair_f64(1.0, 2.0), make_pair_i32(1, 2));
}
