import { h as R, C as U, D as $, E as w, F as x, H as C, G as F, I as H, J as q, K as A, L as G, M as Z, i as N, N as y, S as D, O as z, P as J, Q as O, R as K, T as Q, U as V, l as g, V as L, W, X, Y as j, Z as m, _ as p, $ as k, a0 as ee, a1 as re, t as ae, a2 as se, a3 as ne, o as ie, a4 as te, a5 as ue, a6 as fe } from "./Bp716flw.js";
import { B as _e } from "./DRCcKVOS.js";
function ve(e, r, n = false) {
  R && U();
  var a = new _e(e), i = n ? w : 0;
  function s(l, u) {
    if (R) {
      const _ = x(e);
      var f;
      if (_ === C ? f = 0 : _ === F ? f = false : f = parseInt(_.substring(1)), l !== f) {
        var c = H();
        q(c), a.anchor = c, A(false), a.ensure(l, u), A(true);
        return;
      }
    }
    a.ensure(l, u);
  }
  $(() => {
    var l = false;
    r((u, f = 0) => {
      l = true, s(f, u);
    }), l || s(false, null);
  }, i);
}
function h(e, r) {
  return e === r || (e == null ? void 0 : e[D]) === r;
}
function be(e = {}, r, n, a) {
  return G(() => {
    var i, s;
    return Z(() => {
      i = s, s = [], N(() => {
        e !== n(...s) && (r(e, ...s), i && h(n(...i), e) && r(null, ...i));
      });
    }), () => {
      y(() => {
        s && h(n(...s), e) && r(null, ...s);
      });
    };
  }), e;
}
let S = false, E = /* @__PURE__ */ Symbol();
function oe(e, r, n) {
  const a = n[r] ?? (n[r] = { store: null, source: K(void 0), unsubscribe: O });
  if (a.store !== e && !(E in n)) if (a.unsubscribe(), a.store = e ?? null, e == null) a.source.v = void 0, a.unsubscribe = O;
  else {
    var i = true;
    a.unsubscribe = Q(e, (s) => {
      i ? a.source.v = s : L(a.source, s);
    }), i = false;
  }
  return e && E in n ? V(e) : g(a.source);
}
function Se() {
  const e = {};
  function r() {
    z(() => {
      for (var n in e) e[n].unsubscribe();
      J(e, E, { enumerable: false, value: true });
    });
  }
  return [e, r];
}
function le(e) {
  var r = S;
  try {
    return S = false, [e(), S];
  } finally {
    S = r;
  }
}
function ge(e, r, n, a) {
  var _a;
  var i = !ae || (n & se) !== 0, s = (n & re) !== 0, l = (n & ue) !== 0, u = a, f = true, c = () => (f && (f = false, u = l ? N(a) : a), u), _;
  if (s) {
    var Y = D in e || fe in e;
    _ = ((_a = W(e, r)) == null ? void 0 : _a.set) ?? (Y && r in e ? (t) => e[r] = t : void 0);
  }
  var v, I = false;
  s ? [v, I] = le(() => e[r]) : v = e[r], v === void 0 && a !== void 0 && (v = c(), _ && (i && X(), _(v)));
  var d;
  if (i ? d = () => {
    var t = e[r];
    return t === void 0 ? c() : (f = true, t);
  } : d = () => {
    var t = e[r];
    return t !== void 0 && (u = void 0), t === void 0 ? u : t;
  }, i && (n & j) === 0) return d;
  if (_) {
    var B = e.$$legacy;
    return (function(t, o) {
      return arguments.length > 0 ? ((!i || !o || B || I) && _(o ? d() : t), t) : d();
    });
  }
  var T = false, b = ((n & ne) !== 0 ? ie : te)(() => (T = false, d()));
  s && g(b);
  var M = k;
  return (function(t, o) {
    if (arguments.length > 0) {
      const P = o ? g(b) : i && s ? m(t) : t;
      return L(b, P), T = true, u !== void 0 && (u = P), t;
    }
    return p && T || (M.f & ee) !== 0 ? b.v : g(b);
  });
}
export {
  oe as a,
  be as b,
  ve as i,
  ge as p,
  Se as s
};
