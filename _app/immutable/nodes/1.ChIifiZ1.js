import { a as u, f as h } from "../chunks/Dqshxp8n.js";
import { i as g } from "../chunks/C1A8SCGe.js";
import { v, w as l, x as d, y as x, z as a, A as e, B as _ } from "../chunks/BfYh-9ES.js";
import { s as o } from "../chunks/DW5xhVcM.js";
import { s as $, p } from "../chunks/BiWuwCQ8.js";
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
