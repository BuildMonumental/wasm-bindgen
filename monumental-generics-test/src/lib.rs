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

// ---------------------------------------------------------------------------
// Typestate replica of mech's build-plan node family (EvalMode GATs):
// generic nodes carrying `()` (input) vs `Evaluated` (debug) state, with
// uniform auto field getters plus per-instantiation concrete impl blocks.

pub trait EvalMode: core::fmt::Debug + Default {
    type Path: Default + Clone + core::fmt::Debug;
    type Evaluation<R: Clone + core::fmt::Debug>: Default + Clone + core::fmt::Debug;
}

#[derive(Default, Clone, Debug)]
pub struct Evaluated;

impl EvalMode for () {
    type Path = ();
    type Evaluation<R: Clone + core::fmt::Debug> = ();
}

impl EvalMode for Evaluated {
    type Path = String;
    type Evaluation<R: Clone + core::fmt::Debug> = Option<R>;
}

#[wasm_bindgen(getter_with_clone, instantiate(Check<()> as CheckInput, Check<Evaluated> as CheckDebug))]
#[derive(Clone, Debug)]
pub struct Check<D: EvalMode> {
    pub name: String,
    #[wasm_bindgen(skip)]
    pub path: D::Path,
    #[wasm_bindgen(skip)]
    pub evaluation: D::Evaluation<f64>,
}

#[wasm_bindgen(getter_with_clone, instantiate(Plan<()> as PlanInput, Plan<Evaluated> as PlanDebug))]
#[derive(Clone, Debug)]
pub struct Plan<D: EvalMode> {
    pub meta: String,
    pub check: Check<D>,
    pub fallback: Option<Check<D>>,
    #[wasm_bindgen(skip)]
    pub path: D::Path,
    #[wasm_bindgen(skip)]
    pub evaluation: D::Evaluation<u32>,
}

#[wasm_bindgen(js_class = "CheckInput")]
impl Check<()> {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        Check { name, path: (), evaluation: () }
    }

    pub fn described(mut self, suffix: String) -> Self {
        self.name.push_str(&suffix);
        self
    }
}

#[wasm_bindgen(js_class = "CheckDebug")]
impl Check<Evaluated> {
    #[wasm_bindgen(getter)]
    pub fn path_key(&self) -> String {
        self.path.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn result(&self) -> Option<f64> {
        self.evaluation
    }
}

#[wasm_bindgen(js_class = "PlanInput")]
impl Plan<()> {
    #[wasm_bindgen(constructor)]
    pub fn new(meta: String, check: Check<()>, fallback: Option<Check<()>>) -> Self {
        Plan { meta, check, fallback, path: (), evaluation: () }
    }

    pub fn evaluate(&self) -> Plan<Evaluated> {
        Plan {
            meta: self.meta.clone(),
            check: Check {
                name: self.check.name.clone(),
                path: "0".to_string(),
                evaluation: Some(self.check.name.len() as f64),
            },
            fallback: self.fallback.as_ref().map(|f| Check {
                name: f.name.clone(),
                path: "1".to_string(),
                evaluation: None,
            }),
            path: "root".to_string(),
            evaluation: Some(self.meta.len() as u32),
        }
    }
}

#[wasm_bindgen(js_class = "PlanDebug")]
impl Plan<Evaluated> {
    #[wasm_bindgen(getter)]
    pub fn path_key(&self) -> String {
        self.path.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn result(&self) -> Option<u32> {
        self.evaluation
    }

    pub fn resolve(&self, path: String) -> JsValue {
        match path.as_str() {
            "0" => self.check.clone().into(),
            "1" => self
                .fallback
                .clone()
                .map(JsValue::from)
                .unwrap_or(JsValue::NULL),
            _ => JsValue::NULL,
        }
    }
}
