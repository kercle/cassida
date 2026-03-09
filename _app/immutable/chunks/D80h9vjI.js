var _a, _b;
import { a7 as v, a as d, $ as l, a8 as u, a9 as T, aa as p, ab as h, h as i, e as s, ac as E, C as y, ad as g, J as w, ae as N } from "./CWZ4Cf0V.js";
const M = ((_a = globalThis == null ? void 0 : globalThis.window) == null ? void 0 : _a.trustedTypes) && globalThis.window.trustedTypes.createPolicy("svelte-trusted-html", { createHTML: (t) => t });
function b(t) {
  return (M == null ? void 0 : M.createHTML(t)) ?? t;
}
function x(t) {
  var r = v("template");
  return r.innerHTML = b(t.replaceAll("<!>", "<!---->")), r.content;
}
function a(t, r) {
  var e = l;
  e.nodes === null && (e.nodes = { start: t, end: r, a: null, t: null });
}
function C(t, r) {
  var e = (r & p) !== 0, f = (r & h) !== 0, n, _ = !t.startsWith("<!>");
  return () => {
    if (i) return a(s, null), s;
    n === void 0 && (n = x(_ ? t : "<!>" + t), e || (n = u(n)));
    var o = f || T ? document.importNode(n, true) : n.cloneNode(true);
    if (e) {
      var c = u(o), m = o.lastChild;
      a(c, m);
    } else a(o, o);
    return o;
  };
}
function O(t = "") {
  if (!i) {
    var r = d(t + "");
    return a(r, r), r;
  }
  var e = s;
  return e.nodeType !== g ? (e.before(e = d()), w(e)) : N(e), a(e, e), e;
}
function P() {
  if (i) return a(s, null), s;
  var t = document.createDocumentFragment(), r = document.createComment(""), e = d();
  return t.append(r, e), a(r, e), t;
}
function R(t, r) {
  if (i) {
    var e = l;
    ((e.f & E) === 0 || e.nodes.end === null) && (e.nodes.end = s), y();
    return;
  }
  t !== null && t.before(r);
}
const A = "5";
typeof window < "u" && ((_b = window.__svelte ?? (window.__svelte = {})).v ?? (_b.v = /* @__PURE__ */ new Set())).add(A);
export {
  R as a,
  a as b,
  P as c,
  C as f,
  O as t
};
