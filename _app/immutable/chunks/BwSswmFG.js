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
var _t, _g, _i, _h, _e2, _a, _r, _s, _n, _l, _o, _d, _c, _u, _f, _b, _Ce_instances, m_fn, E_fn, w_fn, v_fn, __fn, p_fn, y_fn;
import { af as te, l as $, M as re, i as se, ag as P, N as T, ah as G, e as g, h as v, $ as m, ai as V, D as ie, C as ae, G as ne, aj as W, b as p, a as K, ak as I, p as k, m as fe, al as q, am as oe, an as j, ao as he, ap as le, aq as N, ar as A, as as z, at as de, au as U, f as Q, av as ce, d as x, J as O, aw as ue, I as _e, ax as R, E as pe, ay as ge, az as ve, aA as ye, P as be, O as me, aB as Ee, aC as M, a8 as we, aD as X, H as Te, aE as Re, aF as L, K as S, aG as Se, aH as De, aI as Ne, aJ as Ae, v as Oe, aK as Fe, aL as Ie, y as ke } from "./Bp716flw.js";
import { b as xe } from "./DkayNNiV.js";
function Me(t) {
  let e = 0, r = G(0), i;
  return () => {
    te() && ($(r), re(() => (e === 0 && (i = se(() => t(() => P(r)))), e += 1, () => {
      T(() => {
        e -= 1, e === 0 && (i == null ? void 0 : i(), i = void 0, P(r));
      });
    })));
  };
}
var Le = pe | ge;
function Ye(t, e, r, i) {
  new Ce(t, e, r, i);
}
class Ce {
  constructor(e, r, i, a) {
    __privateAdd(this, _Ce_instances);
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
    __privateAdd(this, _c, /* @__PURE__ */ new Set());
    __privateAdd(this, _u, /* @__PURE__ */ new Set());
    __privateAdd(this, _f, null);
    __privateAdd(this, _b, Me(() => (__privateSet(this, _f, G(__privateGet(this, _l))), () => {
      __privateSet(this, _f, null);
    })));
    var _a2;
    __privateSet(this, _t, e), __privateSet(this, _i, r), __privateSet(this, _h, (s) => {
      var n = m;
      n.b = this, n.f |= V, i(s);
    }), this.parent = m.b, this.transform_error = a ?? ((_a2 = this.parent) == null ? void 0 : _a2.transform_error) ?? ((s) => s), __privateSet(this, _e2, ie(() => {
      if (v) {
        const s = __privateGet(this, _g);
        ae();
        const n = s.data === ne;
        if (s.data.startsWith(W)) {
          const f = JSON.parse(s.data.slice(W.length));
          __privateMethod(this, _Ce_instances, E_fn).call(this, f);
        } else n ? __privateMethod(this, _Ce_instances, w_fn).call(this) : __privateMethod(this, _Ce_instances, m_fn).call(this);
      } else __privateMethod(this, _Ce_instances, v_fn).call(this);
    }, Le)), v && __privateSet(this, _t, g);
  }
  defer_effect(e) {
    le(e, __privateGet(this, _c), __privateGet(this, _u));
  }
  is_rendered() {
    return !this.is_pending && (!this.parent || this.parent.is_rendered());
  }
  has_pending_snippet() {
    return !!__privateGet(this, _i).pending;
  }
  update_pending_count(e) {
    __privateMethod(this, _Ce_instances, y_fn).call(this, e), __privateSet(this, _l, __privateGet(this, _l) + e), !(!__privateGet(this, _f) || __privateGet(this, _d)) && (__privateSet(this, _d, true), T(() => {
      __privateSet(this, _d, false), __privateGet(this, _f) && ce(__privateGet(this, _f), __privateGet(this, _l));
    }));
  }
  get_effect_pending() {
    return __privateGet(this, _b).call(this), $(__privateGet(this, _f));
  }
  error(e) {
    var r = __privateGet(this, _i).onerror;
    let i = __privateGet(this, _i).failed;
    if (!r && !i) throw e;
    __privateGet(this, _a) && (x(__privateGet(this, _a)), __privateSet(this, _a, null)), __privateGet(this, _r) && (x(__privateGet(this, _r)), __privateSet(this, _r, null)), __privateGet(this, _s) && (x(__privateGet(this, _s)), __privateSet(this, _s, null)), v && (O(__privateGet(this, _g)), ue(), O(_e()));
    var a = false, s = false;
    const n = () => {
      if (a) {
        ye();
        return;
      }
      a = true, s && ve(), __privateGet(this, _s) !== null && k(__privateGet(this, _s), () => {
        __privateSet(this, _s, null);
      }), __privateMethod(this, _Ce_instances, p_fn).call(this, () => {
        I.ensure(), __privateMethod(this, _Ce_instances, v_fn).call(this);
      });
    }, c = (f) => {
      try {
        s = true, r == null ? void 0 : r(f, n), s = false;
      } catch (o) {
        R(o, __privateGet(this, _e2) && __privateGet(this, _e2).parent);
      }
      i && __privateSet(this, _s, __privateMethod(this, _Ce_instances, p_fn).call(this, () => {
        I.ensure();
        try {
          return p(() => {
            var o = m;
            o.b = this, o.f |= V, i(__privateGet(this, _t), () => f, () => n);
          });
        } catch (o) {
          return R(o, __privateGet(this, _e2).parent), null;
        }
      }));
    };
    T(() => {
      var f;
      try {
        f = this.transform_error(e);
      } catch (o) {
        R(o, __privateGet(this, _e2) && __privateGet(this, _e2).parent);
        return;
      }
      f !== null && typeof f == "object" && typeof f.then == "function" ? f.then(c, (o) => R(o, __privateGet(this, _e2) && __privateGet(this, _e2).parent)) : c(f);
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
_c = new WeakMap();
_u = new WeakMap();
_f = new WeakMap();
_b = new WeakMap();
_Ce_instances = new WeakSet();
m_fn = function() {
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
w_fn = function() {
  const e = __privateGet(this, _i).pending;
  e && (this.is_pending = true, __privateSet(this, _r, p(() => e(__privateGet(this, _t)))), T(() => {
    var r = __privateSet(this, _n, document.createDocumentFragment()), i = K();
    r.append(i), __privateSet(this, _a, __privateMethod(this, _Ce_instances, p_fn).call(this, () => (I.ensure(), p(() => __privateGet(this, _h).call(this, i))))), __privateGet(this, _o) === 0 && (__privateGet(this, _t).before(r), __privateSet(this, _n, null), k(__privateGet(this, _r), () => {
      __privateSet(this, _r, null);
    }), __privateMethod(this, _Ce_instances, __fn).call(this));
  }));
};
v_fn = function() {
  try {
    if (this.is_pending = this.has_pending_snippet(), __privateSet(this, _o, 0), __privateSet(this, _l, 0), __privateSet(this, _a, p(() => {
      __privateGet(this, _h).call(this, __privateGet(this, _t));
    })), __privateGet(this, _o) > 0) {
      var e = __privateSet(this, _n, document.createDocumentFragment());
      fe(__privateGet(this, _a), e);
      const r = __privateGet(this, _i).pending;
      __privateSet(this, _r, p(() => r(__privateGet(this, _t))));
    } else __privateMethod(this, _Ce_instances, __fn).call(this);
  } catch (r) {
    this.error(r);
  }
};
__fn = function() {
  this.is_pending = false;
  for (const e of __privateGet(this, _c)) q(e, oe), j(e);
  for (const e of __privateGet(this, _u)) q(e, he), j(e);
  __privateGet(this, _c).clear(), __privateGet(this, _u).clear();
};
p_fn = function(e) {
  var r = m, i = U, a = Q;
  N(__privateGet(this, _e2)), A(__privateGet(this, _e2)), z(__privateGet(this, _e2).ctx);
  try {
    return e();
  } catch (s) {
    return de(s), null;
  } finally {
    N(r), A(i), z(a);
  }
};
y_fn = function(e) {
  var _a2;
  if (!this.has_pending_snippet()) {
    this.parent && __privateMethod(_a2 = this.parent, _Ce_instances, y_fn).call(_a2, e);
    return;
  }
  __privateSet(this, _o, __privateGet(this, _o) + e), __privateGet(this, _o) === 0 && (__privateMethod(this, _Ce_instances, __fn).call(this), __privateGet(this, _r) && k(__privateGet(this, _r), () => {
    __privateSet(this, _r, null);
  }), __privateGet(this, _n) && (__privateGet(this, _t).before(__privateGet(this, _n)), __privateSet(this, _n, null)));
};
const He = ["touchstart", "touchmove"];
function Be(t) {
  return He.includes(t);
}
const w = /* @__PURE__ */ Symbol("events"), Z = /* @__PURE__ */ new Set(), Y = /* @__PURE__ */ new Set();
function Pe(t, e, r, i = {}) {
  function a(s) {
    if (i.capture || C.call(e, s), !s.cancelBubble) return Ee(() => r == null ? void 0 : r.call(this, s));
  }
  return t.startsWith("pointer") || t.startsWith("touch") || t === "wheel" ? T(() => {
    e.addEventListener(t, a, i);
  }) : e.addEventListener(t, a, i), a;
}
function je(t, e, r, i, a) {
  var s = { capture: i, passive: a }, n = Pe(t, e, r, s);
  (e === document.body || e === window || e === document || e instanceof HTMLMediaElement) && me(() => {
    e.removeEventListener(t, n, s);
  });
}
function ze(t, e, r) {
  (e[w] ?? (e[w] = {}))[t] = r;
}
function Je(t) {
  for (var e = 0; e < t.length; e++) Z.add(t[e]);
  for (var r of Y) r(t);
}
let J = null;
function C(t) {
  var _a2, _b2;
  var e = this, r = e.ownerDocument, i = t.type, a = ((_a2 = t.composedPath) == null ? void 0 : _a2.call(t)) || [], s = a[0] || t.target;
  J = t;
  var n = 0, c = J === t && t[w];
  if (c) {
    var f = a.indexOf(c);
    if (f !== -1 && (e === document || e === window)) {
      t[w] = e;
      return;
    }
    var o = a.indexOf(e);
    if (o === -1) return;
    f <= o && (n = f);
  }
  if (s = a[n] || t.target, s !== e) {
    be(t, "currentTarget", { configurable: true, get() {
      return s || r;
    } });
    var y = U, E = m;
    A(null), N(null);
    try {
      for (var _, l = []; s !== null; ) {
        var h = s.assignedSlot || s.parentNode || s.host || null;
        try {
          var d = (_b2 = s[w]) == null ? void 0 : _b2[i];
          d != null && (!s.disabled || t.target === s) && d.call(s, t);
        } catch (u) {
          _ ? l.push(u) : _ = u;
        }
        if (t.cancelBubble || h === e || h === null) break;
        s = h;
      }
      if (_) {
        for (let u of l) queueMicrotask(() => {
          throw u;
        });
        throw _;
      }
    } finally {
      t[w] = e, delete t.currentTarget, A(y), N(E);
    }
  }
}
function $e(t, e) {
  var r = e == null ? "" : typeof e == "object" ? e + "" : e;
  r !== (t.__t ?? (t.__t = t.nodeValue)) && (t.__t = r, t.nodeValue = r + "");
}
function Ve(t, e) {
  return ee(t, e);
}
function Ge(t, e) {
  M(), e.intro = e.intro ?? false;
  const r = e.target, i = v, a = g;
  try {
    for (var s = we(r); s && (s.nodeType !== X || s.data !== Te); ) s = Re(s);
    if (!s) throw L;
    S(true), O(s);
    const n = ee(t, { ...e, anchor: s });
    return S(false), n;
  } catch (n) {
    if (n instanceof Error && n.message.split(`
`).some((c) => c.startsWith("https://svelte.dev/e/"))) throw n;
    return n !== L && console.warn("Failed to hydrate: ", n), e.recover === false && Se(), M(), De(r), S(false), Ve(t, e);
  } finally {
    S(i), O(a);
  }
}
const D = /* @__PURE__ */ new Map();
function ee(t, { target: e, anchor: r, props: i = {}, events: a, context: s, intro: n = true, transformError: c }) {
  M();
  var f = void 0, o = Ne(() => {
    var y = r ?? e.appendChild(K());
    Ye(y, { pending: () => {
    } }, (l) => {
      Oe({});
      var h = Q;
      if (s && (h.c = s), a && (i.$$events = a), v && xe(l, null), f = t(l, i) || {}, v && (m.nodes.end = g, g === null || g.nodeType !== X || g.data !== Fe)) throw Ie(), L;
      ke();
    }, c);
    var E = /* @__PURE__ */ new Set(), _ = (l) => {
      for (var h = 0; h < l.length; h++) {
        var d = l[h];
        if (!E.has(d)) {
          E.add(d);
          var u = Be(d);
          for (const F of [e, document]) {
            var b = D.get(F);
            b === void 0 && (b = /* @__PURE__ */ new Map(), D.set(F, b));
            var B = b.get(d);
            B === void 0 ? (F.addEventListener(d, C, { passive: u }), b.set(d, 1)) : b.set(d, B + 1);
          }
        }
      }
    };
    return _(Ae(Z)), Y.add(_), () => {
      var _a2;
      for (var l of E) for (const u of [e, document]) {
        var h = D.get(u), d = h.get(l);
        --d == 0 ? (u.removeEventListener(l, C), h.delete(l), h.size === 0 && D.delete(u)) : h.set(l, d);
      }
      Y.delete(_), y !== r && ((_a2 = y.parentNode) == null ? void 0 : _a2.removeChild(y));
    };
  });
  return H.set(f, o), f;
}
let H = /* @__PURE__ */ new WeakMap();
function Ke(t, e) {
  const r = H.get(t);
  return r ? (H.delete(t), r(e)) : Promise.resolve();
}
export {
  ze as a,
  Je as d,
  je as e,
  Ge as h,
  Ve as m,
  $e as s,
  Ke as u
};
