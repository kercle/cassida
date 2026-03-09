import { c as u, a as o, f as c } from "../chunks/D80h9vjI.js";
import { D as p, E as _, a as E, aP as v, ay as y, h as i, aC as h, aD as l, K as f, J as m, e as g, a8 as T, w as C, x as F } from "../chunks/CWZ4Cf0V.js";
import { B as w } from "../chunks/8tNrr6Dq.js";
import { s as D, __tla as __tla_0 } from "../chunks/IBaAlioH.js";
let B;
let __tla = Promise.all([
  (() => {
    try {
      return __tla_0;
    } catch {
    }
  })()
]).then(async () => {
  function R(s, n, ...t) {
    var r = new w(s);
    p(() => {
      const a = n() ?? null;
      r.ensure(a, a && ((e) => a(e, ...t)));
    }, _);
  }
  function b(s, n) {
    let t = null, r = i;
    var a;
    if (i) {
      t = g;
      for (var e = T(document.head); e !== null && (e.nodeType !== h || e.data !== s); ) e = l(e);
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
  const x = "" + new URL("../assets/favicon.dfyh6Zu5.svg", import.meta.url).href;
  var N = c('<link rel="icon"/>');
  B = function(s, n) {
    var t = u();
    b("12qhfyh", (a) => {
      var e = N();
      F(() => D(e, "href", x)), o(a, e);
    });
    var r = C(t);
    R(r, () => n.children), o(s, t);
  };
});
export {
  __tla,
  B as component
};
