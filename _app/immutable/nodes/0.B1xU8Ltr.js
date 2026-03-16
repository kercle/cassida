import { c as u, a as o, f as c } from "../chunks/DkjPntfO.js";
import { D as p, E, a as _, aQ as v, ay as y, h as i, aD as h, aE as l, K as f, J as m, e as g, a8 as T, w as F, x as w } from "../chunks/BfX1JZpI.js";
import { B as C } from "../chunks/67kIHg_j.js";
import { s as D, __tla as __tla_0 } from "../chunks/C9mE4gcY.js";
let O;
let __tla = Promise.all([
  (() => {
    try {
      return __tla_0;
    } catch {
    }
  })()
]).then(async () => {
  function R(s, n, ...t) {
    var r = new C(s);
    p(() => {
      const a = n() ?? null;
      r.ensure(a, a && ((e) => a(e, ...t)));
    }, E);
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
    i || (a = document.head.appendChild(_()));
    try {
      p(() => n(a), v | y);
    } finally {
      r && (f(true), m(t));
    }
  }
  const x = "" + new URL("../assets/favicon.dfyh6Zu5.svg", import.meta.url).href;
  var N = c('<link rel="icon"/>');
  O = function(s, n) {
    var t = u();
    b("12qhfyh", (a) => {
      var e = N();
      w(() => D(e, "href", x)), o(a, e);
    });
    var r = F(t);
    R(r, () => n.children), o(s, t);
  };
});
export {
  __tla,
  O as component
};
