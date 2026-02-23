import { h as R, C as U, D as $, E as q, F as w, H as x, G as C, I as F, J as H, K as A, L as G, M as Z, g as N, N as y, S as D, O as z, P as J, Q as O, R as K, T as Q, U as V, n as g, V as L, W, X, Y as j, Z as m, _ as p, $ as k, a0 as ee, a1 as re, l as ae, a2 as ne, a3 as se, q as ie, a4 as te, a5 as ue, a6 as fe } from "./BfYh-9ES.js";
import { B as _e } from "./Dptbz3CV.js";
function ve(e, r, s = false) {
  R && U();
  var a = new _e(e), i = s ? q : 0;
  function n(l, u) {
    if (R) {
      const _ = w(e);
      var f;
      if (_ === x ? f = 0 : _ === C ? f = false : f = parseInt(_.substring(1)), l !== f) {
        var c = F();
        H(c), a.anchor = c, A(false), a.ensure(l, u), A(true);
        return;
      }
    }
    a.ensure(l, u);
  }
  $(() => {
    var l = false;
    r((u, f = 0) => {
      l = true, n(f, u);
    }), l || n(false, null);
  }, i);
}
function h(e, r) {
  return e === r || (e == null ? void 0 : e[D]) === r;
}
function be(e = {}, r, s, a) {
  return G(() => {
    var i, n;
    return Z(() => {
      i = n, n = [], N(() => {
        e !== s(...n) && (r(e, ...n), i && h(s(...i), e) && r(null, ...i));
      });
    }), () => {
      y(() => {
        n && h(s(...n), e) && r(null, ...n);
      });
    };
  }), e;
}
let S = false, E = /* @__PURE__ */ Symbol();
function oe(e, r, s) {
  const a = s[r] ?? (s[r] = { store: null, source: K(void 0), unsubscribe: O });
  if (a.store !== e && !(E in s)) if (a.unsubscribe(), a.store = e ?? null, e == null) a.source.v = void 0, a.unsubscribe = O;
  else {
    var i = true;
    a.unsubscribe = Q(e, (n) => {
      i ? a.source.v = n : L(a.source, n);
    }), i = false;
  }
  return e && E in s ? V(e) : g(a.source);
}
function Se() {
  const e = {};
  function r() {
    z(() => {
      for (var s in e) e[s].unsubscribe();
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
function ge(e, r, s, a) {
  var _a;
  var i = !ae || (s & ne) !== 0, n = (s & re) !== 0, l = (s & ue) !== 0, u = a, f = true, c = () => (f && (f = false, u = l ? N(a) : a), u), _;
  if (n) {
    var Y = D in e || fe in e;
    _ = ((_a = W(e, r)) == null ? void 0 : _a.set) ?? (Y && r in e ? (t) => e[r] = t : void 0);
  }
  var v, I = false;
  n ? [v, I] = le(() => e[r]) : v = e[r], v === void 0 && a !== void 0 && (v = c(), _ && (i && X(), _(v)));
  var d;
  if (i ? d = () => {
    var t = e[r];
    return t === void 0 ? c() : (f = true, t);
  } : d = () => {
    var t = e[r];
    return t !== void 0 && (u = void 0), t === void 0 ? u : t;
  }, i && (s & j) === 0) return d;
  if (_) {
    var B = e.$$legacy;
    return (function(t, o) {
      return arguments.length > 0 ? ((!i || !o || B || I) && _(o ? d() : t), t) : d();
    });
  }
  var T = false, b = ((s & se) !== 0 ? ie : te)(() => (T = false, d()));
  n && g(b);
  var M = k;
  return (function(t, o) {
    if (arguments.length > 0) {
      const P = o ? g(b) : i && n ? m(t) : t;
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
