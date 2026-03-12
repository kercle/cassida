import { a as u, f as h } from "../chunks/DkayNNiV.js";
import { i as g } from "../chunks/D_5bwYY3.js";
import { v, w as l, x as d, y as x, z as a, A as e, B as _ } from "../chunks/Bp716flw.js";
import { s as o } from "../chunks/BwSswmFG.js";
import { s as $, p } from "../chunks/CwYOBcx7.js";
const k = { get error() {
  return p.error;
}, get status() {
  return p.status;
} };
$.updated.check;
const m = k;
var b = h("<h1> </h1> <p> </p>", 1);
function E(i, f) {
  v(f, false), g();
  var t = b(), r = l(t), n = a(r, true);
  e(r);
  var s = _(r, 2), c = a(s, true);
  e(s), d(() => {
    var _a;
    o(n, m.status), o(c, (_a = m.error) == null ? void 0 : _a.message);
  }), u(i, t), x();
}
export {
  E as component
};
