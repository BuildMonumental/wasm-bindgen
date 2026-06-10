import assert from "node:assert";
import { createRequire } from "node:module";
const require = createRequire(import.meta.url);
const { HolderF64, HolderString, HolderJs, NamedScore } = require("./pkg/wbg_generics_test.js");

const hf = new HolderF64(2.5, 4);
assert.strictEqual(hf.get(), 2.5);
hf.set(7.25);
assert.strictEqual(hf.get(), 7.25);
assert.ok(hf.duplicate() instanceof HolderF64);
assert.strictEqual(hf.duplicate().count, 8);
assert.strictEqual(hf.halved().count, 2);
assert.strictEqual(new HolderF64(1, 1).halved(), undefined);
const parts = hf.split();
assert.strictEqual(parts.length, 4);
assert.ok(parts.every((p) => p instanceof HolderF64 && p.get() === 7.25 && p.count === 1));
const stat = HolderF64.with_count(3.5, 9);
assert.ok(stat instanceof HolderF64);
assert.strictEqual(stat.count, 9);

const hs = new HolderString("brick", 2);
assert.strictEqual(hs.get(), "brick");
assert.ok(hs.split().every((p) => p instanceof HolderString && p.get() === "brick"));

const obj = { kind: "wall", id: 42 };
const hj = new HolderJs(obj, 1);
assert.strictEqual(hj.get(), obj);
assert.strictEqual(hj.get().id, 42);
hj.set([1, 2, 3]);
assert.deepStrictEqual(hj.get(), [1, 2, 3]);
assert.ok(hj.duplicate() instanceof HolderJs);

const ns = new NamedScore("petra", 99.5);
assert.strictEqual(ns.first, "petra");
assert.strictEqual(ns.second, 99.5);
assert.strictEqual(ns.label(), "petra");
ns.first = "pisa";
assert.strictEqual(ns.label(), "pisa");

console.log("all extended runtime assertions passed");

// --- typestate (build-plan node family replica) ---
const { CheckInput, CheckDebug, PlanInput, PlanDebug } = require("./pkg/wbg_generics_test.js");

const check = new CheckInput("reach").described("-front");
assert.strictEqual(check.name, "reach-front");
const plan = new PlanInput("wall-3", check, new CheckInput("collision"));
assert.strictEqual(plan.meta, "wall-3");
assert.ok(plan.check instanceof CheckInput);
assert.strictEqual(plan.fallback.name, "collision");

const dbg = plan.evaluate();
assert.ok(dbg instanceof PlanDebug);
assert.strictEqual(dbg.path_key, "root");
assert.strictEqual(dbg.result, 6);
assert.ok(dbg.check instanceof CheckDebug);
assert.strictEqual(dbg.check.path_key, "0");
assert.strictEqual(dbg.check.result, 11);
assert.strictEqual(dbg.fallback.result, undefined);

const resolved = dbg.resolve("0");
assert.ok(resolved instanceof CheckDebug);
assert.strictEqual(resolved.name, "reach-front");
assert.strictEqual(dbg.resolve("9"), null);

assert.strictEqual(typeof PlanDebug.prototype.evaluate, "undefined");
assert.strictEqual(typeof CheckInput.prototype.result, "undefined");

console.log("typestate assertions passed");
