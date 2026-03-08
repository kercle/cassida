import { c as u, a as o, f as c } from "../chunks/BINAPQrJ.js";
import { C as p, E as _, a as E, aP as v, ay as y, h as i, aC as h, aD as l, K as f, J as m, e as g, a8 as C, w as T, x as F } from "../chunks/YVIW7mlL.js";
import { B as w } from "../chunks/Drrjwcid.js";
import { s as R, __tla as __tla_0 } from "../chunks/PDCHMTvm.js";
let B;
let __tla = Promise.all([
  (() => {
    try {
      return __tla_0;
    } catch {
    }
  })()
]).then(async () => {
  function b(s, n, ...t) {
    var r = new w(s);
    p(() => {
      const a = n() ?? null;
      r.ensure(a, a && ((e) => a(e, ...t)));
    }, _);
  }
  function x(s, n) {
    let t = null, r = i;
    var a;
    if (i) {
      t = g;
      for (var e = C(document.head); e !== null && (e.nodeType !== h || e.data !== s); ) e = l(e);
      if (e === null) f(false);
      else {
        var d = l(e);
        e.remove(), m(d);
      }
    }
    i || (a = document.head.appendChild(E()));
    try {
      p(() => n(a), v | y);
    } finally {
      r && (f(true), m(t));
    }
  }
  const D = "" + new URL("../assets/favicon.dfyh6Zu5.svg", import.meta.url).href;
  var N = c('<link rel="icon"/>');
  B = function(s, n) {
    var t = u();
    x("12qhfyh", (a) => {
      var e = N();
      F(() => R(e, "href", D)), o(a, e);
    });
    var r = T(t);
    b(r, () => n.children), o(s, t);
  };
});
export {
  __tla,
  B as component
};
