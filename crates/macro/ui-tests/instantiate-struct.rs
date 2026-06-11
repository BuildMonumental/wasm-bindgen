use wasm_bindgen::prelude::*;

// `instantiate` on a struct with no generic parameters.
#[wasm_bindgen(instantiate(Concrete<f64> as ConcreteF64))]
struct Concrete {
    pub value: f64,
}

// Two instantiations sharing the same JS name.
#[wasm_bindgen(instantiate(Dup<f64> as Same, Dup<u32> as Same))]
struct Dup<T> {
    pub value: T,
}

// Wrong number of generic arguments for the struct's parameter list.
#[wasm_bindgen(instantiate(Two<f64> as TwoF64))]
struct Two<T, U> {
    pub a: T,
    pub b: U,
}

// Const generic parameters cannot be monomorphized through `instantiate`.
#[wasm_bindgen(instantiate(Arr<1> as Arr1))]
struct Arr<const N: usize> {
    pub len: u32,
}

// `js_name` cannot be combined with `instantiate`.
#[wasm_bindgen(js_name = "Renamed", instantiate(Named<f64> as NamedF64))]
struct Named<T> {
    pub value: T,
}

// The instantiation must name the struct being instantiated.
#[wasm_bindgen(instantiate(Other<f64> as WrongF64))]
struct Wrong<T> {
    pub value: T,
}

// The JS name cannot be a JS reserved word.
#[wasm_bindgen(instantiate(Keyword<f64> as class))]
struct Keyword<T> {
    pub value: T,
}

fn main() {}
