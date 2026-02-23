import { a as u, f as h } from "../chunks/BINAPQrJ.js";
import { i as g } from "../chunks/BgFLKGoI.js";
import { v, w as l, x as d, y as x, z as a, A as e, B as _ } from "../chunks/YVIW7mlL.js";
import { s as o } from "../chunks/C9qaub46.js";
import { s as $, p } from "../chunks/DeTTMXOr.js";
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
