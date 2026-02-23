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
var _t, _g, _i, _h, _e2, _a, _r, _s, _n, _l, _o, _d, _u, _c, _f, _m, _Me_instances, b_fn, E_fn, T_fn, v_fn, __fn, p_fn, y_fn;
import { af as re, n as G, M as se, g as ie, ag as P, N as D, ah as K, e as g, h as v, $ as b, ai as V, D as ae, C as ne, G as fe, aj as q, b as p, a as U, ak as I, p as k, m as oe, al as L, am as he, an as j, ao as le, ap as de, aq as N, ar as A, as as z, at as ue, au as Q, f as X, av as ce, d as x, J as O, aw as _e, I as pe, ax as w, E as ge, ay as ve, az as ye, aA as me, P as be, aB as Y, a8 as Ee, aC as Z, H as Te, aD as we, aE as C, K as R, aF as Re, aG as Se, aH as De, aI as Ne, v as Ae, aJ as Oe, aK as Fe, y as Ie } from "./BfYh-9ES.js";
import { b as ke } from "./Dqshxp8n.js";
function xe(t) {
  let e = 0, r = K(0), i;
  return () => {
    re() && (G(r), se(() => (e === 0 && (i = ie(() => t(() => P(r)))), e += 1, () => {
      D(() => {
        e -= 1, e === 0 && (i == null ? void 0 : i(), i = void 0, P(r));
      });
    })));
  };
}
var Ye = ge | ve;
function Ce(t, e, r, i) {
  new Me(t, e, r, i);
}
class Me {
  constructor(e, r, i, f) {
    __privateAdd(this, _Me_instances);
    __publicField(this, "parent");
    __publicField(this, "is_pending", false);
    __publicField(this, "transform_error");
    __privateAdd(this, _t);
    __privateAdd(this, _g, v ? g : null);
    __privateAdd(this, _i);
    __privateAdd(this, _h);
    __privateAdd(this, _e2);
    __privateAdd(this, _a, null);
    __privateAdd(this, _r, null);
    __privateAdd(this, _s, null);
    __privateAdd(this, _n, null);
    __privateAdd(this, _l, 0);
    __privateAdd(this, _o, 0);
    __privateAdd(this, _d, false);
    __privateAdd(this, _u, /* @__PURE__ */ new Set());
    __privateAdd(this, _c, /* @__PURE__ */ new Set());
    __privateAdd(this, _f, null);
    __privateAdd(this, _m, xe(() => (__privateSet(this, _f, K(__privateGet(this, _l))), () => {
      __privateSet(this, _f, null);
    })));
    var _a2;
    __privateSet(this, _t, e), __privateSet(this, _i, r), __privateSet(this, _h, (s) => {
      var a = b;
      a.b = this, a.f |= V, i(s);
    }), this.parent = b.b, this.transform_error = f ?? ((_a2 = this.parent) == null ? void 0 : _a2.transform_error) ?? ((s) => s), __privateSet(this, _e2, ae(() => {
      if (v) {
        const s = __privateGet(this, _g);
        ne();
        const a = s.data === fe;
        if (s.data.startsWith(q)) {
          const n = JSON.parse(s.data.slice(q.length));
          __privateMethod(this, _Me_instances, E_fn).call(this, n);
        } else a ? __privateMethod(this, _Me_instances, T_fn).call(this) : __privateMethod(this, _Me_instances, b_fn).call(this);
      } else __privateMethod(this, _Me_instances, v_fn).call(this);
    }, Ye)), v && __privateSet(this, _t, g);
  }
  defer_effect(e) {
    de(e, __privateGet(this, _u), __privateGet(this, _c));
  }
  is_rendered() {
    return !this.is_pending && (!this.parent || this.parent.is_rendered());
  }
  has_pending_snippet() {
    return !!__privateGet(this, _i).pending;
  }
  update_pending_count(e) {
    __privateMethod(this, _Me_instances, y_fn).call(this, e), __privateSet(this, _l, __privateGet(this, _l) + e), !(!__privateGet(this, _f) || __privateGet(this, _d)) && (__privateSet(this, _d, true), D(() => {
      __privateSet(this, _d, false), __privateGet(this, _f) && ce(__privateGet(this, _f), __privateGet(this, _l));
    }));
  }
  get_effect_pending() {
    return __privateGet(this, _m).call(this), G(__privateGet(this, _f));
  }
  error(e) {
    var r = __privateGet(this, _i).onerror;
    let i = __privateGet(this, _i).failed;
    if (!r && !i) throw e;
    __privateGet(this, _a) && (x(__privateGet(this, _a)), __privateSet(this, _a, null)), __privateGet(this, _r) && (x(__privateGet(this, _r)), __privateSet(this, _r, null)), __privateGet(this, _s) && (x(__privateGet(this, _s)), __privateSet(this, _s, null)), v && (O(__privateGet(this, _g)), _e(), O(pe()));
    var f = false, s = false;
    const a = () => {
      if (f) {
        me();
        return;
      }
      f = true, s && ye(), __privateGet(this, _s) !== null && k(__privateGet(this, _s), () => {
        __privateSet(this, _s, null);
      }), __privateMethod(this, _Me_instances, p_fn).call(this, () => {
        I.ensure(), __privateMethod(this, _Me_instances, v_fn).call(this);
      });
    }, u = (n) => {
      try {
        s = true, r == null ? void 0 : r(n, a), s = false;
      } catch (o) {
        w(o, __privateGet(this, _e2) && __privateGet(this, _e2).parent);
      }
      i && __privateSet(this, _s, __privateMethod(this, _Me_instances, p_fn).call(this, () => {
        I.ensure();
        try {
          return p(() => {
            var o = b;
            o.b = this, o.f |= V, i(__privateGet(this, _t), () => n, () => a);
          });
        } catch (o) {
          return w(o, __privateGet(this, _e2).parent), null;
        }
      }));
    };
    D(() => {
      var n;
      try {
        n = this.transform_error(e);
      } catch (o) {
        w(o, __privateGet(this, _e2) && __privateGet(this, _e2).parent);
        return;
      }
      n !== null && typeof n == "object" && typeof n.then == "function" ? n.then(u, (o) => w(o, __privateGet(this, _e2) && __privateGet(this, _e2).parent)) : u(n);
    });
  }
}
_t = new WeakMap();
_g = new WeakMap();
_i = new WeakMap();
_h = new WeakMap();
_e2 = new WeakMap();
_a = new WeakMap();
_r = new WeakMap();
_s = new WeakMap();
_n = new WeakMap();
_l = new WeakMap();
_o = new WeakMap();
_d = new WeakMap();
_u = new WeakMap();
_c = new WeakMap();
_f = new WeakMap();
_m = new WeakMap();
_Me_instances = new WeakSet();
b_fn = function() {
  try {
    __privateSet(this, _a, p(() => __privateGet(this, _h).call(this, __privateGet(this, _t))));
  } catch (e) {
    this.error(e);
  }
};
E_fn = function(e) {
  const r = __privateGet(this, _i).failed;
  r && __privateSet(this, _s, p(() => {
    r(__privateGet(this, _t), () => e, () => () => {
    });
  }));
};
T_fn = function() {
  const e = __privateGet(this, _i).pending;
  e && (this.is_pending = true, __privateSet(this, _r, p(() => e(__privateGet(this, _t)))), D(() => {
    var r = __privateSet(this, _n, document.createDocumentFragment()), i = U();
    r.append(i), __privateSet(this, _a, __privateMethod(this, _Me_instances, p_fn).call(this, () => (I.ensure(), p(() => __privateGet(this, _h).call(this, i))))), __privateGet(this, _o) === 0 && (__privateGet(this, _t).before(r), __privateSet(this, _n, null), k(__privateGet(this, _r), () => {
      __privateSet(this, _r, null);
    }), __privateMethod(this, _Me_instances, __fn).call(this));
  }));
};
v_fn = function() {
  try {
    if (this.is_pending = this.has_pending_snippet(), __privateSet(this, _o, 0), __privateSet(this, _l, 0), __privateSet(this, _a, p(() => {
      __privateGet(this, _h).call(this, __privateGet(this, _t));
    })), __privateGet(this, _o) > 0) {
      var e = __privateSet(this, _n, document.createDocumentFragment());
      oe(__privateGet(this, _a), e);
      const r = __privateGet(this, _i).pending;
      __privateSet(this, _r, p(() => r(__privateGet(this, _t))));
    } else __privateMethod(this, _Me_instances, __fn).call(this);
  } catch (r) {
    this.error(r);
  }
};
__fn = function() {
  this.is_pending = false;
  for (const e of __privateGet(this, _u)) L(e, he), j(e);
  for (const e of __privateGet(this, _c)) L(e, le), j(e);
  __privateGet(this, _u).clear(), __privateGet(this, _c).clear();
};
p_fn = function(e) {
  var r = b, i = Q, f = X;
  N(__privateGet(this, _e2)), A(__privateGet(this, _e2)), z(__privateGet(this, _e2).ctx);
  try {
    return e();
  } catch (s) {
    return ue(s), null;
  } finally {
    N(r), A(i), z(f);
  }
};
y_fn = function(e) {
  var _a2;
  if (!this.has_pending_snippet()) {
    this.parent && __privateMethod(_a2 = this.parent, _Me_instances, y_fn).call(_a2, e);
    return;
  }
  __privateSet(this, _o, __privateGet(this, _o) + e), __privateGet(this, _o) === 0 && (__privateMethod(this, _Me_instances, __fn).call(this), __privateGet(this, _r) && k(__privateGet(this, _r), () => {
    __privateSet(this, _r, null);
  }), __privateGet(this, _n) && (__privateGet(this, _t).before(__privateGet(this, _n)), __privateSet(this, _n, null)));
};
const He = ["touchstart", "touchmove"];
function Be(t) {
  return He.includes(t);
}
const T = /* @__PURE__ */ Symbol("events"), ee = /* @__PURE__ */ new Set(), M = /* @__PURE__ */ new Set();
function Le(t, e, r) {
  (e[T] ?? (e[T] = {}))[t] = r;
}
function je(t) {
  for (var e = 0; e < t.length; e++) ee.add(t[e]);
  for (var r of M) r(t);
}
let J = null;
function W(t) {
  var _a2, _b;
  var e = this, r = e.ownerDocument, i = t.type, f = ((_a2 = t.composedPath) == null ? void 0 : _a2.call(t)) || [], s = f[0] || t.target;
  J = t;
  var a = 0, u = J === t && t[T];
  if (u) {
    var n = f.indexOf(u);
    if (n !== -1 && (e === document || e === window)) {
      t[T] = e;
      return;
    }
    var o = f.indexOf(e);
    if (o === -1) return;
    n <= o && (a = n);
  }
  if (s = f[a] || t.target, s !== e) {
    be(t, "currentTarget", { configurable: true, get() {
      return s || r;
    } });
    var y = Q, E = b;
    A(null), N(null);
    try {
      for (var _, l = []; s !== null; ) {
        var h = s.assignedSlot || s.parentNode || s.host || null;
        try {
          var d = (_b = s[T]) == null ? void 0 : _b[i];
          d != null && (!s.disabled || t.target === s) && d.call(s, t);
        } catch (c) {
          _ ? l.push(c) : _ = c;
        }
        if (t.cancelBubble || h === e || h === null) break;
        s = h;
      }
      if (_) {
        for (let c of l) queueMicrotask(() => {
          throw c;
        });
        throw _;
      }
    } finally {
      t[T] = e, delete t.currentTarget, A(y), N(E);
    }
  }
}
let $ = true;
function ze(t, e) {
  var r = e == null ? "" : typeof e == "object" ? e + "" : e;
  r !== (t.__t ?? (t.__t = t.nodeValue)) && (t.__t = r, t.nodeValue = r + "");
}
function Pe(t, e) {
  return te(t, e);
}
function Je(t, e) {
  Y(), e.intro = e.intro ?? false;
  const r = e.target, i = v, f = g;
  try {
    for (var s = Ee(r); s && (s.nodeType !== Z || s.data !== Te); ) s = we(s);
    if (!s) throw C;
    R(true), O(s);
    const a = te(t, { ...e, anchor: s });
    return R(false), a;
  } catch (a) {
    if (a instanceof Error && a.message.split(`
`).some((u) => u.startsWith("https://svelte.dev/e/"))) throw a;
    return a !== C && console.warn("Failed to hydrate: ", a), e.recover === false && Re(), Y(), Se(r), R(false), Pe(t, e);
  } finally {
    R(i), O(f);
  }
}
const S = /* @__PURE__ */ new Map();
function te(t, { target: e, anchor: r, props: i = {}, events: f, context: s, intro: a = true, transformError: u }) {
  Y();
  var n = void 0, o = De(() => {
    var y = r ?? e.appendChild(U());
    Ce(y, { pending: () => {
    } }, (l) => {
      Ae({});
      var h = X;
      if (s && (h.c = s), f && (i.$$events = f), v && ke(l, null), $ = a, n = t(l, i) || {}, $ = true, v && (b.nodes.end = g, g === null || g.nodeType !== Z || g.data !== Oe)) throw Fe(), C;
      Ie();
    }, u);
    var E = /* @__PURE__ */ new Set(), _ = (l) => {
      for (var h = 0; h < l.length; h++) {
        var d = l[h];
        if (!E.has(d)) {
          E.add(d);
          var c = Be(d);
          for (const F of [e, document]) {
            var m = S.get(F);
            m === void 0 && (m = /* @__PURE__ */ new Map(), S.set(F, m));
            var B = m.get(d);
            B === void 0 ? (F.addEventListener(d, W, { passive: c }), m.set(d, 1)) : m.set(d, B + 1);
          }
        }
      }
    };
    return _(Ne(ee)), M.add(_), () => {
      var _a2;
      for (var l of E) for (const c of [e, document]) {
        var h = S.get(c), d = h.get(l);
        --d == 0 ? (c.removeEventListener(l, W), h.delete(l), h.size === 0 && S.delete(c)) : h.set(l, d);
      }
      M.delete(_), y !== r && ((_a2 = y.parentNode) == null ? void 0 : _a2.removeChild(y));
    };
  });
  return H.set(n, o), n;
}
let H = /* @__PURE__ */ new WeakMap();
function We(t, e) {
  const r = H.get(t);
  return r ? (H.delete(t), r(e)) : Promise.resolve();
}
export {
  $ as a,
  Le as b,
  je as d,
  Je as h,
  Pe as m,
  ze as s,
  We as u
};
