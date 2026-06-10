use wasm_bindgen::prelude::*;

#[wasm_bindgen(instantiate(Holder<f64> as HolderF64, Holder<String> as HolderString, Holder<JsValue> as HolderJs))]
pub struct Holder<P> {
    inner: P,
    pub count: u32,
}

#[wasm_bindgen(instantiate(Holder<f64> as HolderF64, Holder<String> as HolderString, Holder<JsValue> as HolderJs))]
impl<P: Clone> Holder<P> {
    #[wasm_bindgen(constructor)]
    pub fn new(inner: P, count: u32) -> Self {
        Holder { inner, count }
    }

    pub fn with_count(inner: P, count: u32) -> Holder<P> {
        Holder { inner, count }
    }

    pub fn get(&self) -> P {
        self.inner.clone()
    }

    pub fn set(&mut self, value: P) {
        self.inner = value;
    }

    pub fn duplicate(&self) -> Holder<P> {
        Holder { inner: self.inner.clone(), count: self.count * 2 }
    }

    pub fn halved(&self) -> Option<Holder<P>> {
        let halved_count = self.count.checked_div(2).filter(|c| *c > 0)?;
        Some(Holder { inner: self.inner.clone(), count: halved_count })
    }

    pub fn split(&self) -> Vec<Holder<P>> {
        (0..self.count)
            .map(|_| Holder { inner: self.inner.clone(), count: 1 })
            .collect()
    }
}

#[wasm_bindgen(instantiate(Pair<String, f64> as NamedScore))]
pub struct Pair<A, B> {
    #[wasm_bindgen(getter_with_clone)]
    pub first: A,
    pub second: B,
}

#[wasm_bindgen(instantiate(Pair<String, f64> as NamedScore))]
impl<A: Clone, B: Clone> Pair<A, B> {
    #[wasm_bindgen(constructor)]
    pub fn new(first: A, second: B) -> Self {
        Pair { first, second }
    }

    pub fn label(&self) -> A {
        self.first.clone()
    }
}
