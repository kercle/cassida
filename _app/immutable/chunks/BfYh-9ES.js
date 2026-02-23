var __defProp = Object.defineProperty;
var __typeError = (msg) => {
  throw TypeError(msg);
};
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => __defNormalProp(obj, typeof key !== "symbol" ? key + "" : key, value);
var __accessCheck = (obj, member, msg) => member.has(obj) || __typeError("Cannot " + msg);
var __privateGet = (obj, member, getter) => (__accessCheck(obj, member, "read from private field"), getter ? getter.call(obj) : member.get(obj));
var __privateAdd = (obj, member, value) => member.has(obj) ? __typeError("Cannot add the same private member more than once") : member instanceof WeakSet ? member.add(obj) : member.set(obj, value);
var __privateSet = (obj, member, value, setter) => (__accessCheck(obj, member, "write to private field"), setter ? setter.call(obj, value) : member.set(obj, value), value);
var __privateMethod = (obj, member, method) => (__accessCheck(obj, member, "access private method"), method);
var _a, _r2, _s, _t2, _f, _a2, _l, _n2, _e2, _i, _ue_instances, u_fn, o_fn, c_fn, __fn;
var Ut = Array.isArray, Gt = Array.prototype.indexOf, ie = Array.prototype.includes, Fn = Array.from, Ln = Object.defineProperty, ve = Object.getOwnPropertyDescriptor, zt = Object.getOwnPropertyDescriptors, $t = Object.prototype, Kt = Array.prototype, at = Object.getPrototypeOf, Je = Object.isExtensible;
function jn(e) {
  return typeof e == "function";
}
const de = () => {
};
function Yn(e) {
  return e();
}
function Xt(e) {
  for (var t = 0; t < e.length; t++) e[t]();
}
function lt() {
  var e, t, n = new Promise((r, s) => {
    e = r, t = s;
  });
  return { promise: n, resolve: e, reject: t };
}
const g = 2, ye = 4, be = 8, ut = 1 << 24, q = 16, F = 32, se = 64, Zt = 128, O = 512, y = 1024, T = 2048, C = 4096, M = 8192, V = 16384, J = 32768, Ae = 65536, Qe = 1 << 17, ot = 1 << 18, Ee = 1 << 19, ct = 1 << 20, qn = 1 << 25, Q = 65536, Pe = 1 << 21, qe = 1 << 22, B = 1 << 23, he = /* @__PURE__ */ Symbol("$state"), Hn = /* @__PURE__ */ Symbol("legacy props"), Vn = /* @__PURE__ */ Symbol(""), K = new class extends Error {
  constructor() {
    super(...arguments);
    __publicField(this, "name", "StaleReactionError");
    __publicField(this, "message", "The reaction that called `getAbortSignal()` was re-run or destroyed");
  }
}(), Un = !!((_a = globalThis.document) == null ? void 0 : _a.contentType) && globalThis.document.contentType.includes("xml"), Ne = 3, _t = 8;
function Wt() {
  throw new Error("https://svelte.dev/e/async_derived_orphan");
}
function Gn(e, t, n) {
  throw new Error("https://svelte.dev/e/each_key_duplicate");
}
function Jt(e) {
  throw new Error("https://svelte.dev/e/effect_in_teardown");
}
function Qt() {
  throw new Error("https://svelte.dev/e/effect_in_unowned_derived");
}
function en(e) {
  throw new Error("https://svelte.dev/e/effect_orphan");
}
function tn() {
  throw new Error("https://svelte.dev/e/effect_update_depth_exceeded");
}
function zn() {
  throw new Error("https://svelte.dev/e/hydration_failed");
}
function $n(e) {
  throw new Error("https://svelte.dev/e/props_invalid_value");
}
function nn() {
  throw new Error("https://svelte.dev/e/state_descriptors_fixed");
}
function rn() {
  throw new Error("https://svelte.dev/e/state_prototype_fixed");
}
function sn() {
  throw new Error("https://svelte.dev/e/state_unsafe_mutation");
}
function Kn() {
  throw new Error("https://svelte.dev/e/svelte_boundary_reset_onerror");
}
const Xn = 1, Zn = 2, Wn = 16, Jn = 1, Qn = 2, er = 4, tr = 8, nr = 16, rr = 4, sr = 1, fr = 2, fn = "[", an = "[!", ir = "[?", ln = "]", He = {}, b = /* @__PURE__ */ Symbol(), un = "http://www.w3.org/1999/xhtml", ar = "http://www.w3.org/2000/svg", lr = "http://www.w3.org/1998/Math/MathML";
function Ve(e) {
  console.warn("https://svelte.dev/e/hydration_mismatch");
}
function ur() {
  console.warn("https://svelte.dev/e/svelte_boundary_reset_noop");
}
let ee = false;
function or(e) {
  ee = e;
}
let R;
function ae(e) {
  if (e === null) throw Ve(), He;
  return R = e;
}
function cr() {
  return ae(z(R));
}
function _r(e) {
  if (ee) {
    if (z(R) !== null) throw Ve(), He;
    R = e;
  }
}
function vr(e = 1) {
  if (ee) {
    for (var t = e, n = R; t--; ) n = z(n);
    R = n;
  }
}
function dr(e = true) {
  for (var t = 0, n = R; ; ) {
    if (n.nodeType === _t) {
      var r = n.data;
      if (r === ln) {
        if (t === 0) return n;
        t -= 1;
      } else (r === fn || r === an || r[0] === "[" && !isNaN(Number(r.slice(1)))) && (t += 1);
    }
    var s = z(n);
    e && n.remove(), n = s;
  }
}
function hr(e) {
  if (!e || e.nodeType !== _t) throw Ve(), He;
  return e.data;
}
function vt(e) {
  return e === this.v;
}
function dt(e, t) {
  return e != e ? t == t : e !== t || e !== null && typeof e == "object" || typeof e == "function";
}
function ht(e) {
  return !dt(e, this.v);
}
let Oe = false;
function pr() {
  Oe = true;
}
let E = null;
function Se(e) {
  E = e;
}
function wr(e, t = false, n) {
  E = { p: E, i: false, c: null, e: null, s: e, x: null, l: Oe && !t ? { s: null, u: null, $: [] } : null };
}
function yr(e) {
  var t = E, n = t.e;
  if (n !== null) {
    t.e = null;
    for (var r of n) kt(r);
  }
  return t.i = true, E = t.p, {};
}
function ge() {
  return !Oe || E !== null && E.l === null;
}
let X = [];
function pt() {
  var e = X;
  X = [], Xt(e);
}
function et(e) {
  if (X.length === 0 && !pe) {
    var t = X;
    queueMicrotask(() => {
      t === X && pt();
    });
  }
  X.push(e);
}
function on() {
  for (; X.length > 0; ) pt();
}
function cn(e) {
  var t = d;
  if (t === null) return v.f |= B, e;
  if ((t.f & J) === 0 && (t.f & ye) === 0) throw e;
  Re(e, t);
}
function Re(e, t) {
  for (; t !== null; ) {
    if ((t.f & Zt) !== 0) {
      if ((t.f & J) === 0) throw e;
      try {
        t.b.error(e);
        return;
      } catch (n) {
        e = n;
      }
    }
    t = t.parent;
  }
  throw e;
}
const _n = -7169;
function w(e, t) {
  e.f = e.f & _n | t;
}
function Be(e) {
  (e.f & O) !== 0 || e.deps === null ? w(e, y) : w(e, C);
}
function wt(e) {
  if (e !== null) for (const t of e) (t.f & g) === 0 || (t.f & Q) === 0 || (t.f ^= Q, wt(t.deps));
}
function vn(e, t, n) {
  (e.f & T) !== 0 ? t.add(e) : (e.f & C) !== 0 && n.add(e), wt(e.deps), w(e, y);
}
const me = /* @__PURE__ */ new Set();
let p = null, tt = null, I = null, A = [], ke = null, Ce = false, pe = false;
const _ue = class _ue {
  constructor() {
    __privateAdd(this, _ue_instances);
    __publicField(this, "current", /* @__PURE__ */ new Map());
    __publicField(this, "previous", /* @__PURE__ */ new Map());
    __privateAdd(this, _r2, /* @__PURE__ */ new Set());
    __privateAdd(this, _s, /* @__PURE__ */ new Set());
    __privateAdd(this, _t2, 0);
    __privateAdd(this, _f, 0);
    __privateAdd(this, _a2, null);
    __privateAdd(this, _l, /* @__PURE__ */ new Set());
    __privateAdd(this, _n2, /* @__PURE__ */ new Set());
    __privateAdd(this, _e2, /* @__PURE__ */ new Map());
    __publicField(this, "is_fork", false);
    __privateAdd(this, _i, false);
  }
  skip_effect(t) {
    __privateGet(this, _e2).has(t) || __privateGet(this, _e2).set(t, { d: [], m: [] });
  }
  unskip_effect(t) {
    var n = __privateGet(this, _e2).get(t);
    if (n) {
      __privateGet(this, _e2).delete(t);
      for (var r of n.d) w(r, T), Y(r);
      for (r of n.m) w(r, C), Y(r);
    }
  }
  process(t) {
    var _a3;
    A = [], this.apply();
    var n = [], r = [];
    for (const s of t) __privateMethod(this, _ue_instances, o_fn).call(this, s, n, r);
    if (__privateMethod(this, _ue_instances, u_fn).call(this)) {
      __privateMethod(this, _ue_instances, c_fn).call(this, r), __privateMethod(this, _ue_instances, c_fn).call(this, n);
      for (const [s, f] of __privateGet(this, _e2)) gt(s, f);
    } else {
      for (const s of __privateGet(this, _r2)) s();
      __privateGet(this, _r2).clear(), __privateGet(this, _t2) === 0 && __privateMethod(this, _ue_instances, __fn).call(this), tt = this, p = null, nt(r), nt(n), tt = null, (_a3 = __privateGet(this, _a2)) == null ? void 0 : _a3.resolve();
    }
    I = null;
  }
  capture(t, n) {
    n !== b && !this.previous.has(t) && this.previous.set(t, n), (t.f & B) === 0 && (this.current.set(t, t.v), I == null ? void 0 : I.set(t, t.v));
  }
  activate() {
    p = this, this.apply();
  }
  deactivate() {
    p === this && (p = null, I = null);
  }
  flush() {
    if (this.activate(), A.length > 0) {
      if (yt(), p !== null && p !== this) return;
    } else __privateGet(this, _t2) === 0 && this.process([]);
    this.deactivate();
  }
  discard() {
    for (const t of __privateGet(this, _s)) t(this);
    __privateGet(this, _s).clear();
  }
  increment(t) {
    __privateSet(this, _t2, __privateGet(this, _t2) + 1), t && __privateSet(this, _f, __privateGet(this, _f) + 1);
  }
  decrement(t) {
    __privateSet(this, _t2, __privateGet(this, _t2) - 1), t && __privateSet(this, _f, __privateGet(this, _f) - 1), !__privateGet(this, _i) && (__privateSet(this, _i, true), et(() => {
      __privateSet(this, _i, false), __privateMethod(this, _ue_instances, u_fn).call(this) ? A.length > 0 && this.flush() : this.revive();
    }));
  }
  revive() {
    for (const t of __privateGet(this, _l)) __privateGet(this, _n2).delete(t), w(t, T), Y(t);
    for (const t of __privateGet(this, _n2)) w(t, C), Y(t);
    this.flush();
  }
  oncommit(t) {
    __privateGet(this, _r2).add(t);
  }
  ondiscard(t) {
    __privateGet(this, _s).add(t);
  }
  settled() {
    return (__privateGet(this, _a2) ?? __privateSet(this, _a2, lt())).promise;
  }
  static ensure() {
    if (p === null) {
      const t = p = new _ue();
      me.add(p), pe || et(() => {
        p === t && t.flush();
      });
    }
    return p;
  }
  apply() {
  }
};
_r2 = new WeakMap();
_s = new WeakMap();
_t2 = new WeakMap();
_f = new WeakMap();
_a2 = new WeakMap();
_l = new WeakMap();
_n2 = new WeakMap();
_e2 = new WeakMap();
_i = new WeakMap();
_ue_instances = new WeakSet();
u_fn = function() {
  return this.is_fork || __privateGet(this, _f) > 0;
};
o_fn = function(t, n, r) {
  t.f ^= y;
  for (var s = t.first; s !== null; ) {
    var f = s.f, u = (f & (F | se)) !== 0, i = u && (f & y) !== 0, a = i || (f & M) !== 0 || __privateGet(this, _e2).has(s);
    if (!a && s.fn !== null) {
      u ? s.f ^= y : (f & ye) !== 0 ? n.push(s) : oe(s) && ((f & q) !== 0 && __privateGet(this, _n2).add(s), re(s));
      var l = s.first;
      if (l !== null) {
        s = l;
        continue;
      }
    }
    for (; s !== null; ) {
      var c = s.next;
      if (c !== null) {
        s = c;
        break;
      }
      s = s.parent;
    }
  }
};
c_fn = function(t) {
  for (var n = 0; n < t.length; n += 1) vn(t[n], __privateGet(this, _l), __privateGet(this, _n2));
};
__fn = function() {
  var _a3;
  if (me.size > 1) {
    this.previous.clear();
    var t = I, n = true;
    for (const s of me) {
      if (s === this) {
        n = false;
        continue;
      }
      const f = [];
      for (const [i, a] of this.current) {
        if (s.current.has(i)) if (n && a !== s.current.get(i)) s.current.set(i, a);
        else continue;
        f.push(i);
      }
      if (f.length === 0) continue;
      const u = [...s.current.keys()].filter((i) => !this.current.has(i));
      if (u.length > 0) {
        var r = A;
        A = [];
        const i = /* @__PURE__ */ new Set(), a = /* @__PURE__ */ new Map();
        for (const l of f) bt(l, u, i, a);
        if (A.length > 0) {
          p = s, s.apply();
          for (const l of A) __privateMethod(_a3 = s, _ue_instances, o_fn).call(_a3, l, [], []);
          s.deactivate();
        }
        A = r;
      }
    }
    p = null, I = t;
  }
  me.delete(this);
};
let ue = _ue;
function dn(e) {
  var t = pe;
  pe = true;
  try {
    for (var n; ; ) {
      if (on(), A.length === 0 && (p == null ? void 0 : p.flush(), A.length === 0)) return ke = null, n;
      yt();
    }
  } finally {
    pe = t;
  }
}
function yt() {
  Ce = true;
  var e = null;
  try {
    for (var t = 0; A.length > 0; ) {
      var n = ue.ensure();
      if (t++ > 1e3) {
        var r, s;
        hn();
      }
      n.process(A), U.clear();
    }
  } finally {
    A = [], Ce = false, ke = null;
  }
}
function hn() {
  try {
    tn();
  } catch (e) {
    Re(e, ke);
  }
}
let j = null;
function nt(e) {
  var t = e.length;
  if (t !== 0) {
    for (var n = 0; n < t; ) {
      var r = e[n++];
      if ((r.f & (V | M)) === 0 && oe(r) && (j = /* @__PURE__ */ new Set(), re(r), r.deps === null && r.first === null && r.nodes === null && r.teardown === null && r.ac === null && Pt(r), (j == null ? void 0 : j.size) > 0)) {
        U.clear();
        for (const s of j) {
          if ((s.f & (V | M)) !== 0) continue;
          const f = [s];
          let u = s.parent;
          for (; u !== null; ) j.has(u) && (j.delete(u), f.push(u)), u = u.parent;
          for (let i = f.length - 1; i >= 0; i--) {
            const a = f[i];
            (a.f & (V | M)) === 0 && re(a);
          }
        }
        j.clear();
      }
    }
    j = null;
  }
}
function bt(e, t, n, r) {
  if (!n.has(e) && (n.add(e), e.reactions !== null)) for (const s of e.reactions) {
    const f = s.f;
    (f & g) !== 0 ? bt(s, t, n, r) : (f & (qe | q)) !== 0 && (f & T) === 0 && Et(s, t, r) && (w(s, T), Y(s));
  }
}
function Et(e, t, n) {
  const r = n.get(e);
  if (r !== void 0) return r;
  if (e.deps !== null) for (const s of e.deps) {
    if (ie.call(t, s)) return true;
    if ((s.f & g) !== 0 && Et(s, t, n)) return n.set(s, true), true;
  }
  return n.set(e, false), false;
}
function Y(e) {
  var t = ke = e, n = t.b;
  if ((n == null ? void 0 : n.is_pending) && (e.f & (ye | be | ut)) !== 0 && (e.f & J) === 0) {
    n.defer_effect(e);
    return;
  }
  for (; t.parent !== null; ) {
    t = t.parent;
    var r = t.f;
    if (Ce && t === d && (r & q) !== 0 && (r & ot) === 0 && (r & J) !== 0) return;
    if ((r & (se | F)) !== 0) {
      if ((r & y) === 0) return;
      t.f ^= y;
    }
  }
  A.push(t);
}
function gt(e, t) {
  if (!((e.f & F) !== 0 && (e.f & y) !== 0)) {
    (e.f & T) !== 0 ? t.d.push(e) : (e.f & C) !== 0 && t.m.push(e), w(e, y);
    for (var n = e.first; n !== null; ) gt(n, t), n = n.next;
  }
}
function pn(e, t, n, r) {
  const s = ge() ? Ue : En;
  var f = e.filter((o) => !o.settled);
  if (n.length === 0 && f.length === 0) {
    r(t.map(s));
    return;
  }
  var u = d, i = wn(), a = f.length === 1 ? f[0].promise : f.length > 1 ? Promise.all(f.map((o) => o.promise)) : null;
  function l(o) {
    i();
    try {
      r(o);
    } catch (_) {
      (u.f & V) === 0 && Re(_, u);
    }
    Me();
  }
  if (n.length === 0) {
    a.then(() => l(t.map(s)));
    return;
  }
  function c() {
    i(), Promise.all(n.map((o) => bn(o))).then((o) => l([...t.map(s), ...o])).catch((o) => Re(o, u));
  }
  a ? a.then(c) : c();
}
function wn() {
  var e = d, t = v, n = E, r = p;
  return function(f = true) {
    le(e), G(t), Se(n), f && (r == null ? void 0 : r.activate());
  };
}
function Me(e = true) {
  le(null), G(null), Se(null), e && (p == null ? void 0 : p.deactivate());
}
function yn() {
  var e = d.b, t = p, n = e.is_rendered();
  return e.update_pending_count(1), t.increment(n), () => {
    e.update_pending_count(-1), t.decrement(n);
  };
}
function Ue(e) {
  var t = g | T, n = v !== null && (v.f & g) !== 0 ? v : null;
  return d !== null && (d.f |= Ee), { ctx: E, deps: null, effects: null, equals: vt, f: t, fn: e, reactions: null, rv: 0, v: b, wv: 0, parent: n ?? d, ac: null };
}
function bn(e, t, n) {
  d === null && Wt();
  var s = void 0, f = ze(b), u = !v, i = /* @__PURE__ */ new Map();
  return On(() => {
    var _a3;
    var a = lt();
    s = a.promise;
    try {
      Promise.resolve(e()).then(a.resolve, a.reject).finally(Me);
    } catch (_) {
      a.reject(_), Me();
    }
    var l = p;
    if (u) {
      var c = yn();
      (_a3 = i.get(l)) == null ? void 0 : _a3.reject(K), i.delete(l), i.set(l, a);
    }
    const o = (_, m = void 0) => {
      if (l.activate(), m) m !== K && (f.f |= B, Le(f, m));
      else {
        (f.f & B) !== 0 && (f.f ^= B), Le(f, _);
        for (const [h, D] of i) {
          if (i.delete(h), h === l) break;
          D.reject(K);
        }
      }
      c && c();
    };
    a.promise.then(o, (_) => o(null, _ || "unknown"));
  }), Nn(() => {
    for (const a of i.values()) a.reject(K);
  }), new Promise((a) => {
    function l(c) {
      function o() {
        c === s ? a(f) : l(s);
      }
      c.then(o, o);
    }
    l(s);
  });
}
function br(e) {
  const t = Ue(e);
  return Ft(t), t;
}
function En(e) {
  const t = Ue(e);
  return t.equals = ht, t;
}
function gn(e) {
  var t = e.effects;
  if (t !== null) {
    e.effects = null;
    for (var n = 0; n < t.length; n += 1) te(t[n]);
  }
}
function mn(e) {
  for (var t = e.parent; t !== null; ) {
    if ((t.f & g) === 0) return (t.f & V) === 0 ? t : null;
    t = t.parent;
  }
  return null;
}
function Ge(e) {
  var t, n = d;
  le(mn(e));
  try {
    e.f &= ~Q, gn(e), t = qt(e);
  } finally {
    le(n);
  }
  return t;
}
function mt(e) {
  var t = Ge(e);
  if (!e.equals(t) && (e.wv = jt(), (!(p == null ? void 0 : p.is_fork) || e.deps === null) && (e.v = t, e.deps === null))) {
    w(e, y);
    return;
  }
  ne || (I !== null ? (Ot() || (p == null ? void 0 : p.is_fork)) && I.set(e, t) : Be(e));
}
function Tn(e) {
  var _a3, _b;
  if (e.effects !== null) for (const t of e.effects) (t.teardown || t.ac) && ((_a3 = t.teardown) == null ? void 0 : _a3.call(t), (_b = t.ac) == null ? void 0 : _b.abort(K), t.teardown = de, t.ac = null, we(t, 0), Xe(t));
}
function Tt(e) {
  if (e.effects !== null) for (const t of e.effects) t.teardown && re(t);
}
let Fe = /* @__PURE__ */ new Set();
const U = /* @__PURE__ */ new Map();
let At = false;
function ze(e, t) {
  var n = { f: 0, v: e, reactions: null, equals: vt, rv: 0, wv: 0 };
  return n;
}
function H(e, t) {
  const n = ze(e);
  return Ft(n), n;
}
function Er(e, t = false, n = true) {
  var _a3;
  const r = ze(e);
  return t || (r.equals = ht), Oe && n && E !== null && E.l !== null && ((_a3 = E.l).s ?? (_a3.s = [])).push(r), r;
}
function $(e, t, n = false) {
  v !== null && (!P || (v.f & Qe) !== 0) && ge() && (v.f & (g | q | qe | Qe)) !== 0 && (k === null || !ie.call(k, e)) && sn();
  let r = n ? ce(t) : t;
  return Le(e, r);
}
function Le(e, t) {
  if (!e.equals(t)) {
    var n = e.v;
    ne ? U.set(e, t) : U.set(e, n), e.v = t;
    var r = ue.ensure();
    if (r.capture(e, n), (e.f & g) !== 0) {
      const s = e;
      (e.f & T) !== 0 && Ge(s), Be(s);
    }
    e.wv = jt(), St(e, T), ge() && d !== null && (d.f & y) !== 0 && (d.f & (F | se)) === 0 && (N === null ? Pn([e]) : N.push(e)), !r.is_fork && Fe.size > 0 && !At && An();
  }
  return t;
}
function An() {
  At = false;
  for (const e of Fe) (e.f & y) !== 0 && w(e, C), oe(e) && re(e);
  Fe.clear();
}
function Ie(e) {
  $(e, e.v + 1);
}
function St(e, t) {
  var n = e.reactions;
  if (n !== null) for (var r = ge(), s = n.length, f = 0; f < s; f++) {
    var u = n[f], i = u.f;
    if (!(!r && u === d)) {
      var a = (i & T) === 0;
      if (a && w(u, t), (i & g) !== 0) {
        var l = u;
        I == null ? void 0 : I.delete(l), (i & Q) === 0 && (i & O && (u.f |= Q), St(l, C));
      } else a && ((i & q) !== 0 && j !== null && j.add(u), Y(u));
    }
  }
}
function ce(e) {
  if (typeof e != "object" || e === null || he in e) return e;
  const t = at(e);
  if (t !== $t && t !== Kt) return e;
  var n = /* @__PURE__ */ new Map(), r = Ut(e), s = H(0), f = W, u = (i) => {
    if (W === f) return i();
    var a = v, l = W;
    G(null), it(f);
    var c = i();
    return G(a), it(l), c;
  };
  return r && n.set("length", H(e.length)), new Proxy(e, { defineProperty(i, a, l) {
    (!("value" in l) || l.configurable === false || l.enumerable === false || l.writable === false) && nn();
    var c = n.get(a);
    return c === void 0 ? u(() => {
      var o = H(l.value);
      return n.set(a, o), o;
    }) : $(c, l.value, true), true;
  }, deleteProperty(i, a) {
    var l = n.get(a);
    if (l === void 0) {
      if (a in i) {
        const c = u(() => H(b));
        n.set(a, c), Ie(s);
      }
    } else $(l, b), Ie(s);
    return true;
  }, get(i, a, l) {
    var _a3;
    if (a === he) return e;
    var c = n.get(a), o = a in i;
    if (c === void 0 && (!o || ((_a3 = ve(i, a)) == null ? void 0 : _a3.writable)) && (c = u(() => {
      var m = ce(o ? i[a] : b), h = H(m);
      return h;
    }), n.set(a, c)), c !== void 0) {
      var _ = _e(c);
      return _ === b ? void 0 : _;
    }
    return Reflect.get(i, a, l);
  }, getOwnPropertyDescriptor(i, a) {
    var l = Reflect.getOwnPropertyDescriptor(i, a);
    if (l && "value" in l) {
      var c = n.get(a);
      c && (l.value = _e(c));
    } else if (l === void 0) {
      var o = n.get(a), _ = o == null ? void 0 : o.v;
      if (o !== void 0 && _ !== b) return { enumerable: true, configurable: true, value: _, writable: true };
    }
    return l;
  }, has(i, a) {
    var _a3;
    if (a === he) return true;
    var l = n.get(a), c = l !== void 0 && l.v !== b || Reflect.has(i, a);
    if (l !== void 0 || d !== null && (!c || ((_a3 = ve(i, a)) == null ? void 0 : _a3.writable))) {
      l === void 0 && (l = u(() => {
        var _ = c ? ce(i[a]) : b, m = H(_);
        return m;
      }), n.set(a, l));
      var o = _e(l);
      if (o === b) return false;
    }
    return c;
  }, set(i, a, l, c) {
    var _a3;
    var o = n.get(a), _ = a in i;
    if (r && a === "length") for (var m = l; m < o.v; m += 1) {
      var h = n.get(m + "");
      h !== void 0 ? $(h, b) : m in i && (h = u(() => H(b)), n.set(m + "", h));
    }
    if (o === void 0) (!_ || ((_a3 = ve(i, a)) == null ? void 0 : _a3.writable)) && (o = u(() => H(void 0)), $(o, ce(l)), n.set(a, o));
    else {
      _ = o.v !== b;
      var D = u(() => ce(l));
      $(o, D);
    }
    var Ze = Reflect.getOwnPropertyDescriptor(i, a);
    if ((Ze == null ? void 0 : Ze.set) && Ze.set.call(c, l), !_) {
      if (r && typeof a == "string") {
        var We = n.get("length"), De = Number(a);
        Number.isInteger(De) && De >= We.v && $(We, De + 1);
      }
      Ie(s);
    }
    return true;
  }, ownKeys(i) {
    _e(s);
    var a = Reflect.ownKeys(i).filter((o) => {
      var _ = n.get(o);
      return _ === void 0 || _.v !== b;
    });
    for (var [l, c] of n) c.v !== b && !(l in i) && a.push(l);
    return a;
  }, setPrototypeOf() {
    rn();
  } });
}
var rt, Sn, Rt, xt;
function gr() {
  if (rt === void 0) {
    rt = window, Sn = /Firefox/.test(navigator.userAgent);
    var e = Element.prototype, t = Node.prototype, n = Text.prototype;
    Rt = ve(t, "firstChild").get, xt = ve(t, "nextSibling").get, Je(e) && (e.__click = void 0, e.__className = void 0, e.__attributes = null, e.__style = void 0, e.__e = void 0), Je(n) && (n.__t = void 0);
  }
}
function xe(e = "") {
  return document.createTextNode(e);
}
function je(e) {
  return Rt.call(e);
}
function z(e) {
  return xt.call(e);
}
function mr(e, t) {
  if (!ee) return je(e);
  var n = je(R);
  if (n === null) n = R.appendChild(xe());
  else if (t && n.nodeType !== Ne) {
    var r = xe();
    return n == null ? void 0 : n.before(r), ae(r), r;
  }
  return t && $e(n), ae(n), n;
}
function Tr(e, t = false) {
  if (!ee) {
    var n = je(e);
    return n instanceof Comment && n.data === "" ? z(n) : n;
  }
  if (t) {
    if ((R == null ? void 0 : R.nodeType) !== Ne) {
      var r = xe();
      return R == null ? void 0 : R.before(r), ae(r), r;
    }
    $e(R);
  }
  return R;
}
function Ar(e, t = 1, n = false) {
  let r = ee ? R : e;
  for (var s; t--; ) s = r, r = z(r);
  if (!ee) return r;
  if (n) {
    if ((r == null ? void 0 : r.nodeType) !== Ne) {
      var f = xe();
      return r === null ? s == null ? void 0 : s.after(f) : r.before(f), ae(f), f;
    }
    $e(r);
  }
  return ae(r), r;
}
function Sr(e) {
  e.textContent = "";
}
function Rr() {
  return false;
}
function xr(e, t, n) {
  return document.createElementNS(t ?? un, e, void 0);
}
function $e(e) {
  if (e.nodeValue.length < 65536) return;
  let t = e.nextSibling;
  for (; t !== null && t.nodeType === Ne; ) t.remove(), e.nodeValue += t.nodeValue, t = e.nextSibling;
}
let st = false;
function Rn() {
  st || (st = true, document.addEventListener("reset", (e) => {
    Promise.resolve().then(() => {
      var _a3;
      if (!e.defaultPrevented) for (const t of e.target.elements) (_a3 = t.__on_r) == null ? void 0 : _a3.call(t);
    });
  }, { capture: true }));
}
function Ke(e) {
  var t = v, n = d;
  G(null), le(null);
  try {
    return e();
  } finally {
    G(t), le(n);
  }
}
function Nr(e, t, n, r = n) {
  e.addEventListener(t, () => Ke(n));
  const s = e.__on_r;
  s ? e.__on_r = () => {
    s(), r(true);
  } : e.__on_r = () => r(true), Rn();
}
function Nt(e) {
  d === null && (v === null && en(), Qt()), ne && Jt();
}
function xn(e, t) {
  var n = t.last;
  n === null ? t.last = t.first = e : (n.next = e, e.prev = n, t.last = e);
}
function L(e, t, n) {
  var r = d;
  r !== null && (r.f & M) !== 0 && (e |= M);
  var s = { ctx: E, deps: null, nodes: null, f: e | T | O, first: null, fn: t, last: null, next: null, parent: r, b: r && r.b, prev: null, teardown: null, wv: 0, ac: null };
  if (n) try {
    re(s);
  } catch (i) {
    throw te(s), i;
  }
  else t !== null && Y(s);
  var f = s;
  if (n && f.deps === null && f.teardown === null && f.nodes === null && f.first === f.last && (f.f & Ee) === 0 && (f = f.first, (e & q) !== 0 && (e & Ae) !== 0 && f !== null && (f.f |= Ae)), f !== null && (f.parent = r, r !== null && xn(f, r), v !== null && (v.f & g) !== 0 && (e & se) === 0)) {
    var u = v;
    (u.effects ?? (u.effects = [])).push(f);
  }
  return s;
}
function Ot() {
  return v !== null && !P;
}
function Nn(e) {
  const t = L(be, null, false);
  return w(t, y), t.teardown = e, t;
}
function Or(e) {
  Nt();
  var t = d.f, n = !v && (t & F) !== 0 && (t & J) === 0;
  if (n) {
    var r = E;
    (r.e ?? (r.e = [])).push(e);
  } else return kt(e);
}
function kt(e) {
  return L(ye | ct, e, false);
}
function kr(e) {
  return Nt(), L(be | ct, e, true);
}
function Dr(e) {
  ue.ensure();
  const t = L(se | Ee, e, true);
  return (n = {}) => new Promise((r) => {
    n.outro ? In(t, () => {
      te(t), r(void 0);
    }) : (te(t), r(void 0));
  });
}
function Ir(e) {
  return L(ye, e, false);
}
function Pr(e, t) {
  var n = E, r = { effect: null, ran: false, deps: e };
  n.l.$.push(r), r.effect = Dt(() => {
    e(), !r.ran && (r.ran = true, Bt(t));
  });
}
function Cr() {
  var e = E;
  Dt(() => {
    for (var t of e.l.$) {
      t.deps();
      var n = t.effect;
      (n.f & y) !== 0 && n.deps !== null && w(n, C), oe(n) && re(n), t.ran = false;
    }
  });
}
function On(e) {
  return L(qe | Ee, e, true);
}
function Dt(e, t = 0) {
  return L(be | t, e, true);
}
function Mr(e, t = [], n = [], r = []) {
  pn(r, t, n, (s) => {
    L(be, () => e(...s.map(_e)), true);
  });
}
function Fr(e, t = 0) {
  var n = L(q | t, e, true);
  return n;
}
function Lr(e) {
  return L(F | Ee, e, true);
}
function It(e) {
  var t = e.teardown;
  if (t !== null) {
    const n = ne, r = v;
    ft(true), G(null);
    try {
      t.call(null);
    } finally {
      ft(n), G(r);
    }
  }
}
function Xe(e, t = false) {
  var n = e.first;
  for (e.first = e.last = null; n !== null; ) {
    const s = n.ac;
    s !== null && Ke(() => {
      s.abort(K);
    });
    var r = n.next;
    (n.f & se) !== 0 ? n.parent = null : te(n, t), n = r;
  }
}
function kn(e) {
  for (var t = e.first; t !== null; ) {
    var n = t.next;
    (t.f & F) === 0 && te(t), t = n;
  }
}
function te(e, t = true) {
  var n = false;
  (t || (e.f & ot) !== 0) && e.nodes !== null && e.nodes.end !== null && (Dn(e.nodes.start, e.nodes.end), n = true), Xe(e, t && !n), we(e, 0), w(e, V);
  var r = e.nodes && e.nodes.t;
  if (r !== null) for (const f of r) f.stop();
  It(e);
  var s = e.parent;
  s !== null && s.first !== null && Pt(e), e.next = e.prev = e.teardown = e.ctx = e.deps = e.fn = e.nodes = e.ac = null;
}
function Dn(e, t) {
  for (; e !== null; ) {
    var n = e === t ? null : z(e);
    e.remove(), e = n;
  }
}
function Pt(e) {
  var t = e.parent, n = e.prev, r = e.next;
  n !== null && (n.next = r), r !== null && (r.prev = n), t !== null && (t.first === e && (t.first = r), t.last === e && (t.last = n));
}
function In(e, t, n = true) {
  var r = [];
  Ct(e, r, true);
  var s = () => {
    n && te(e), t && t();
  }, f = r.length;
  if (f > 0) {
    var u = () => --f || s();
    for (var i of r) i.out(u);
  } else s();
}
function Ct(e, t, n) {
  if ((e.f & M) === 0) {
    e.f ^= M;
    var r = e.nodes && e.nodes.t;
    if (r !== null) for (const i of r) (i.is_global || n) && t.push(i);
    for (var s = e.first; s !== null; ) {
      var f = s.next, u = (s.f & Ae) !== 0 || (s.f & F) !== 0 && (e.f & q) !== 0;
      Ct(s, t, u ? n : false), s = f;
    }
  }
}
function jr(e) {
  Mt(e, true);
}
function Mt(e, t) {
  if ((e.f & M) !== 0) {
    e.f ^= M, (e.f & y) === 0 && (w(e, T), Y(e));
    for (var n = e.first; n !== null; ) {
      var r = n.next, s = (n.f & Ae) !== 0 || (n.f & F) !== 0;
      Mt(n, s ? t : false), n = r;
    }
    var f = e.nodes && e.nodes.t;
    if (f !== null) for (const u of f) (u.is_global || t) && u.in();
  }
}
function Yr(e, t) {
  if (e.nodes) for (var n = e.nodes.start, r = e.nodes.end; n !== null; ) {
    var s = n === r ? null : z(n);
    t.append(n), n = s;
  }
}
let Te = false, ne = false;
function ft(e) {
  ne = e;
}
let v = null, P = false;
function G(e) {
  v = e;
}
let d = null;
function le(e) {
  d = e;
}
let k = null;
function Ft(e) {
  v !== null && (k === null ? k = [e] : k.push(e));
}
let S = null, x = 0, N = null;
function Pn(e) {
  N = e;
}
let Lt = 1, Z = 0, W = Z;
function it(e) {
  W = e;
}
function jt() {
  return ++Lt;
}
function oe(e) {
  var t = e.f;
  if ((t & T) !== 0) return true;
  if (t & g && (e.f &= ~Q), (t & C) !== 0) {
    for (var n = e.deps, r = n.length, s = 0; s < r; s++) {
      var f = n[s];
      if (oe(f) && mt(f), f.wv > e.wv) return true;
    }
    (t & O) !== 0 && I === null && w(e, y);
  }
  return false;
}
function Yt(e, t, n = true) {
  var r = e.reactions;
  if (r !== null && !(k !== null && ie.call(k, e))) for (var s = 0; s < r.length; s++) {
    var f = r[s];
    (f.f & g) !== 0 ? Yt(f, t, false) : t === f && (n ? w(f, T) : (f.f & y) !== 0 && w(f, C), Y(f));
  }
}
function qt(e) {
  var _a3;
  var t = S, n = x, r = N, s = v, f = k, u = E, i = P, a = W, l = e.f;
  S = null, x = 0, N = null, v = (l & (F | se)) === 0 ? e : null, k = null, Se(e.ctx), P = false, W = ++Z, e.ac !== null && (Ke(() => {
    e.ac.abort(K);
  }), e.ac = null);
  try {
    e.f |= Pe;
    var c = e.fn, o = c();
    e.f |= J;
    var _ = e.deps, m = p == null ? void 0 : p.is_fork;
    if (S !== null) {
      var h;
      if (m || we(e, x), _ !== null && x > 0) for (_.length = x + S.length, h = 0; h < S.length; h++) _[x + h] = S[h];
      else e.deps = _ = S;
      if (Ot() && (e.f & O) !== 0) for (h = x; h < _.length; h++) ((_a3 = _[h]).reactions ?? (_a3.reactions = [])).push(e);
    } else !m && _ !== null && x < _.length && (we(e, x), _.length = x);
    if (ge() && N !== null && !P && _ !== null && (e.f & (g | C | T)) === 0) for (h = 0; h < N.length; h++) Yt(N[h], e);
    if (s !== null && s !== e) {
      if (Z++, s.deps !== null) for (let D = 0; D < n; D += 1) s.deps[D].rv = Z;
      if (t !== null) for (const D of t) D.rv = Z;
      N !== null && (r === null ? r = N : r.push(...N));
    }
    return (e.f & B) !== 0 && (e.f ^= B), o;
  } catch (D) {
    return cn(D);
  } finally {
    e.f ^= Pe, S = t, x = n, N = r, v = s, k = f, Se(u), P = i, W = a;
  }
}
function Cn(e, t) {
  let n = t.reactions;
  if (n !== null) {
    var r = Gt.call(n, e);
    if (r !== -1) {
      var s = n.length - 1;
      s === 0 ? n = t.reactions = null : (n[r] = n[s], n.pop());
    }
  }
  if (n === null && (t.f & g) !== 0 && (S === null || !ie.call(S, t))) {
    var f = t;
    (f.f & O) !== 0 && (f.f ^= O, f.f &= ~Q), Be(f), Tn(f), we(f, 0);
  }
}
function we(e, t) {
  var n = e.deps;
  if (n !== null) for (var r = t; r < n.length; r++) Cn(e, n[r]);
}
function re(e) {
  var t = e.f;
  if ((t & V) === 0) {
    w(e, y);
    var n = d, r = Te;
    d = e, Te = true;
    try {
      (t & (q | ut)) !== 0 ? kn(e) : Xe(e), It(e);
      var s = qt(e);
      e.teardown = typeof s == "function" ? s : null, e.wv = Lt;
      var f;
    } finally {
      Te = r, d = n;
    }
  }
}
async function qr() {
  await Promise.resolve(), dn();
}
function Hr() {
  return ue.ensure().settled();
}
function _e(e) {
  var t = e.f, n = (t & g) !== 0;
  if (v !== null && !P) {
    var r = d !== null && (d.f & V) !== 0;
    if (!r && (k === null || !ie.call(k, e))) {
      var s = v.deps;
      if ((v.f & Pe) !== 0) e.rv < Z && (e.rv = Z, S === null && s !== null && s[x] === e ? x++ : S === null ? S = [e] : S.push(e));
      else {
        (v.deps ?? (v.deps = [])).push(e);
        var f = e.reactions;
        f === null ? e.reactions = [v] : ie.call(f, v) || f.push(v);
      }
    }
  }
  if (ne && U.has(e)) return U.get(e);
  if (n) {
    var u = e;
    if (ne) {
      var i = u.v;
      return ((u.f & y) === 0 && u.reactions !== null || Vt(u)) && (i = Ge(u)), U.set(u, i), i;
    }
    var a = (u.f & O) === 0 && !P && v !== null && (Te || (v.f & O) !== 0), l = (u.f & J) === 0;
    oe(u) && (a && (u.f |= O), mt(u)), a && !l && (Tt(u), Ht(u));
  }
  if (I == null ? void 0 : I.has(e)) return I.get(e);
  if ((e.f & B) !== 0) throw e.v;
  return e.v;
}
function Ht(e) {
  if (e.f |= O, e.deps !== null) for (const t of e.deps) (t.reactions ?? (t.reactions = [])).push(e), (t.f & g) !== 0 && (t.f & O) === 0 && (Tt(t), Ht(t));
}
function Vt(e) {
  if (e.v === b) return true;
  if (e.deps === null) return false;
  for (const t of e.deps) if (U.has(t) || (t.f & g) !== 0 && Vt(t)) return true;
  return false;
}
function Bt(e) {
  var t = P;
  try {
    return P = true, e();
  } finally {
    P = t;
  }
}
function Vr(e) {
  if (!(typeof e != "object" || !e || e instanceof EventTarget)) {
    if (he in e) Ye(e);
    else if (!Array.isArray(e)) for (let t in e) {
      const n = e[t];
      typeof n == "object" && n && he in n && Ye(n);
    }
  }
}
function Ye(e, t = /* @__PURE__ */ new Set()) {
  if (typeof e == "object" && e !== null && !(e instanceof EventTarget) && !t.has(e)) {
    t.add(e), e instanceof Date && e.getTime();
    for (let r in e) try {
      Ye(e[r], t);
    } catch {
    }
    const n = at(e);
    if (n !== Object.prototype && n !== Array.prototype && n !== Map.prototype && n !== Set.prototype && n !== Date.prototype) {
      const r = zt(n);
      for (let s in r) {
        const f = r[s].get;
        if (f) try {
          f.call(e);
        } catch {
        }
      }
    }
  }
}
function Mn(e, t, n) {
  if (e == null) return t(void 0), de;
  const r = Bt(() => e.subscribe(t, n));
  return r.unsubscribe ? () => r.unsubscribe() : r;
}
const fe = [];
function Br(e, t = de) {
  let n = null;
  const r = /* @__PURE__ */ new Set();
  function s(i) {
    if (dt(e, i) && (e = i, n)) {
      const a = !fe.length;
      for (const l of r) l[1](), fe.push(l, e);
      if (a) {
        for (let l = 0; l < fe.length; l += 2) fe[l][0](fe[l + 1]);
        fe.length = 0;
      }
    }
  }
  function f(i) {
    s(i(e));
  }
  function u(i, a = de) {
    const l = [i, a];
    return r.add(l), r.size === 1 && (n = t(s, f) || de), i(e), () => {
      r.delete(l), r.size === 0 && n && (n(), n = null);
    };
  }
  return { set: s, update: f, subscribe: u };
}
function Ur(e) {
  let t;
  return Mn(e, (n) => t = n)(), t;
}
export {
  d as $,
  _r as A,
  Ar as B,
  cr as C,
  Fr as D,
  Ae as E,
  hr as F,
  an as G,
  fn as H,
  dr as I,
  ae as J,
  or as K,
  Ir as L,
  Dt as M,
  et as N,
  Nn as O,
  Ln as P,
  de as Q,
  Er as R,
  he as S,
  Mn as T,
  Ur as U,
  $ as V,
  ve as W,
  $n as X,
  er as Y,
  ce as Z,
  ne as _,
  xe as a,
  Wn as a$,
  V as a0,
  tr as a1,
  Qn as a2,
  Jn as a3,
  En as a4,
  nr as a5,
  Hn as a6,
  xr as a7,
  je as a8,
  Sn as a9,
  ur as aA,
  gr as aB,
  _t as aC,
  z as aD,
  He as aE,
  zn as aF,
  Sr as aG,
  Dr as aH,
  Fn as aI,
  ln as aJ,
  Ve as aK,
  dn as aL,
  qr as aM,
  H as aN,
  br as aO,
  ot as aP,
  un as aQ,
  Un as aR,
  Rn as aS,
  Vn as aT,
  at as aU,
  zt as aV,
  Br as aW,
  qn as aX,
  Gn as aY,
  Ut as aZ,
  Xn as a_,
  sr as aa,
  fr as ab,
  J as ac,
  Ne as ad,
  $e as ae,
  Ot as af,
  Ie as ag,
  ze as ah,
  Zt as ai,
  ir as aj,
  ue as ak,
  w as al,
  T as am,
  Y as an,
  C as ao,
  vn as ap,
  le as aq,
  G as ar,
  Se as as,
  cn as at,
  v as au,
  Le as av,
  vr as aw,
  Re as ax,
  Ee as ay,
  Kn as az,
  Lr as b,
  Zn as b0,
  M as b1,
  F as b2,
  Dn as b3,
  ar as b4,
  lr as b5,
  q as b6,
  rr as b7,
  jn as b8,
  Ke as b9,
  Nr as ba,
  tt as bb,
  Pr as bc,
  Cr as bd,
  Hr as be,
  p as c,
  te as d,
  R as e,
  E as f,
  Bt as g,
  ee as h,
  kr as i,
  Xt as j,
  Yn as k,
  Oe as l,
  Yr as m,
  _e as n,
  Vr as o,
  In as p,
  Ue as q,
  jr as r,
  Rr as s,
  pr as t,
  Or as u,
  wr as v,
  Tr as w,
  Mr as x,
  yr as y,
  mr as z
};
