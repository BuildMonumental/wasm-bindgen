# `instantiate`

By default, generic structs cannot be exported to JavaScript: `wasm-bindgen`
generates a single concrete class per struct, and there is no way to represent
an un-monomorphized type parameter on the JS side.

The `instantiate` attribute lets you export a generic struct by listing the
concrete monomorphizations you want. Each entry is written as `Type<Args> as
JsName` and produces a separate JavaScript class:

```rust
#[wasm_bindgen(instantiate(Pair<f64> as PairF64, Pair<u32> as PairU32))]
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}
```

This exports two independent classes, `PairF64` and `PairU32`, each with its
type parameter substituted throughout the struct's fields:

```ts
export class PairF64 {
    free(): void;
    first: number;
    second: number;
}

export class PairU32 {
    free(): void;
    first: number;
    second: number;
}
```

Because each instantiation is a distinct concrete Rust type (`Pair<f64>` and
`Pair<u32>`), the two classes do not share an identity in JavaScript — an
instance of one is never an instance of the other.

## Producing and consuming instances

Functions that work with an instantiation refer to the concrete Rust type. A
function returning `Pair<f64>` is typed as returning `PairF64`, and one taking
`Pair<u32>` accepts a `PairU32`:

```rust
#[wasm_bindgen]
pub fn make_pair_f64(first: f64, second: f64) -> Pair<f64> {
    Pair { first, second }
}

#[wasm_bindgen]
pub fn sum_pair_u32(pair: Pair<u32>) -> u32 {
    pair.first + pair.second
}
```

Public fields are accessible from JavaScript exactly as they are for a
non-generic exported struct, with the field type fixed to the concrete type of
that instantiation.

## Rules

* Only plain type parameters can be substituted. Lifetimes and `const`
  parameters are not supported.
* Each instantiation must supply the right number of type arguments for the
  struct's parameters, and each must declare a unique JS class name.
* `js_name` cannot be combined with `instantiate`, since each instantiation
  already names the class it produces.
