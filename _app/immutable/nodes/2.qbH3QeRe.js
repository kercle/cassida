import { b as br, a as z0, f as C0 } from "../chunks/BINAPQrJ.js";
import { i as Wt } from "../chunks/BgFLKGoI.js";
import { a as ct, C as y1, h as S0, J as Ue, a8 as $e, D as qt, n as p0, F as x1, G as w1, I as yr, K as dt, e as me, aC as oa, aJ as k1, av as xr, c as he, aX as Z0, b as Et, aY as S1, s as M1, a4 as ua, aZ as z1, aI as Zt, a_ as A1, a$ as T1, R as ha, ah as wr, b0 as B1, r as ma, p as ca, b1 as ft, b2 as C1, aG as D1, d as N1, aD as da, x as Se, $ as fa, b3 as q1, aK as E1, aE as R1, a7 as I1, b4 as O1, b5 as F1, E as H1, b6 as L1, ac as P1, L as V1, g as A0, b7 as G1, b8 as U1, b9 as $1, ba as Y1, N as X1, Q as ye, bb as va, bc as W1, aM as pa, M as Z1, bd as j1, v as Qe, y as et, B as de, z as i0, A as a0, V as Rt, aN as K1, o as ce, be as _1, bf as J1, w as Q1 } from "../chunks/YVIW7mlL.js";
import { p as ga, i as It, b as en, s as tn, a as rn } from "../chunks/LnvLu4n2.js";
import { a as an, d as nn, b as sn, s as kr } from "../chunks/C9qaub46.js";
import { r as ln, a as ba, s as on, __tla as __tla_0 } from "../chunks/DQaNyVij.js";
let X4;
let __tla = Promise.all([
  (() => {
    try {
      return __tla_0;
    } catch {
    }
  })()
]).then(async () => {
  function un(r, e) {
    return e;
  }
  function hn(r, e, t) {
    for (var a = [], n = e.length, i, o = e.length, u = 0; u < n; u++) {
      let b = e[u];
      ca(b, () => {
        if (i) {
          if (i.pending.delete(b), i.done.add(b), i.pending.size === 0) {
            var y = r.outrogroups;
            Ot(Zt(i.done)), y.delete(i), y.size === 0 && (r.outrogroups = null);
          }
        } else o -= 1;
      }, false);
    }
    if (o === 0) {
      var m = a.length === 0 && t !== null;
      if (m) {
        var d = t, p = d.parentNode;
        D1(p), p.append(d), r.items.clear();
      }
      Ot(e, !m);
    } else i = {
      pending: new Set(e),
      done: /* @__PURE__ */ new Set()
    }, (r.outrogroups ?? (r.outrogroups = /* @__PURE__ */ new Set())).add(i);
  }
  function Ot(r, e = true) {
    for (var t = 0; t < r.length; t++) N1(r[t], e);
  }
  var Sr;
  function mn(r, e, t, a, n, i = null) {
    var o = r, u = /* @__PURE__ */ new Map();
    {
      var m = r;
      o = S0 ? Ue($e(m)) : m.appendChild(ct());
    }
    S0 && qt();
    var d = null, p = ua(() => {
      var z = t();
      return z1(z) ? z : z == null ? [] : Zt(z);
    }), b, y = true;
    function x() {
      S.fallback = d, cn(S, b, o, e, a), d !== null && (b.length === 0 ? (d.f & Z0) === 0 ? ma(d) : (d.f ^= Z0, we(d, null, o)) : ca(d, () => {
        d = null;
      }));
    }
    var k = y1(() => {
      b = p0(p);
      var z = b.length;
      let B = false;
      if (S0) {
        var N = x1(o) === w1;
        N !== (z === 0) && (o = yr(), Ue(o), dt(false), B = true);
      }
      for (var E = /* @__PURE__ */ new Set(), H = he, O = M1(), L = 0; L < z; L += 1) {
        S0 && me.nodeType === oa && me.data === k1 && (o = me, B = true, dt(false));
        var V = b[L], U = a(V, L), P = y ? null : u.get(U);
        P ? (P.v && xr(P.v, V), P.i && xr(P.i, L), O && H.unskip_effect(P.e)) : (P = dn(u, y ? o : Sr ?? (Sr = ct()), V, U, L, n, e, t), y || (P.e.f |= Z0), u.set(U, P)), E.add(U);
      }
      if (z === 0 && i && !d && (y ? d = Et(() => i(o)) : (d = Et(() => i(Sr ?? (Sr = ct()))), d.f |= Z0)), z > E.size && S1(), S0 && z > 0 && Ue(yr()), !y) if (O) {
        for (const [x0, r0] of u) E.has(x0) || H.skip_effect(r0.e);
        H.oncommit(x), H.ondiscard(() => {
        });
      } else x();
      B && dt(true), p0(p);
    }), S = {
      effect: k,
      items: u,
      outrogroups: null,
      fallback: d
    };
    y = false, S0 && (o = me);
  }
  function xe(r) {
    for (; r !== null && (r.f & C1) === 0; ) r = r.next;
    return r;
  }
  function cn(r, e, t, a, n) {
    var _a2;
    var i = e.length, o = r.items, u = xe(r.effect.first), m, d = null, p = [], b = [], y, x, k, S;
    for (S = 0; S < i; S += 1) {
      if (y = e[S], x = n(y, S), k = o.get(x).e, r.outrogroups !== null) for (const U of r.outrogroups) U.pending.delete(k), U.done.delete(k);
      if ((k.f & Z0) !== 0) if (k.f ^= Z0, k === u) we(k, null, t);
      else {
        var z = d ? d.next : u;
        k === r.effect.last && (r.effect.last = k.prev), k.prev && (k.prev.next = k.next), k.next && (k.next.prev = k.prev), Y0(r, d, k), Y0(r, k, z), we(k, z, t), d = k, p = [], b = [], u = xe(d.next);
        continue;
      }
      if ((k.f & ft) !== 0 && ma(k), k !== u) {
        if (m !== void 0 && m.has(k)) {
          if (p.length < b.length) {
            var B = b[0], N;
            d = B.prev;
            var E = p[0], H = p[p.length - 1];
            for (N = 0; N < p.length; N += 1) we(p[N], B, t);
            for (N = 0; N < b.length; N += 1) m.delete(b[N]);
            Y0(r, E.prev, H.next), Y0(r, d, E), Y0(r, H, B), u = B, d = H, S -= 1, p = [], b = [];
          } else m.delete(k), we(k, u, t), Y0(r, k.prev, k.next), Y0(r, k, d === null ? r.effect.first : d.next), Y0(r, d, k), d = k;
          continue;
        }
        for (p = [], b = []; u !== null && u !== k; ) (m ?? (m = /* @__PURE__ */ new Set())).add(u), b.push(u), u = xe(u.next);
        if (u === null) continue;
      }
      (k.f & Z0) === 0 && p.push(k), d = k, u = xe(k.next);
    }
    if (r.outrogroups !== null) {
      for (const U of r.outrogroups) U.pending.size === 0 && (Ot(Zt(U.done)), (_a2 = r.outrogroups) == null ? void 0 : _a2.delete(U));
      r.outrogroups.size === 0 && (r.outrogroups = null);
    }
    if (u !== null || m !== void 0) {
      var O = [];
      if (m !== void 0) for (k of m) (k.f & ft) === 0 && O.push(k);
      for (; u !== null; ) (u.f & ft) === 0 && u !== r.fallback && O.push(u), u = xe(u.next);
      var L = O.length;
      if (L > 0) {
        var V = i === 0 ? t : null;
        hn(r, O, V);
      }
    }
  }
  function dn(r, e, t, a, n, i, o, u) {
    var m = (o & A1) !== 0 ? (o & T1) === 0 ? ha(t, false, false) : wr(t) : null, d = (o & B1) !== 0 ? wr(n) : null;
    return {
      v: m,
      i: d,
      e: Et(() => (i(e, m ?? t, d ?? n, u), () => {
        r.delete(a);
      }))
    };
  }
  function we(r, e, t) {
    if (r.nodes) for (var a = r.nodes.start, n = r.nodes.end, i = e && (e.f & Z0) === 0 ? e.nodes.start : t; a !== null; ) {
      var o = da(a);
      if (i.before(a), a === n) return;
      a = o;
    }
  }
  function Y0(r, e, t) {
    e === null ? r.effect.first = t : e.next = t, t === null ? r.effect.last = e : t.prev = e;
  }
  function fn(r, e, t = false, a = false, n = false) {
    var i = r, o = "";
    Se(() => {
      var u = fa;
      if (o === (o = e() ?? "")) {
        S0 && qt();
        return;
      }
      if (u.nodes !== null && (q1(u.nodes.start, u.nodes.end), u.nodes = null), o !== "") {
        if (S0) {
          me.data;
          for (var m = qt(), d = m; m !== null && (m.nodeType !== oa || m.data !== ""); ) d = m, m = da(m);
          if (m === null) throw E1(), R1;
          br(me, d), i = Ue(m);
          return;
        }
        var p = t ? O1 : a ? F1 : void 0, b = I1(t ? "svg" : a ? "math" : "template", p);
        b.innerHTML = o;
        var y = t || a ? b : b.content;
        if (br($e(y), y.lastChild), t || a) for (; $e(y); ) i.before($e(y));
        else i.before(y);
      }
    });
  }
  const vn = () => performance.now(), I0 = {
    tick: (r) => requestAnimationFrame(r),
    now: () => vn(),
    tasks: /* @__PURE__ */ new Set()
  };
  function ya() {
    const r = I0.now();
    I0.tasks.forEach((e) => {
      e.c(r) || (I0.tasks.delete(e), e.f());
    }), I0.tasks.size !== 0 && I0.tick(ya);
  }
  function pn(r) {
    let e;
    return I0.tasks.size === 0 && I0.tick(ya), {
      promise: new Promise((t) => {
        I0.tasks.add(e = {
          c: r,
          f: t
        });
      }),
      abort() {
        I0.tasks.delete(e);
      }
    };
  }
  function We(r, e) {
    va(() => {
      r.dispatchEvent(new CustomEvent(e));
    });
  }
  function gn(r) {
    if (r === "float") return "cssFloat";
    if (r === "offset") return "cssOffset";
    if (r.startsWith("--")) return r;
    const e = r.split("-");
    return e.length === 1 ? e[0] : e[0] + e.slice(1).map((t) => t[0].toUpperCase() + t.slice(1)).join("");
  }
  function Mr(r) {
    const e = {}, t = r.split(";");
    for (const a of t) {
      const [n, i] = a.split(":");
      if (!n || i === void 0) break;
      const o = gn(n.trim());
      e[o] = i.trim();
    }
    return e;
  }
  const bn = (r) => r;
  function xa(r, e, t, a) {
    var _a2;
    var n = (r & U1) !== 0, i = (r & $1) !== 0, o = n && i, u = (r & G1) !== 0, m = o ? "both" : n ? "in" : "out", d, p = e.inert, b = e.style.overflow, y, x;
    function k() {
      return va(() => d ?? (d = t()(e, (a == null ? void 0 : a()) ?? {}, {
        direction: m
      })));
    }
    var S = {
      is_global: u,
      in() {
        var _a3;
        if (e.inert = p, !n) {
          x == null ? void 0 : x.abort(), (_a3 = x == null ? void 0 : x.reset) == null ? void 0 : _a3.call(x);
          return;
        }
        i || (y == null ? void 0 : y.abort()), y = Ft(e, k(), x, 1, () => {
          We(e, "introend"), y == null ? void 0 : y.abort(), y = d = void 0, e.style.overflow = b;
        });
      },
      out(E) {
        if (!i) {
          E == null ? void 0 : E(), d = void 0;
          return;
        }
        e.inert = true, x = Ft(e, k(), y, 0, () => {
          We(e, "outroend"), E == null ? void 0 : E();
        });
      },
      stop: () => {
        y == null ? void 0 : y.abort(), x == null ? void 0 : x.abort();
      }
    }, z = fa;
    if (((_a2 = z.nodes).t ?? (_a2.t = [])).push(S), n && an) {
      var B = u;
      if (!B) {
        for (var N = z.parent; N && (N.f & H1) !== 0; ) for (; (N = N.parent) && (N.f & L1) === 0; ) ;
        B = !N || (N.f & P1) !== 0;
      }
      B && V1(() => {
        A0(() => S.in());
      });
    }
  }
  function Ft(r, e, t, a, n) {
    var i = a === 1;
    if (Y1(e)) {
      var o, u = false;
      return X1(() => {
        if (!u) {
          var z = e({
            direction: i ? "in" : "out"
          });
          o = Ft(r, z, t, a, n);
        }
      }), {
        abort: () => {
          u = true, o == null ? void 0 : o.abort();
        },
        deactivate: () => o.deactivate(),
        reset: () => o.reset(),
        t: () => o.t()
      };
    }
    if (t == null ? void 0 : t.deactivate(), !(e == null ? void 0 : e.duration) && !(e == null ? void 0 : e.delay)) return We(r, i ? "introstart" : "outrostart"), n(), {
      abort: ye,
      deactivate: ye,
      reset: ye,
      t: () => a
    };
    const { delay: m = 0, css: d, tick: p, easing: b = bn } = e;
    var y = [];
    if (i && t === void 0 && (p && p(0, 1), d)) {
      var x = Mr(d(0, 1));
      y.push(x, x);
    }
    var k = () => 1 - a, S = r.animate(y, {
      duration: m,
      fill: "forwards"
    });
    return S.onfinish = () => {
      S.cancel(), We(r, i ? "introstart" : "outrostart");
      var z = (t == null ? void 0 : t.t()) ?? 1 - a;
      t == null ? void 0 : t.abort();
      var B = a - z, N = e.duration * Math.abs(B), E = [];
      if (N > 0) {
        var H = false;
        if (d) for (var O = Math.ceil(N / 16.666666666666668), L = 0; L <= O; L += 1) {
          var V = z + B * b(L / O), U = Mr(d(V, 1 - V));
          E.push(U), H || (H = U.overflow === "hidden");
        }
        H && (r.style.overflow = "hidden"), k = () => {
          var P = S.currentTime;
          return z + B * b(P / N);
        }, p && pn(() => {
          if (S.playState !== "running") return false;
          var P = k();
          return p(P, 1 - P), true;
        });
      }
      S = r.animate(E, {
        duration: N,
        fill: "forwards"
      }), S.onfinish = () => {
        k = () => a, p == null ? void 0 : p(a, 1 - a), n();
      };
    }, {
      abort: () => {
        S && (S.cancel(), S.effect = null, S.onfinish = ye);
      },
      deactivate: () => {
        n = ye;
      },
      reset: () => {
        a === 0 && (p == null ? void 0 : p(1, 0));
      },
      t: () => k()
    };
  }
  function yn(r, e, t) {
    var a = r == null ? "" : "" + r;
    return a === "" ? null : a;
  }
  function xn(r, e, t, a, n, i) {
    var o = r.__className;
    if (S0 || o !== t || o === void 0) {
      var u = yn(t);
      (!S0 || u !== r.getAttribute("class")) && (u == null ? r.removeAttribute("class") : r.className = u), r.__className = t;
    }
    return i;
  }
  function wn(r, e, t = e) {
    var a = /* @__PURE__ */ new WeakSet();
    W1(r, "input", async (n) => {
      var i = n ? r.defaultValue : r.value;
      if (i = vt(r) ? pt(i) : i, t(i), he !== null && a.add(he), await pa(), i !== (i = e())) {
        var o = r.selectionStart, u = r.selectionEnd, m = r.value.length;
        if (r.value = i ?? "", u !== null) {
          var d = r.value.length;
          o === u && u === m && d > m ? (r.selectionStart = d, r.selectionEnd = d) : (r.selectionStart = o, r.selectionEnd = Math.min(u, d));
        }
      }
    }), (S0 && r.defaultValue !== r.value || A0(e) == null && r.value) && (t(vt(r) ? pt(r.value) : r.value), he !== null && a.add(he)), Z1(() => {
      var n = e();
      if (r === document.activeElement) {
        var i = j1 ?? he;
        if (a.has(i)) return;
      }
      vt(r) && n === pt(r.value) || r.type === "date" && !n && !r.value || n !== r.value && (r.value = n ?? "");
    });
  }
  function vt(r) {
    var e = r.type;
    return e === "number" || e === "range";
  }
  function pt(r) {
    return r === "" ? null : +r;
  }
  var kn = C0('<div class="bg-base-200 w-full rounded-sm p-3 shadow-md"><label class="input input-ghost flex w-full items-center gap-2"><i class="fa-solid fa-angle-right"></i> <input type="text" class="grow" placeholder="Input expression"/></label></div>');
  function Sn(r, e) {
    Qe(e, true);
    let t = K1("");
    function a(m) {
      m.key === "Enter" && (m.preventDefault(), p0(t).trim() && (n(p0(t)), Rt(t, "")));
    }
    function n(m) {
      const d = {
        eval: m
      };
      ba.send(JSON.stringify(d));
    }
    var i = kn(), o = i0(i), u = de(i0(o), 2);
    ln(u), a0(o), a0(i), sn("keydown", u, a), wn(u, () => p0(t), (m) => Rt(t, m)), z0(r, i), et();
  }
  nn([
    "keydown"
  ]);
  const Mn = (r) => r;
  function zn(r) {
    const e = r - 1;
    return e * e * e + 1;
  }
  function zr(r) {
    const e = typeof r == "string" && r.match(/^\s*(-?[\d.]+)([^\s]*)\s*$/);
    return e ? [
      parseFloat(e[1]),
      e[2] || "px"
    ] : [
      r,
      "px"
    ];
  }
  function An(r, { delay: e = 0, duration: t = 400, easing: a = Mn } = {}) {
    const n = +getComputedStyle(r).opacity;
    return {
      delay: e,
      duration: t,
      easing: a,
      css: (i) => `opacity: ${i * n}`
    };
  }
  function Tn(r, { delay: e = 0, duration: t = 400, easing: a = zn, x: n = 0, y: i = 0, opacity: o = 0 } = {}) {
    const u = getComputedStyle(r), m = +u.opacity, d = u.transform === "none" ? "" : u.transform, p = m * (1 - o), [b, y] = zr(n), [x, k] = zr(i);
    return {
      delay: e,
      duration: t,
      easing: a,
      css: (S, z) => `
			transform: ${d} translate(${(1 - S) * b}${y}, ${(1 - S) * x}${k});
			opacity: ${m - p * z}`
    };
  }
  function Bn(r) {
    const e = r - 1;
    return e * e * e + 1;
  }
  class h0 {
    constructor(e, t, a) {
      this.lexer = void 0, this.start = void 0, this.end = void 0, this.lexer = e, this.start = t, this.end = a;
    }
    static range(e, t) {
      return t ? !e || !e.loc || !t.loc || e.loc.lexer !== t.loc.lexer ? null : new h0(e.loc.lexer, e.loc.start, t.loc.end) : e && e.loc;
    }
  }
  class d0 {
    constructor(e, t) {
      this.text = void 0, this.loc = void 0, this.noexpand = void 0, this.treatAsRelax = void 0, this.text = e, this.loc = t;
    }
    range(e, t) {
      return new d0(t, h0.range(this, e));
    }
  }
  class T {
    constructor(e, t) {
      this.name = void 0, this.position = void 0, this.length = void 0, this.rawMessage = void 0;
      var a = "KaTeX parse error: " + e, n, i, o = t && t.loc;
      if (o && o.start <= o.end) {
        var u = o.lexer.input;
        n = o.start, i = o.end, n === u.length ? a += " at end of input: " : a += " at position " + (n + 1) + ": ";
        var m = u.slice(n, i).replace(/[^]/g, "$&\u0332"), d;
        n > 15 ? d = "\u2026" + u.slice(n - 15, n) : d = u.slice(0, n);
        var p;
        i + 15 < u.length ? p = u.slice(i, i + 15) + "\u2026" : p = u.slice(i), a += d + m + p;
      }
      var b = new Error(a);
      return b.name = "ParseError", b.__proto__ = T.prototype, b.position = n, n != null && i != null && (b.length = i - n), b.rawMessage = e, b;
    }
  }
  T.prototype.__proto__ = Error.prototype;
  var Cn = function(e, t) {
    return e === void 0 ? t : e;
  }, Dn = /([A-Z])/g, Nn = function(e) {
    return e.replace(Dn, "-$1").toLowerCase();
  }, qn = {
    "&": "&amp;",
    ">": "&gt;",
    "<": "&lt;",
    '"': "&quot;",
    "'": "&#x27;"
  }, En = /[&><"']/g;
  function Rn(r) {
    return String(r).replace(En, (e) => qn[e]);
  }
  var wa = function r(e) {
    return e.type === "ordgroup" || e.type === "color" ? e.body.length === 1 ? r(e.body[0]) : e : e.type === "font" ? r(e.body) : e;
  }, In = function(e) {
    var t = wa(e);
    return t.type === "mathord" || t.type === "textord" || t.type === "atom";
  }, On = function(e) {
    if (!e) throw new Error("Expected non-null, but got " + String(e));
    return e;
  }, Fn = function(e) {
    var t = /^[\x00-\x20]*([^\\/#?]*?)(:|&#0*58|&#x0*3a|&colon)/i.exec(e);
    return t ? t[2] !== ":" || !/^[a-zA-Z][a-zA-Z0-9+\-.]*$/.test(t[1]) ? null : t[1].toLowerCase() : "_relative";
  }, Y = {
    deflt: Cn,
    escape: Rn,
    hyphenate: Nn,
    getBaseElem: wa,
    isCharacterBox: In,
    protocolFromUrl: Fn
  }, Ye = {
    displayMode: {
      type: "boolean",
      description: "Render math in display mode, which puts the math in display style (so \\int and \\sum are large, for example), and centers the math on the page on its own line.",
      cli: "-d, --display-mode"
    },
    output: {
      type: {
        enum: [
          "htmlAndMathml",
          "html",
          "mathml"
        ]
      },
      description: "Determines the markup language of the output.",
      cli: "-F, --format <type>"
    },
    leqno: {
      type: "boolean",
      description: "Render display math in leqno style (left-justified tags)."
    },
    fleqn: {
      type: "boolean",
      description: "Render display math flush left."
    },
    throwOnError: {
      type: "boolean",
      default: true,
      cli: "-t, --no-throw-on-error",
      cliDescription: "Render errors (in the color given by --error-color) instead of throwing a ParseError exception when encountering an error."
    },
    errorColor: {
      type: "string",
      default: "#cc0000",
      cli: "-c, --error-color <color>",
      cliDescription: "A color string given in the format 'rgb' or 'rrggbb' (no #). This option determines the color of errors rendered by the -t option.",
      cliProcessor: (r) => "#" + r
    },
    macros: {
      type: "object",
      cli: "-m, --macro <def>",
      cliDescription: "Define custom macro of the form '\\foo:expansion' (use multiple -m arguments for multiple macros).",
      cliDefault: [],
      cliProcessor: (r, e) => (e.push(r), e)
    },
    minRuleThickness: {
      type: "number",
      description: "Specifies a minimum thickness, in ems, for fraction lines, `\\sqrt` top lines, `{array}` vertical lines, `\\hline`, `\\hdashline`, `\\underline`, `\\overline`, and the borders of `\\fbox`, `\\boxed`, and `\\fcolorbox`.",
      processor: (r) => Math.max(0, r),
      cli: "--min-rule-thickness <size>",
      cliProcessor: parseFloat
    },
    colorIsTextColor: {
      type: "boolean",
      description: "Makes \\color behave like LaTeX's 2-argument \\textcolor, instead of LaTeX's one-argument \\color mode change.",
      cli: "-b, --color-is-text-color"
    },
    strict: {
      type: [
        {
          enum: [
            "warn",
            "ignore",
            "error"
          ]
        },
        "boolean",
        "function"
      ],
      description: "Turn on strict / LaTeX faithfulness mode, which throws an error if the input uses features that are not supported by LaTeX.",
      cli: "-S, --strict",
      cliDefault: false
    },
    trust: {
      type: [
        "boolean",
        "function"
      ],
      description: "Trust the input, enabling all HTML features such as \\url.",
      cli: "-T, --trust"
    },
    maxSize: {
      type: "number",
      default: 1 / 0,
      description: "If non-zero, all user-specified sizes, e.g. in \\rule{500em}{500em}, will be capped to maxSize ems. Otherwise, elements and spaces can be arbitrarily large",
      processor: (r) => Math.max(0, r),
      cli: "-s, --max-size <n>",
      cliProcessor: parseInt
    },
    maxExpand: {
      type: "number",
      default: 1e3,
      description: "Limit the number of macro expansions to the specified number, to prevent e.g. infinite macro loops. If set to Infinity, the macro expander will try to fully expand as in LaTeX.",
      processor: (r) => Math.max(0, r),
      cli: "-e, --max-expand <n>",
      cliProcessor: (r) => r === "Infinity" ? 1 / 0 : parseInt(r)
    },
    globalGroup: {
      type: "boolean",
      cli: false
    }
  };
  function Hn(r) {
    if (r.default) return r.default;
    var e = r.type, t = Array.isArray(e) ? e[0] : e;
    if (typeof t != "string") return t.enum[0];
    switch (t) {
      case "boolean":
        return false;
      case "string":
        return "";
      case "number":
        return 0;
      case "object":
        return {};
    }
  }
  class jt {
    constructor(e) {
      this.displayMode = void 0, this.output = void 0, this.leqno = void 0, this.fleqn = void 0, this.throwOnError = void 0, this.errorColor = void 0, this.macros = void 0, this.minRuleThickness = void 0, this.colorIsTextColor = void 0, this.strict = void 0, this.trust = void 0, this.maxSize = void 0, this.maxExpand = void 0, this.globalGroup = void 0, e = e || {};
      for (var t in Ye) if (Ye.hasOwnProperty(t)) {
        var a = Ye[t];
        this[t] = e[t] !== void 0 ? a.processor ? a.processor(e[t]) : e[t] : Hn(a);
      }
    }
    reportNonstrict(e, t, a) {
      var n = this.strict;
      if (typeof n == "function" && (n = n(e, t, a)), !(!n || n === "ignore")) {
        if (n === true || n === "error") throw new T("LaTeX-incompatible input and strict mode is set to 'error': " + (t + " [" + e + "]"), a);
        n === "warn" ? typeof console < "u" && console.warn("LaTeX-incompatible input and strict mode is set to 'warn': " + (t + " [" + e + "]")) : typeof console < "u" && console.warn("LaTeX-incompatible input and strict mode is set to " + ("unrecognized '" + n + "': " + t + " [" + e + "]"));
      }
    }
    useStrictBehavior(e, t, a) {
      var n = this.strict;
      if (typeof n == "function") try {
        n = n(e, t, a);
      } catch {
        n = "error";
      }
      return !n || n === "ignore" ? false : n === true || n === "error" ? true : n === "warn" ? (typeof console < "u" && console.warn("LaTeX-incompatible input and strict mode is set to 'warn': " + (t + " [" + e + "]")), false) : (typeof console < "u" && console.warn("LaTeX-incompatible input and strict mode is set to " + ("unrecognized '" + n + "': " + t + " [" + e + "]")), false);
    }
    isTrusted(e) {
      if (e.url && !e.protocol) {
        var t = Y.protocolFromUrl(e.url);
        if (t == null) return false;
        e.protocol = t;
      }
      var a = typeof this.trust == "function" ? this.trust(e) : this.trust;
      return !!a;
    }
  }
  class X0 {
    constructor(e, t, a) {
      this.id = void 0, this.size = void 0, this.cramped = void 0, this.id = e, this.size = t, this.cramped = a;
    }
    sup() {
      return M0[Ln[this.id]];
    }
    sub() {
      return M0[Pn[this.id]];
    }
    fracNum() {
      return M0[Vn[this.id]];
    }
    fracDen() {
      return M0[Gn[this.id]];
    }
    cramp() {
      return M0[Un[this.id]];
    }
    text() {
      return M0[$n[this.id]];
    }
    isTight() {
      return this.size >= 2;
    }
  }
  var Kt = 0, Ze = 1, fe = 2, O0 = 3, ze = 4, g0 = 5, ve = 6, s0 = 7, M0 = [
    new X0(Kt, 0, false),
    new X0(Ze, 0, true),
    new X0(fe, 1, false),
    new X0(O0, 1, true),
    new X0(ze, 2, false),
    new X0(g0, 2, true),
    new X0(ve, 3, false),
    new X0(s0, 3, true)
  ], Ln = [
    ze,
    g0,
    ze,
    g0,
    ve,
    s0,
    ve,
    s0
  ], Pn = [
    g0,
    g0,
    g0,
    g0,
    s0,
    s0,
    s0,
    s0
  ], Vn = [
    fe,
    O0,
    ze,
    g0,
    ve,
    s0,
    ve,
    s0
  ], Gn = [
    O0,
    O0,
    g0,
    g0,
    s0,
    s0,
    s0,
    s0
  ], Un = [
    Ze,
    Ze,
    O0,
    O0,
    g0,
    g0,
    s0,
    s0
  ], $n = [
    Kt,
    Ze,
    fe,
    O0,
    fe,
    O0,
    fe,
    O0
  ], I = {
    DISPLAY: M0[Kt],
    TEXT: M0[fe],
    SCRIPT: M0[ze],
    SCRIPTSCRIPT: M0[ve]
  }, Ht = [
    {
      name: "latin",
      blocks: [
        [
          256,
          591
        ],
        [
          768,
          879
        ]
      ]
    },
    {
      name: "cyrillic",
      blocks: [
        [
          1024,
          1279
        ]
      ]
    },
    {
      name: "armenian",
      blocks: [
        [
          1328,
          1423
        ]
      ]
    },
    {
      name: "brahmic",
      blocks: [
        [
          2304,
          4255
        ]
      ]
    },
    {
      name: "georgian",
      blocks: [
        [
          4256,
          4351
        ]
      ]
    },
    {
      name: "cjk",
      blocks: [
        [
          12288,
          12543
        ],
        [
          19968,
          40879
        ],
        [
          65280,
          65376
        ]
      ]
    },
    {
      name: "hangul",
      blocks: [
        [
          44032,
          55215
        ]
      ]
    }
  ];
  function Yn(r) {
    for (var e = 0; e < Ht.length; e++) for (var t = Ht[e], a = 0; a < t.blocks.length; a++) {
      var n = t.blocks[a];
      if (r >= n[0] && r <= n[1]) return t.name;
    }
    return null;
  }
  var Xe = [];
  Ht.forEach((r) => r.blocks.forEach((e) => Xe.push(...e)));
  function ka(r) {
    for (var e = 0; e < Xe.length; e += 2) if (r >= Xe[e] && r <= Xe[e + 1]) return true;
    return false;
  }
  var ue = 80, Xn = function(e, t) {
    return "M95," + (622 + e + t) + `
c-2.7,0,-7.17,-2.7,-13.5,-8c-5.8,-5.3,-9.5,-10,-9.5,-14
c0,-2,0.3,-3.3,1,-4c1.3,-2.7,23.83,-20.7,67.5,-54
c44.2,-33.3,65.8,-50.3,66.5,-51c1.3,-1.3,3,-2,5,-2c4.7,0,8.7,3.3,12,10
s173,378,173,378c0.7,0,35.3,-71,104,-213c68.7,-142,137.5,-285,206.5,-429
c69,-144,104.5,-217.7,106.5,-221
l` + e / 2.075 + " -" + e + `
c5.3,-9.3,12,-14,20,-14
H400000v` + (40 + e) + `H845.2724
s-225.272,467,-225.272,467s-235,486,-235,486c-2.7,4.7,-9,7,-19,7
c-6,0,-10,-1,-12,-3s-194,-422,-194,-422s-65,47,-65,47z
M` + (834 + e) + " " + t + "h400000v" + (40 + e) + "h-400000z";
  }, Wn = function(e, t) {
    return "M263," + (601 + e + t) + `c0.7,0,18,39.7,52,119
c34,79.3,68.167,158.7,102.5,238c34.3,79.3,51.8,119.3,52.5,120
c340,-704.7,510.7,-1060.3,512,-1067
l` + e / 2.084 + " -" + e + `
c4.7,-7.3,11,-11,19,-11
H40000v` + (40 + e) + `H1012.3
s-271.3,567,-271.3,567c-38.7,80.7,-84,175,-136,283c-52,108,-89.167,185.3,-111.5,232
c-22.3,46.7,-33.8,70.3,-34.5,71c-4.7,4.7,-12.3,7,-23,7s-12,-1,-12,-1
s-109,-253,-109,-253c-72.7,-168,-109.3,-252,-110,-252c-10.7,8,-22,16.7,-34,26
c-22,17.3,-33.3,26,-34,26s-26,-26,-26,-26s76,-59,76,-59s76,-60,76,-60z
M` + (1001 + e) + " " + t + "h400000v" + (40 + e) + "h-400000z";
  }, Zn = function(e, t) {
    return "M983 " + (10 + e + t) + `
l` + e / 3.13 + " -" + e + `
c4,-6.7,10,-10,18,-10 H400000v` + (40 + e) + `
H1013.1s-83.4,268,-264.1,840c-180.7,572,-277,876.3,-289,913c-4.7,4.7,-12.7,7,-24,7
s-12,0,-12,0c-1.3,-3.3,-3.7,-11.7,-7,-25c-35.3,-125.3,-106.7,-373.3,-214,-744
c-10,12,-21,25,-33,39s-32,39,-32,39c-6,-5.3,-15,-14,-27,-26s25,-30,25,-30
c26.7,-32.7,52,-63,76,-91s52,-60,52,-60s208,722,208,722
c56,-175.3,126.3,-397.3,211,-666c84.7,-268.7,153.8,-488.2,207.5,-658.5
c53.7,-170.3,84.5,-266.8,92.5,-289.5z
M` + (1001 + e) + " " + t + "h400000v" + (40 + e) + "h-400000z";
  }, jn = function(e, t) {
    return "M424," + (2398 + e + t) + `
c-1.3,-0.7,-38.5,-172,-111.5,-514c-73,-342,-109.8,-513.3,-110.5,-514
c0,-2,-10.7,14.3,-32,49c-4.7,7.3,-9.8,15.7,-15.5,25c-5.7,9.3,-9.8,16,-12.5,20
s-5,7,-5,7c-4,-3.3,-8.3,-7.7,-13,-13s-13,-13,-13,-13s76,-122,76,-122s77,-121,77,-121
s209,968,209,968c0,-2,84.7,-361.7,254,-1079c169.3,-717.3,254.7,-1077.7,256,-1081
l` + e / 4.223 + " -" + e + `c4,-6.7,10,-10,18,-10 H400000
v` + (40 + e) + `H1014.6
s-87.3,378.7,-272.6,1166c-185.3,787.3,-279.3,1182.3,-282,1185
c-2,6,-10,9,-24,9
c-8,0,-12,-0.7,-12,-2z M` + (1001 + e) + " " + t + `
h400000v` + (40 + e) + "h-400000z";
  }, Kn = function(e, t) {
    return "M473," + (2713 + e + t) + `
c339.3,-1799.3,509.3,-2700,510,-2702 l` + e / 5.298 + " -" + e + `
c3.3,-7.3,9.3,-11,18,-11 H400000v` + (40 + e) + `H1017.7
s-90.5,478,-276.2,1466c-185.7,988,-279.5,1483,-281.5,1485c-2,6,-10,9,-24,9
c-8,0,-12,-0.7,-12,-2c0,-1.3,-5.3,-32,-16,-92c-50.7,-293.3,-119.7,-693.3,-207,-1200
c0,-1.3,-5.3,8.7,-16,30c-10.7,21.3,-21.3,42.7,-32,64s-16,33,-16,33s-26,-26,-26,-26
s76,-153,76,-153s77,-151,77,-151c0.7,0.7,35.7,202,105,604c67.3,400.7,102,602.7,104,
606zM` + (1001 + e) + " " + t + "h400000v" + (40 + e) + "H1017.7z";
  }, _n = function(e) {
    var t = e / 2;
    return "M400000 " + e + " H0 L" + t + " 0 l65 45 L145 " + (e - 80) + " H400000z";
  }, Jn = function(e, t, a) {
    var n = a - 54 - t - e;
    return "M702 " + (e + t) + "H400000" + (40 + e) + `
H742v` + n + `l-4 4-4 4c-.667.7 -2 1.5-4 2.5s-4.167 1.833-6.5 2.5-5.5 1-9.5 1
h-12l-28-84c-16.667-52-96.667 -294.333-240-727l-212 -643 -85 170
c-4-3.333-8.333-7.667-13 -13l-13-13l77-155 77-156c66 199.333 139 419.667
219 661 l218 661zM702 ` + t + "H400000v" + (40 + e) + "H742z";
  }, Qn = function(e, t, a) {
    t = 1e3 * t;
    var n = "";
    switch (e) {
      case "sqrtMain":
        n = Xn(t, ue);
        break;
      case "sqrtSize1":
        n = Wn(t, ue);
        break;
      case "sqrtSize2":
        n = Zn(t, ue);
        break;
      case "sqrtSize3":
        n = jn(t, ue);
        break;
      case "sqrtSize4":
        n = Kn(t, ue);
        break;
      case "sqrtTall":
        n = Jn(t, ue, a);
    }
    return n;
  }, ei = function(e, t) {
    switch (e) {
      case "\u239C":
        return "M291 0 H417 V" + t + " H291z M291 0 H417 V" + t + " H291z";
      case "\u2223":
        return "M145 0 H188 V" + t + " H145z M145 0 H188 V" + t + " H145z";
      case "\u2225":
        return "M145 0 H188 V" + t + " H145z M145 0 H188 V" + t + " H145z" + ("M367 0 H410 V" + t + " H367z M367 0 H410 V" + t + " H367z");
      case "\u239F":
        return "M457 0 H583 V" + t + " H457z M457 0 H583 V" + t + " H457z";
      case "\u23A2":
        return "M319 0 H403 V" + t + " H319z M319 0 H403 V" + t + " H319z";
      case "\u23A5":
        return "M263 0 H347 V" + t + " H263z M263 0 H347 V" + t + " H263z";
      case "\u23AA":
        return "M384 0 H504 V" + t + " H384z M384 0 H504 V" + t + " H384z";
      case "\u23D0":
        return "M312 0 H355 V" + t + " H312z M312 0 H355 V" + t + " H312z";
      case "\u2016":
        return "M257 0 H300 V" + t + " H257z M257 0 H300 V" + t + " H257z" + ("M478 0 H521 V" + t + " H478z M478 0 H521 V" + t + " H478z");
      default:
        return "";
    }
  }, Ar = {
    doubleleftarrow: `M262 157
l10-10c34-36 62.7-77 86-123 3.3-8 5-13.3 5-16 0-5.3-6.7-8-20-8-7.3
 0-12.2.5-14.5 1.5-2.3 1-4.8 4.5-7.5 10.5-49.3 97.3-121.7 169.3-217 216-28
 14-57.3 25-88 33-6.7 2-11 3.8-13 5.5-2 1.7-3 4.2-3 7.5s1 5.8 3 7.5
c2 1.7 6.3 3.5 13 5.5 68 17.3 128.2 47.8 180.5 91.5 52.3 43.7 93.8 96.2 124.5
 157.5 9.3 8 15.3 12.3 18 13h6c12-.7 18-4 18-10 0-2-1.7-7-5-15-23.3-46-52-87
-86-123l-10-10h399738v-40H218c328 0 0 0 0 0l-10-8c-26.7-20-65.7-43-117-69 2.7
-2 6-3.7 10-5 36.7-16 72.3-37.3 107-64l10-8h399782v-40z
m8 0v40h399730v-40zm0 194v40h399730v-40z`,
    doublerightarrow: `M399738 392l
-10 10c-34 36-62.7 77-86 123-3.3 8-5 13.3-5 16 0 5.3 6.7 8 20 8 7.3 0 12.2-.5
 14.5-1.5 2.3-1 4.8-4.5 7.5-10.5 49.3-97.3 121.7-169.3 217-216 28-14 57.3-25 88
-33 6.7-2 11-3.8 13-5.5 2-1.7 3-4.2 3-7.5s-1-5.8-3-7.5c-2-1.7-6.3-3.5-13-5.5-68
-17.3-128.2-47.8-180.5-91.5-52.3-43.7-93.8-96.2-124.5-157.5-9.3-8-15.3-12.3-18
-13h-6c-12 .7-18 4-18 10 0 2 1.7 7 5 15 23.3 46 52 87 86 123l10 10H0v40h399782
c-328 0 0 0 0 0l10 8c26.7 20 65.7 43 117 69-2.7 2-6 3.7-10 5-36.7 16-72.3 37.3
-107 64l-10 8H0v40zM0 157v40h399730v-40zm0 194v40h399730v-40z`,
    leftarrow: `M400000 241H110l3-3c68.7-52.7 113.7-120
 135-202 4-14.7 6-23 6-25 0-7.3-7-11-21-11-8 0-13.2.8-15.5 2.5-2.3 1.7-4.2 5.8
-5.5 12.5-1.3 4.7-2.7 10.3-4 17-12 48.7-34.8 92-68.5 130S65.3 228.3 18 247
c-10 4-16 7.7-18 11 0 8.7 6 14.3 18 17 47.3 18.7 87.8 47 121.5 85S196 441.3 208
 490c.7 2 1.3 5 2 9s1.2 6.7 1.5 8c.3 1.3 1 3.3 2 6s2.2 4.5 3.5 5.5c1.3 1 3.3
 1.8 6 2.5s6 1 10 1c14 0 21-3.7 21-11 0-2-2-10.3-6-25-20-79.3-65-146.7-135-202
 l-3-3h399890zM100 241v40h399900v-40z`,
    leftbrace: `M6 548l-6-6v-35l6-11c56-104 135.3-181.3 238-232 57.3-28.7 117
-45 179-50h399577v120H403c-43.3 7-81 15-113 26-100.7 33-179.7 91-237 174-2.7
 5-6 9-10 13-.7 1-7.3 1-20 1H6z`,
    leftbraceunder: `M0 6l6-6h17c12.688 0 19.313.3 20 1 4 4 7.313 8.3 10 13
 35.313 51.3 80.813 93.8 136.5 127.5 55.688 33.7 117.188 55.8 184.5 66.5.688
 0 2 .3 4 1 18.688 2.7 76 4.3 172 5h399450v120H429l-6-1c-124.688-8-235-61.7
-331-161C60.687 138.7 32.312 99.3 7 54L0 41V6z`,
    leftgroup: `M400000 80
H435C64 80 168.3 229.4 21 260c-5.9 1.2-18 0-18 0-2 0-3-1-3-3v-38C76 61 257 0
 435 0h399565z`,
    leftgroupunder: `M400000 262
H435C64 262 168.3 112.6 21 82c-5.9-1.2-18 0-18 0-2 0-3 1-3 3v38c76 158 257 219
 435 219h399565z`,
    leftharpoon: `M0 267c.7 5.3 3 10 7 14h399993v-40H93c3.3
-3.3 10.2-9.5 20.5-18.5s17.8-15.8 22.5-20.5c50.7-52 88-110.3 112-175 4-11.3 5
-18.3 3-21-1.3-4-7.3-6-18-6-8 0-13 .7-15 2s-4.7 6.7-8 16c-42 98.7-107.3 174.7
-196 228-6.7 4.7-10.7 8-12 10-1.3 2-2 5.7-2 11zm100-26v40h399900v-40z`,
    leftharpoonplus: `M0 267c.7 5.3 3 10 7 14h399993v-40H93c3.3-3.3 10.2-9.5
 20.5-18.5s17.8-15.8 22.5-20.5c50.7-52 88-110.3 112-175 4-11.3 5-18.3 3-21-1.3
-4-7.3-6-18-6-8 0-13 .7-15 2s-4.7 6.7-8 16c-42 98.7-107.3 174.7-196 228-6.7 4.7
-10.7 8-12 10-1.3 2-2 5.7-2 11zm100-26v40h399900v-40zM0 435v40h400000v-40z
m0 0v40h400000v-40z`,
    leftharpoondown: `M7 241c-4 4-6.333 8.667-7 14 0 5.333.667 9 2 11s5.333
 5.333 12 10c90.667 54 156 130 196 228 3.333 10.667 6.333 16.333 9 17 2 .667 5
 1 9 1h5c10.667 0 16.667-2 18-6 2-2.667 1-9.667-3-21-32-87.333-82.667-157.667
-152-211l-3-3h399907v-40zM93 281 H400000 v-40L7 241z`,
    leftharpoondownplus: `M7 435c-4 4-6.3 8.7-7 14 0 5.3.7 9 2 11s5.3 5.3 12
 10c90.7 54 156 130 196 228 3.3 10.7 6.3 16.3 9 17 2 .7 5 1 9 1h5c10.7 0 16.7
-2 18-6 2-2.7 1-9.7-3-21-32-87.3-82.7-157.7-152-211l-3-3h399907v-40H7zm93 0
v40h399900v-40zM0 241v40h399900v-40zm0 0v40h399900v-40z`,
    lefthook: `M400000 281 H103s-33-11.2-61-33.5S0 197.3 0 164s14.2-61.2 42.5
-83.5C70.8 58.2 104 47 142 47 c16.7 0 25 6.7 25 20 0 12-8.7 18.7-26 20-40 3.3
-68.7 15.7-86 37-10 12-15 25.3-15 40 0 22.7 9.8 40.7 29.5 54 19.7 13.3 43.5 21
 71.5 23h399859zM103 281v-40h399897v40z`,
    leftlinesegment: `M40 281 V428 H0 V94 H40 V241 H400000 v40z
M40 281 V428 H0 V94 H40 V241 H400000 v40z`,
    leftmapsto: `M40 281 V448H0V74H40V241H400000v40z
M40 281 V448H0V74H40V241H400000v40z`,
    leftToFrom: `M0 147h400000v40H0zm0 214c68 40 115.7 95.7 143 167h22c15.3 0 23
-.3 23-1 0-1.3-5.3-13.7-16-37-18-35.3-41.3-69-70-101l-7-8h399905v-40H95l7-8
c28.7-32 52-65.7 70-101 10.7-23.3 16-35.7 16-37 0-.7-7.7-1-23-1h-22C115.7 265.3
 68 321 0 361zm0-174v-40h399900v40zm100 154v40h399900v-40z`,
    longequal: `M0 50 h400000 v40H0z m0 194h40000v40H0z
M0 50 h400000 v40H0z m0 194h40000v40H0z`,
    midbrace: `M200428 334
c-100.7-8.3-195.3-44-280-108-55.3-42-101.7-93-139-153l-9-14c-2.7 4-5.7 8.7-9 14
-53.3 86.7-123.7 153-211 199-66.7 36-137.3 56.3-212 62H0V214h199568c178.3-11.7
 311.7-78.3 403-201 6-8 9.7-12 11-12 .7-.7 6.7-1 18-1s17.3.3 18 1c1.3 0 5 4 11
 12 44.7 59.3 101.3 106.3 170 141s145.3 54.3 229 60h199572v120z`,
    midbraceunder: `M199572 214
c100.7 8.3 195.3 44 280 108 55.3 42 101.7 93 139 153l9 14c2.7-4 5.7-8.7 9-14
 53.3-86.7 123.7-153 211-199 66.7-36 137.3-56.3 212-62h199568v120H200432c-178.3
 11.7-311.7 78.3-403 201-6 8-9.7 12-11 12-.7.7-6.7 1-18 1s-17.3-.3-18-1c-1.3 0
-5-4-11-12-44.7-59.3-101.3-106.3-170-141s-145.3-54.3-229-60H0V214z`,
    oiintSize1: `M512.6 71.6c272.6 0 320.3 106.8 320.3 178.2 0 70.8-47.7 177.6
-320.3 177.6S193.1 320.6 193.1 249.8c0-71.4 46.9-178.2 319.5-178.2z
m368.1 178.2c0-86.4-60.9-215.4-368.1-215.4-306.4 0-367.3 129-367.3 215.4 0 85.8
60.9 214.8 367.3 214.8 307.2 0 368.1-129 368.1-214.8z`,
    oiintSize2: `M757.8 100.1c384.7 0 451.1 137.6 451.1 230 0 91.3-66.4 228.8
-451.1 228.8-386.3 0-452.7-137.5-452.7-228.8 0-92.4 66.4-230 452.7-230z
m502.4 230c0-111.2-82.4-277.2-502.4-277.2s-504 166-504 277.2
c0 110 84 276 504 276s502.4-166 502.4-276z`,
    oiiintSize1: `M681.4 71.6c408.9 0 480.5 106.8 480.5 178.2 0 70.8-71.6 177.6
-480.5 177.6S202.1 320.6 202.1 249.8c0-71.4 70.5-178.2 479.3-178.2z
m525.8 178.2c0-86.4-86.8-215.4-525.7-215.4-437.9 0-524.7 129-524.7 215.4 0
85.8 86.8 214.8 524.7 214.8 438.9 0 525.7-129 525.7-214.8z`,
    oiiintSize2: `M1021.2 53c603.6 0 707.8 165.8 707.8 277.2 0 110-104.2 275.8
-707.8 275.8-606 0-710.2-165.8-710.2-275.8C311 218.8 415.2 53 1021.2 53z
m770.4 277.1c0-131.2-126.4-327.6-770.5-327.6S248.4 198.9 248.4 330.1
c0 130 128.8 326.4 772.7 326.4s770.5-196.4 770.5-326.4z`,
    rightarrow: `M0 241v40h399891c-47.3 35.3-84 78-110 128
-16.7 32-27.7 63.7-33 95 0 1.3-.2 2.7-.5 4-.3 1.3-.5 2.3-.5 3 0 7.3 6.7 11 20
 11 8 0 13.2-.8 15.5-2.5 2.3-1.7 4.2-5.5 5.5-11.5 2-13.3 5.7-27 11-41 14.7-44.7
 39-84.5 73-119.5s73.7-60.2 119-75.5c6-2 9-5.7 9-11s-3-9-9-11c-45.3-15.3-85
-40.5-119-75.5s-58.3-74.8-73-119.5c-4.7-14-8.3-27.3-11-40-1.3-6.7-3.2-10.8-5.5
-12.5-2.3-1.7-7.5-2.5-15.5-2.5-14 0-21 3.7-21 11 0 2 2 10.3 6 25 20.7 83.3 67
 151.7 139 205zm0 0v40h399900v-40z`,
    rightbrace: `M400000 542l
-6 6h-17c-12.7 0-19.3-.3-20-1-4-4-7.3-8.3-10-13-35.3-51.3-80.8-93.8-136.5-127.5
s-117.2-55.8-184.5-66.5c-.7 0-2-.3-4-1-18.7-2.7-76-4.3-172-5H0V214h399571l6 1
c124.7 8 235 61.7 331 161 31.3 33.3 59.7 72.7 85 118l7 13v35z`,
    rightbraceunder: `M399994 0l6 6v35l-6 11c-56 104-135.3 181.3-238 232-57.3
 28.7-117 45-179 50H-300V214h399897c43.3-7 81-15 113-26 100.7-33 179.7-91 237
-174 2.7-5 6-9 10-13 .7-1 7.3-1 20-1h17z`,
    rightgroup: `M0 80h399565c371 0 266.7 149.4 414 180 5.9 1.2 18 0 18 0 2 0
 3-1 3-3v-38c-76-158-257-219-435-219H0z`,
    rightgroupunder: `M0 262h399565c371 0 266.7-149.4 414-180 5.9-1.2 18 0 18
 0 2 0 3 1 3 3v38c-76 158-257 219-435 219H0z`,
    rightharpoon: `M0 241v40h399993c4.7-4.7 7-9.3 7-14 0-9.3
-3.7-15.3-11-18-92.7-56.7-159-133.7-199-231-3.3-9.3-6-14.7-8-16-2-1.3-7-2-15-2
-10.7 0-16.7 2-18 6-2 2.7-1 9.7 3 21 15.3 42 36.7 81.8 64 119.5 27.3 37.7 58
 69.2 92 94.5zm0 0v40h399900v-40z`,
    rightharpoonplus: `M0 241v40h399993c4.7-4.7 7-9.3 7-14 0-9.3-3.7-15.3-11
-18-92.7-56.7-159-133.7-199-231-3.3-9.3-6-14.7-8-16-2-1.3-7-2-15-2-10.7 0-16.7
 2-18 6-2 2.7-1 9.7 3 21 15.3 42 36.7 81.8 64 119.5 27.3 37.7 58 69.2 92 94.5z
m0 0v40h399900v-40z m100 194v40h399900v-40zm0 0v40h399900v-40z`,
    rightharpoondown: `M399747 511c0 7.3 6.7 11 20 11 8 0 13-.8 15-2.5s4.7-6.8
 8-15.5c40-94 99.3-166.3 178-217 13.3-8 20.3-12.3 21-13 5.3-3.3 8.5-5.8 9.5
-7.5 1-1.7 1.5-5.2 1.5-10.5s-2.3-10.3-7-15H0v40h399908c-34 25.3-64.7 57-92 95
-27.3 38-48.7 77.7-64 119-3.3 8.7-5 14-5 16zM0 241v40h399900v-40z`,
    rightharpoondownplus: `M399747 705c0 7.3 6.7 11 20 11 8 0 13-.8
 15-2.5s4.7-6.8 8-15.5c40-94 99.3-166.3 178-217 13.3-8 20.3-12.3 21-13 5.3-3.3
 8.5-5.8 9.5-7.5 1-1.7 1.5-5.2 1.5-10.5s-2.3-10.3-7-15H0v40h399908c-34 25.3
-64.7 57-92 95-27.3 38-48.7 77.7-64 119-3.3 8.7-5 14-5 16zM0 435v40h399900v-40z
m0-194v40h400000v-40zm0 0v40h400000v-40z`,
    righthook: `M399859 241c-764 0 0 0 0 0 40-3.3 68.7-15.7 86-37 10-12 15-25.3
 15-40 0-22.7-9.8-40.7-29.5-54-19.7-13.3-43.5-21-71.5-23-17.3-1.3-26-8-26-20 0
-13.3 8.7-20 26-20 38 0 71 11.2 99 33.5 0 0 7 5.6 21 16.7 14 11.2 21 33.5 21
 66.8s-14 61.2-42 83.5c-28 22.3-61 33.5-99 33.5L0 241z M0 281v-40h399859v40z`,
    rightlinesegment: `M399960 241 V94 h40 V428 h-40 V281 H0 v-40z
M399960 241 V94 h40 V428 h-40 V281 H0 v-40z`,
    rightToFrom: `M400000 167c-70.7-42-118-97.7-142-167h-23c-15.3 0-23 .3-23
 1 0 1.3 5.3 13.7 16 37 18 35.3 41.3 69 70 101l7 8H0v40h399905l-7 8c-28.7 32
-52 65.7-70 101-10.7 23.3-16 35.7-16 37 0 .7 7.7 1 23 1h23c24-69.3 71.3-125 142
-167z M100 147v40h399900v-40zM0 341v40h399900v-40z`,
    twoheadleftarrow: `M0 167c68 40
 115.7 95.7 143 167h22c15.3 0 23-.3 23-1 0-1.3-5.3-13.7-16-37-18-35.3-41.3-69
-70-101l-7-8h125l9 7c50.7 39.3 85 86 103 140h46c0-4.7-6.3-18.7-19-42-18-35.3
-40-67.3-66-96l-9-9h399716v-40H284l9-9c26-28.7 48-60.7 66-96 12.7-23.333 19
-37.333 19-42h-46c-18 54-52.3 100.7-103 140l-9 7H95l7-8c28.7-32 52-65.7 70-101
 10.7-23.333 16-35.7 16-37 0-.7-7.7-1-23-1h-22C115.7 71.3 68 127 0 167z`,
    twoheadrightarrow: `M400000 167
c-68-40-115.7-95.7-143-167h-22c-15.3 0-23 .3-23 1 0 1.3 5.3 13.7 16 37 18 35.3
 41.3 69 70 101l7 8h-125l-9-7c-50.7-39.3-85-86-103-140h-46c0 4.7 6.3 18.7 19 42
 18 35.3 40 67.3 66 96l9 9H0v40h399716l-9 9c-26 28.7-48 60.7-66 96-12.7 23.333
-19 37.333-19 42h46c18-54 52.3-100.7 103-140l9-7h125l-7 8c-28.7 32-52 65.7-70
 101-10.7 23.333-16 35.7-16 37 0 .7 7.7 1 23 1h22c27.3-71.3 75-127 143-167z`,
    tilde1: `M200 55.538c-77 0-168 73.953-177 73.953-3 0-7
-2.175-9-5.437L2 97c-1-2-2-4-2-6 0-4 2-7 5-9l20-12C116 12 171 0 207 0c86 0
 114 68 191 68 78 0 168-68 177-68 4 0 7 2 9 5l12 19c1 2.175 2 4.35 2 6.525 0
 4.35-2 7.613-5 9.788l-19 13.05c-92 63.077-116.937 75.308-183 76.128
-68.267.847-113-73.952-191-73.952z`,
    tilde2: `M344 55.266c-142 0-300.638 81.316-311.5 86.418
-8.01 3.762-22.5 10.91-23.5 5.562L1 120c-1-2-1-3-1-4 0-5 3-9 8-10l18.4-9C160.9
 31.9 283 0 358 0c148 0 188 122 331 122s314-97 326-97c4 0 8 2 10 7l7 21.114
c1 2.14 1 3.21 1 4.28 0 5.347-3 9.626-7 10.696l-22.3 12.622C852.6 158.372 751
 181.476 676 181.476c-149 0-189-126.21-332-126.21z`,
    tilde3: `M786 59C457 59 32 175.242 13 175.242c-6 0-10-3.457
-11-10.37L.15 138c-1-7 3-12 10-13l19.2-6.4C378.4 40.7 634.3 0 804.3 0c337 0
 411.8 157 746.8 157 328 0 754-112 773-112 5 0 10 3 11 9l1 14.075c1 8.066-.697
 16.595-6.697 17.492l-21.052 7.31c-367.9 98.146-609.15 122.696-778.15 122.696
 -338 0-409-156.573-744-156.573z`,
    tilde4: `M786 58C457 58 32 177.487 13 177.487c-6 0-10-3.345
-11-10.035L.15 143c-1-7 3-12 10-13l22-6.7C381.2 35 637.15 0 807.15 0c337 0 409
 177 744 177 328 0 754-127 773-127 5 0 10 3 11 9l1 14.794c1 7.805-3 13.38-9
 14.495l-20.7 5.574c-366.85 99.79-607.3 139.372-776.3 139.372-338 0-409
 -175.236-744-175.236z`,
    vec: `M377 20c0-5.333 1.833-10 5.5-14S391 0 397 0c4.667 0 8.667 1.667 12 5
3.333 2.667 6.667 9 10 19 6.667 24.667 20.333 43.667 41 57 7.333 4.667 11
10.667 11 18 0 6-1 10-3 12s-6.667 5-14 9c-28.667 14.667-53.667 35.667-75 63
-1.333 1.333-3.167 3.5-5.5 6.5s-4 4.833-5 5.5c-1 .667-2.5 1.333-4.5 2s-4.333 1
-7 1c-4.667 0-9.167-1.833-13.5-5.5S337 184 337 178c0-12.667 15.667-32.333 47-59
H213l-171-1c-8.667-6-13-12.333-13-19 0-4.667 4.333-11.333 13-20h359
c-16-25.333-24-45-24-59z`,
    widehat1: `M529 0h5l519 115c5 1 9 5 9 10 0 1-1 2-1 3l-4 22
c-1 5-5 9-11 9h-2L532 67 19 159h-2c-5 0-9-4-11-9l-5-22c-1-6 2-12 8-13z`,
    widehat2: `M1181 0h2l1171 176c6 0 10 5 10 11l-2 23c-1 6-5 10
-11 10h-1L1182 67 15 220h-1c-6 0-10-4-11-10l-2-23c-1-6 4-11 10-11z`,
    widehat3: `M1181 0h2l1171 236c6 0 10 5 10 11l-2 23c-1 6-5 10
-11 10h-1L1182 67 15 280h-1c-6 0-10-4-11-10l-2-23c-1-6 4-11 10-11z`,
    widehat4: `M1181 0h2l1171 296c6 0 10 5 10 11l-2 23c-1 6-5 10
-11 10h-1L1182 67 15 340h-1c-6 0-10-4-11-10l-2-23c-1-6 4-11 10-11z`,
    widecheck1: `M529,159h5l519,-115c5,-1,9,-5,9,-10c0,-1,-1,-2,-1,-3l-4,-22c-1,
-5,-5,-9,-11,-9h-2l-512,92l-513,-92h-2c-5,0,-9,4,-11,9l-5,22c-1,6,2,12,8,13z`,
    widecheck2: `M1181,220h2l1171,-176c6,0,10,-5,10,-11l-2,-23c-1,-6,-5,-10,
-11,-10h-1l-1168,153l-1167,-153h-1c-6,0,-10,4,-11,10l-2,23c-1,6,4,11,10,11z`,
    widecheck3: `M1181,280h2l1171,-236c6,0,10,-5,10,-11l-2,-23c-1,-6,-5,-10,
-11,-10h-1l-1168,213l-1167,-213h-1c-6,0,-10,4,-11,10l-2,23c-1,6,4,11,10,11z`,
    widecheck4: `M1181,340h2l1171,-296c6,0,10,-5,10,-11l-2,-23c-1,-6,-5,-10,
-11,-10h-1l-1168,273l-1167,-273h-1c-6,0,-10,4,-11,10l-2,23c-1,6,4,11,10,11z`,
    baraboveleftarrow: `M400000 620h-399890l3 -3c68.7 -52.7 113.7 -120 135 -202
c4 -14.7 6 -23 6 -25c0 -7.3 -7 -11 -21 -11c-8 0 -13.2 0.8 -15.5 2.5
c-2.3 1.7 -4.2 5.8 -5.5 12.5c-1.3 4.7 -2.7 10.3 -4 17c-12 48.7 -34.8 92 -68.5 130
s-74.2 66.3 -121.5 85c-10 4 -16 7.7 -18 11c0 8.7 6 14.3 18 17c47.3 18.7 87.8 47
121.5 85s56.5 81.3 68.5 130c0.7 2 1.3 5 2 9s1.2 6.7 1.5 8c0.3 1.3 1 3.3 2 6
s2.2 4.5 3.5 5.5c1.3 1 3.3 1.8 6 2.5s6 1 10 1c14 0 21 -3.7 21 -11
c0 -2 -2 -10.3 -6 -25c-20 -79.3 -65 -146.7 -135 -202l-3 -3h399890z
M100 620v40h399900v-40z M0 241v40h399900v-40zM0 241v40h399900v-40z`,
    rightarrowabovebar: `M0 241v40h399891c-47.3 35.3-84 78-110 128-16.7 32
-27.7 63.7-33 95 0 1.3-.2 2.7-.5 4-.3 1.3-.5 2.3-.5 3 0 7.3 6.7 11 20 11 8 0
13.2-.8 15.5-2.5 2.3-1.7 4.2-5.5 5.5-11.5 2-13.3 5.7-27 11-41 14.7-44.7 39
-84.5 73-119.5s73.7-60.2 119-75.5c6-2 9-5.7 9-11s-3-9-9-11c-45.3-15.3-85-40.5
-119-75.5s-58.3-74.8-73-119.5c-4.7-14-8.3-27.3-11-40-1.3-6.7-3.2-10.8-5.5
-12.5-2.3-1.7-7.5-2.5-15.5-2.5-14 0-21 3.7-21 11 0 2 2 10.3 6 25 20.7 83.3 67
151.7 139 205zm96 379h399894v40H0zm0 0h399904v40H0z`,
    baraboveshortleftharpoon: `M507,435c-4,4,-6.3,8.7,-7,14c0,5.3,0.7,9,2,11
c1.3,2,5.3,5.3,12,10c90.7,54,156,130,196,228c3.3,10.7,6.3,16.3,9,17
c2,0.7,5,1,9,1c0,0,5,0,5,0c10.7,0,16.7,-2,18,-6c2,-2.7,1,-9.7,-3,-21
c-32,-87.3,-82.7,-157.7,-152,-211c0,0,-3,-3,-3,-3l399351,0l0,-40
c-398570,0,-399437,0,-399437,0z M593 435 v40 H399500 v-40z
M0 281 v-40 H399908 v40z M0 281 v-40 H399908 v40z`,
    rightharpoonaboveshortbar: `M0,241 l0,40c399126,0,399993,0,399993,0
c4.7,-4.7,7,-9.3,7,-14c0,-9.3,-3.7,-15.3,-11,-18c-92.7,-56.7,-159,-133.7,-199,
-231c-3.3,-9.3,-6,-14.7,-8,-16c-2,-1.3,-7,-2,-15,-2c-10.7,0,-16.7,2,-18,6
c-2,2.7,-1,9.7,3,21c15.3,42,36.7,81.8,64,119.5c27.3,37.7,58,69.2,92,94.5z
M0 241 v40 H399908 v-40z M0 475 v-40 H399500 v40z M0 475 v-40 H399500 v40z`,
    shortbaraboveleftharpoon: `M7,435c-4,4,-6.3,8.7,-7,14c0,5.3,0.7,9,2,11
c1.3,2,5.3,5.3,12,10c90.7,54,156,130,196,228c3.3,10.7,6.3,16.3,9,17c2,0.7,5,1,9,
1c0,0,5,0,5,0c10.7,0,16.7,-2,18,-6c2,-2.7,1,-9.7,-3,-21c-32,-87.3,-82.7,-157.7,
-152,-211c0,0,-3,-3,-3,-3l399907,0l0,-40c-399126,0,-399993,0,-399993,0z
M93 435 v40 H400000 v-40z M500 241 v40 H400000 v-40z M500 241 v40 H400000 v-40z`,
    shortrightharpoonabovebar: `M53,241l0,40c398570,0,399437,0,399437,0
c4.7,-4.7,7,-9.3,7,-14c0,-9.3,-3.7,-15.3,-11,-18c-92.7,-56.7,-159,-133.7,-199,
-231c-3.3,-9.3,-6,-14.7,-8,-16c-2,-1.3,-7,-2,-15,-2c-10.7,0,-16.7,2,-18,6
c-2,2.7,-1,9.7,3,21c15.3,42,36.7,81.8,64,119.5c27.3,37.7,58,69.2,92,94.5z
M500 241 v40 H399408 v-40z M500 435 v40 H400000 v-40z`
  }, ti = function(e, t) {
    switch (e) {
      case "lbrack":
        return "M403 1759 V84 H666 V0 H319 V1759 v" + t + ` v1759 h347 v-84
H403z M403 1759 V0 H319 V1759 v` + t + " v1759 h84z";
      case "rbrack":
        return "M347 1759 V0 H0 V84 H263 V1759 v" + t + ` v1759 H0 v84 H347z
M347 1759 V0 H263 V1759 v` + t + " v1759 h84z";
      case "vert":
        return "M145 15 v585 v" + t + ` v585 c2.667,10,9.667,15,21,15
c10,0,16.667,-5,20,-15 v-585 v` + -t + ` v-585 c-2.667,-10,-9.667,-15,-21,-15
c-10,0,-16.667,5,-20,15z M188 15 H145 v585 v` + t + " v585 h43z";
      case "doublevert":
        return "M145 15 v585 v" + t + ` v585 c2.667,10,9.667,15,21,15
c10,0,16.667,-5,20,-15 v-585 v` + -t + ` v-585 c-2.667,-10,-9.667,-15,-21,-15
c-10,0,-16.667,5,-20,15z M188 15 H145 v585 v` + t + ` v585 h43z
M367 15 v585 v` + t + ` v585 c2.667,10,9.667,15,21,15
c10,0,16.667,-5,20,-15 v-585 v` + -t + ` v-585 c-2.667,-10,-9.667,-15,-21,-15
c-10,0,-16.667,5,-20,15z M410 15 H367 v585 v` + t + " v585 h43z";
      case "lfloor":
        return "M319 602 V0 H403 V602 v" + t + ` v1715 h263 v84 H319z
MM319 602 V0 H403 V602 v` + t + " v1715 H319z";
      case "rfloor":
        return "M319 602 V0 H403 V602 v" + t + ` v1799 H0 v-84 H319z
MM319 602 V0 H403 V602 v` + t + " v1715 H319z";
      case "lceil":
        return "M403 1759 V84 H666 V0 H319 V1759 v" + t + ` v602 h84z
M403 1759 V0 H319 V1759 v` + t + " v602 h84z";
      case "rceil":
        return "M347 1759 V0 H0 V84 H263 V1759 v" + t + ` v602 h84z
M347 1759 V0 h-84 V1759 v` + t + " v602 h84z";
      case "lparen":
        return `M863,9c0,-2,-2,-5,-6,-9c0,0,-17,0,-17,0c-12.7,0,-19.3,0.3,-20,1
c-5.3,5.3,-10.3,11,-15,17c-242.7,294.7,-395.3,682,-458,1162c-21.3,163.3,-33.3,349,
-36,557 l0,` + (t + 84) + `c0.2,6,0,26,0,60c2,159.3,10,310.7,24,454c53.3,528,210,
949.7,470,1265c4.7,6,9.7,11.7,15,17c0.7,0.7,7,1,19,1c0,0,18,0,18,0c4,-4,6,-7,6,-9
c0,-2.7,-3.3,-8.7,-10,-18c-135.3,-192.7,-235.5,-414.3,-300.5,-665c-65,-250.7,-102.5,
-544.7,-112.5,-882c-2,-104,-3,-167,-3,-189
l0,-` + (t + 92) + `c0,-162.7,5.7,-314,17,-454c20.7,-272,63.7,-513,129,-723c65.3,
-210,155.3,-396.3,270,-559c6.7,-9.3,10,-15.3,10,-18z`;
      case "rparen":
        return `M76,0c-16.7,0,-25,3,-25,9c0,2,2,6.3,6,13c21.3,28.7,42.3,60.3,
63,95c96.7,156.7,172.8,332.5,228.5,527.5c55.7,195,92.8,416.5,111.5,664.5
c11.3,139.3,17,290.7,17,454c0,28,1.7,43,3.3,45l0,` + (t + 9) + `
c-3,4,-3.3,16.7,-3.3,38c0,162,-5.7,313.7,-17,455c-18.7,248,-55.8,469.3,-111.5,664
c-55.7,194.7,-131.8,370.3,-228.5,527c-20.7,34.7,-41.7,66.3,-63,95c-2,3.3,-4,7,-6,11
c0,7.3,5.7,11,17,11c0,0,11,0,11,0c9.3,0,14.3,-0.3,15,-1c5.3,-5.3,10.3,-11,15,-17
c242.7,-294.7,395.3,-681.7,458,-1161c21.3,-164.7,33.3,-350.7,36,-558
l0,-` + (t + 144) + `c-2,-159.3,-10,-310.7,-24,-454c-53.3,-528,-210,-949.7,
-470,-1265c-4.7,-6,-9.7,-11.7,-15,-17c-0.7,-0.7,-6.7,-1,-18,-1z`;
      default:
        throw new Error("Unknown stretchy delimiter.");
    }
  };
  class Te {
    constructor(e) {
      this.children = void 0, this.classes = void 0, this.height = void 0, this.depth = void 0, this.maxFontSize = void 0, this.style = void 0, this.children = e, this.classes = [], this.height = 0, this.depth = 0, this.maxFontSize = 0, this.style = {};
    }
    hasClass(e) {
      return this.classes.includes(e);
    }
    toNode() {
      for (var e = document.createDocumentFragment(), t = 0; t < this.children.length; t++) e.appendChild(this.children[t].toNode());
      return e;
    }
    toMarkup() {
      for (var e = "", t = 0; t < this.children.length; t++) e += this.children[t].toMarkup();
      return e;
    }
    toText() {
      var e = (t) => t.toText();
      return this.children.map(e).join("");
    }
  }
  var T0 = {
    "AMS-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      65: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      66: [
        0,
        0.68889,
        0,
        0,
        0.66667
      ],
      67: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      68: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      69: [
        0,
        0.68889,
        0,
        0,
        0.66667
      ],
      70: [
        0,
        0.68889,
        0,
        0,
        0.61111
      ],
      71: [
        0,
        0.68889,
        0,
        0,
        0.77778
      ],
      72: [
        0,
        0.68889,
        0,
        0,
        0.77778
      ],
      73: [
        0,
        0.68889,
        0,
        0,
        0.38889
      ],
      74: [
        0.16667,
        0.68889,
        0,
        0,
        0.5
      ],
      75: [
        0,
        0.68889,
        0,
        0,
        0.77778
      ],
      76: [
        0,
        0.68889,
        0,
        0,
        0.66667
      ],
      77: [
        0,
        0.68889,
        0,
        0,
        0.94445
      ],
      78: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      79: [
        0.16667,
        0.68889,
        0,
        0,
        0.77778
      ],
      80: [
        0,
        0.68889,
        0,
        0,
        0.61111
      ],
      81: [
        0.16667,
        0.68889,
        0,
        0,
        0.77778
      ],
      82: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      83: [
        0,
        0.68889,
        0,
        0,
        0.55556
      ],
      84: [
        0,
        0.68889,
        0,
        0,
        0.66667
      ],
      85: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      86: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      87: [
        0,
        0.68889,
        0,
        0,
        1
      ],
      88: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      89: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      90: [
        0,
        0.68889,
        0,
        0,
        0.66667
      ],
      107: [
        0,
        0.68889,
        0,
        0,
        0.55556
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      165: [
        0,
        0.675,
        0.025,
        0,
        0.75
      ],
      174: [
        0.15559,
        0.69224,
        0,
        0,
        0.94666
      ],
      240: [
        0,
        0.68889,
        0,
        0,
        0.55556
      ],
      295: [
        0,
        0.68889,
        0,
        0,
        0.54028
      ],
      710: [
        0,
        0.825,
        0,
        0,
        2.33334
      ],
      732: [
        0,
        0.9,
        0,
        0,
        2.33334
      ],
      770: [
        0,
        0.825,
        0,
        0,
        2.33334
      ],
      771: [
        0,
        0.9,
        0,
        0,
        2.33334
      ],
      989: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      1008: [
        0,
        0.43056,
        0.04028,
        0,
        0.66667
      ],
      8245: [
        0,
        0.54986,
        0,
        0,
        0.275
      ],
      8463: [
        0,
        0.68889,
        0,
        0,
        0.54028
      ],
      8487: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      8498: [
        0,
        0.68889,
        0,
        0,
        0.55556
      ],
      8502: [
        0,
        0.68889,
        0,
        0,
        0.66667
      ],
      8503: [
        0,
        0.68889,
        0,
        0,
        0.44445
      ],
      8504: [
        0,
        0.68889,
        0,
        0,
        0.66667
      ],
      8513: [
        0,
        0.68889,
        0,
        0,
        0.63889
      ],
      8592: [
        -0.03598,
        0.46402,
        0,
        0,
        0.5
      ],
      8594: [
        -0.03598,
        0.46402,
        0,
        0,
        0.5
      ],
      8602: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8603: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8606: [
        0.01354,
        0.52239,
        0,
        0,
        1
      ],
      8608: [
        0.01354,
        0.52239,
        0,
        0,
        1
      ],
      8610: [
        0.01354,
        0.52239,
        0,
        0,
        1.11111
      ],
      8611: [
        0.01354,
        0.52239,
        0,
        0,
        1.11111
      ],
      8619: [
        0,
        0.54986,
        0,
        0,
        1
      ],
      8620: [
        0,
        0.54986,
        0,
        0,
        1
      ],
      8621: [
        -0.13313,
        0.37788,
        0,
        0,
        1.38889
      ],
      8622: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8624: [
        0,
        0.69224,
        0,
        0,
        0.5
      ],
      8625: [
        0,
        0.69224,
        0,
        0,
        0.5
      ],
      8630: [
        0,
        0.43056,
        0,
        0,
        1
      ],
      8631: [
        0,
        0.43056,
        0,
        0,
        1
      ],
      8634: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8635: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8638: [
        0.19444,
        0.69224,
        0,
        0,
        0.41667
      ],
      8639: [
        0.19444,
        0.69224,
        0,
        0,
        0.41667
      ],
      8642: [
        0.19444,
        0.69224,
        0,
        0,
        0.41667
      ],
      8643: [
        0.19444,
        0.69224,
        0,
        0,
        0.41667
      ],
      8644: [
        0.1808,
        0.675,
        0,
        0,
        1
      ],
      8646: [
        0.1808,
        0.675,
        0,
        0,
        1
      ],
      8647: [
        0.1808,
        0.675,
        0,
        0,
        1
      ],
      8648: [
        0.19444,
        0.69224,
        0,
        0,
        0.83334
      ],
      8649: [
        0.1808,
        0.675,
        0,
        0,
        1
      ],
      8650: [
        0.19444,
        0.69224,
        0,
        0,
        0.83334
      ],
      8651: [
        0.01354,
        0.52239,
        0,
        0,
        1
      ],
      8652: [
        0.01354,
        0.52239,
        0,
        0,
        1
      ],
      8653: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8654: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8655: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8666: [
        0.13667,
        0.63667,
        0,
        0,
        1
      ],
      8667: [
        0.13667,
        0.63667,
        0,
        0,
        1
      ],
      8669: [
        -0.13313,
        0.37788,
        0,
        0,
        1
      ],
      8672: [
        -0.064,
        0.437,
        0,
        0,
        1.334
      ],
      8674: [
        -0.064,
        0.437,
        0,
        0,
        1.334
      ],
      8705: [
        0,
        0.825,
        0,
        0,
        0.5
      ],
      8708: [
        0,
        0.68889,
        0,
        0,
        0.55556
      ],
      8709: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      8717: [
        0,
        0.43056,
        0,
        0,
        0.42917
      ],
      8722: [
        -0.03598,
        0.46402,
        0,
        0,
        0.5
      ],
      8724: [
        0.08198,
        0.69224,
        0,
        0,
        0.77778
      ],
      8726: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      8733: [
        0,
        0.69224,
        0,
        0,
        0.77778
      ],
      8736: [
        0,
        0.69224,
        0,
        0,
        0.72222
      ],
      8737: [
        0,
        0.69224,
        0,
        0,
        0.72222
      ],
      8738: [
        0.03517,
        0.52239,
        0,
        0,
        0.72222
      ],
      8739: [
        0.08167,
        0.58167,
        0,
        0,
        0.22222
      ],
      8740: [
        0.25142,
        0.74111,
        0,
        0,
        0.27778
      ],
      8741: [
        0.08167,
        0.58167,
        0,
        0,
        0.38889
      ],
      8742: [
        0.25142,
        0.74111,
        0,
        0,
        0.5
      ],
      8756: [
        0,
        0.69224,
        0,
        0,
        0.66667
      ],
      8757: [
        0,
        0.69224,
        0,
        0,
        0.66667
      ],
      8764: [
        -0.13313,
        0.36687,
        0,
        0,
        0.77778
      ],
      8765: [
        -0.13313,
        0.37788,
        0,
        0,
        0.77778
      ],
      8769: [
        -0.13313,
        0.36687,
        0,
        0,
        0.77778
      ],
      8770: [
        -0.03625,
        0.46375,
        0,
        0,
        0.77778
      ],
      8774: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8776: [
        -0.01688,
        0.48312,
        0,
        0,
        0.77778
      ],
      8778: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      8782: [
        0.06062,
        0.54986,
        0,
        0,
        0.77778
      ],
      8783: [
        0.06062,
        0.54986,
        0,
        0,
        0.77778
      ],
      8785: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8786: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8787: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8790: [
        0,
        0.69224,
        0,
        0,
        0.77778
      ],
      8791: [
        0.22958,
        0.72958,
        0,
        0,
        0.77778
      ],
      8796: [
        0.08198,
        0.91667,
        0,
        0,
        0.77778
      ],
      8806: [
        0.25583,
        0.75583,
        0,
        0,
        0.77778
      ],
      8807: [
        0.25583,
        0.75583,
        0,
        0,
        0.77778
      ],
      8808: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      8809: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      8812: [
        0.25583,
        0.75583,
        0,
        0,
        0.5
      ],
      8814: [
        0.20576,
        0.70576,
        0,
        0,
        0.77778
      ],
      8815: [
        0.20576,
        0.70576,
        0,
        0,
        0.77778
      ],
      8816: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8817: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8818: [
        0.22958,
        0.72958,
        0,
        0,
        0.77778
      ],
      8819: [
        0.22958,
        0.72958,
        0,
        0,
        0.77778
      ],
      8822: [
        0.1808,
        0.675,
        0,
        0,
        0.77778
      ],
      8823: [
        0.1808,
        0.675,
        0,
        0,
        0.77778
      ],
      8828: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      8829: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      8830: [
        0.22958,
        0.72958,
        0,
        0,
        0.77778
      ],
      8831: [
        0.22958,
        0.72958,
        0,
        0,
        0.77778
      ],
      8832: [
        0.20576,
        0.70576,
        0,
        0,
        0.77778
      ],
      8833: [
        0.20576,
        0.70576,
        0,
        0,
        0.77778
      ],
      8840: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8841: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8842: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8843: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8847: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      8848: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      8858: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8859: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8861: [
        0.08198,
        0.58198,
        0,
        0,
        0.77778
      ],
      8862: [
        0,
        0.675,
        0,
        0,
        0.77778
      ],
      8863: [
        0,
        0.675,
        0,
        0,
        0.77778
      ],
      8864: [
        0,
        0.675,
        0,
        0,
        0.77778
      ],
      8865: [
        0,
        0.675,
        0,
        0,
        0.77778
      ],
      8872: [
        0,
        0.69224,
        0,
        0,
        0.61111
      ],
      8873: [
        0,
        0.69224,
        0,
        0,
        0.72222
      ],
      8874: [
        0,
        0.69224,
        0,
        0,
        0.88889
      ],
      8876: [
        0,
        0.68889,
        0,
        0,
        0.61111
      ],
      8877: [
        0,
        0.68889,
        0,
        0,
        0.61111
      ],
      8878: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      8879: [
        0,
        0.68889,
        0,
        0,
        0.72222
      ],
      8882: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      8883: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      8884: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      8885: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      8888: [
        0,
        0.54986,
        0,
        0,
        1.11111
      ],
      8890: [
        0.19444,
        0.43056,
        0,
        0,
        0.55556
      ],
      8891: [
        0.19444,
        0.69224,
        0,
        0,
        0.61111
      ],
      8892: [
        0.19444,
        0.69224,
        0,
        0,
        0.61111
      ],
      8901: [
        0,
        0.54986,
        0,
        0,
        0.27778
      ],
      8903: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      8905: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      8906: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      8907: [
        0,
        0.69224,
        0,
        0,
        0.77778
      ],
      8908: [
        0,
        0.69224,
        0,
        0,
        0.77778
      ],
      8909: [
        -0.03598,
        0.46402,
        0,
        0,
        0.77778
      ],
      8910: [
        0,
        0.54986,
        0,
        0,
        0.76042
      ],
      8911: [
        0,
        0.54986,
        0,
        0,
        0.76042
      ],
      8912: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      8913: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      8914: [
        0,
        0.54986,
        0,
        0,
        0.66667
      ],
      8915: [
        0,
        0.54986,
        0,
        0,
        0.66667
      ],
      8916: [
        0,
        0.69224,
        0,
        0,
        0.66667
      ],
      8918: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      8919: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      8920: [
        0.03517,
        0.54986,
        0,
        0,
        1.33334
      ],
      8921: [
        0.03517,
        0.54986,
        0,
        0,
        1.33334
      ],
      8922: [
        0.38569,
        0.88569,
        0,
        0,
        0.77778
      ],
      8923: [
        0.38569,
        0.88569,
        0,
        0,
        0.77778
      ],
      8926: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      8927: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      8928: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8929: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8934: [
        0.23222,
        0.74111,
        0,
        0,
        0.77778
      ],
      8935: [
        0.23222,
        0.74111,
        0,
        0,
        0.77778
      ],
      8936: [
        0.23222,
        0.74111,
        0,
        0,
        0.77778
      ],
      8937: [
        0.23222,
        0.74111,
        0,
        0,
        0.77778
      ],
      8938: [
        0.20576,
        0.70576,
        0,
        0,
        0.77778
      ],
      8939: [
        0.20576,
        0.70576,
        0,
        0,
        0.77778
      ],
      8940: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8941: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      8994: [
        0.19444,
        0.69224,
        0,
        0,
        0.77778
      ],
      8995: [
        0.19444,
        0.69224,
        0,
        0,
        0.77778
      ],
      9416: [
        0.15559,
        0.69224,
        0,
        0,
        0.90222
      ],
      9484: [
        0,
        0.69224,
        0,
        0,
        0.5
      ],
      9488: [
        0,
        0.69224,
        0,
        0,
        0.5
      ],
      9492: [
        0,
        0.37788,
        0,
        0,
        0.5
      ],
      9496: [
        0,
        0.37788,
        0,
        0,
        0.5
      ],
      9585: [
        0.19444,
        0.68889,
        0,
        0,
        0.88889
      ],
      9586: [
        0.19444,
        0.74111,
        0,
        0,
        0.88889
      ],
      9632: [
        0,
        0.675,
        0,
        0,
        0.77778
      ],
      9633: [
        0,
        0.675,
        0,
        0,
        0.77778
      ],
      9650: [
        0,
        0.54986,
        0,
        0,
        0.72222
      ],
      9651: [
        0,
        0.54986,
        0,
        0,
        0.72222
      ],
      9654: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      9660: [
        0,
        0.54986,
        0,
        0,
        0.72222
      ],
      9661: [
        0,
        0.54986,
        0,
        0,
        0.72222
      ],
      9664: [
        0.03517,
        0.54986,
        0,
        0,
        0.77778
      ],
      9674: [
        0.11111,
        0.69224,
        0,
        0,
        0.66667
      ],
      9733: [
        0.19444,
        0.69224,
        0,
        0,
        0.94445
      ],
      10003: [
        0,
        0.69224,
        0,
        0,
        0.83334
      ],
      10016: [
        0,
        0.69224,
        0,
        0,
        0.83334
      ],
      10731: [
        0.11111,
        0.69224,
        0,
        0,
        0.66667
      ],
      10846: [
        0.19444,
        0.75583,
        0,
        0,
        0.61111
      ],
      10877: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      10878: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      10885: [
        0.25583,
        0.75583,
        0,
        0,
        0.77778
      ],
      10886: [
        0.25583,
        0.75583,
        0,
        0,
        0.77778
      ],
      10887: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      10888: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      10889: [
        0.26167,
        0.75726,
        0,
        0,
        0.77778
      ],
      10890: [
        0.26167,
        0.75726,
        0,
        0,
        0.77778
      ],
      10891: [
        0.48256,
        0.98256,
        0,
        0,
        0.77778
      ],
      10892: [
        0.48256,
        0.98256,
        0,
        0,
        0.77778
      ],
      10901: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      10902: [
        0.13667,
        0.63667,
        0,
        0,
        0.77778
      ],
      10933: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      10934: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      10935: [
        0.26167,
        0.75726,
        0,
        0,
        0.77778
      ],
      10936: [
        0.26167,
        0.75726,
        0,
        0,
        0.77778
      ],
      10937: [
        0.26167,
        0.75726,
        0,
        0,
        0.77778
      ],
      10938: [
        0.26167,
        0.75726,
        0,
        0,
        0.77778
      ],
      10949: [
        0.25583,
        0.75583,
        0,
        0,
        0.77778
      ],
      10950: [
        0.25583,
        0.75583,
        0,
        0,
        0.77778
      ],
      10955: [
        0.28481,
        0.79383,
        0,
        0,
        0.77778
      ],
      10956: [
        0.28481,
        0.79383,
        0,
        0,
        0.77778
      ],
      57350: [
        0.08167,
        0.58167,
        0,
        0,
        0.22222
      ],
      57351: [
        0.08167,
        0.58167,
        0,
        0,
        0.38889
      ],
      57352: [
        0.08167,
        0.58167,
        0,
        0,
        0.77778
      ],
      57353: [
        0,
        0.43056,
        0.04028,
        0,
        0.66667
      ],
      57356: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      57357: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      57358: [
        0.41951,
        0.91951,
        0,
        0,
        0.77778
      ],
      57359: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      57360: [
        0.30274,
        0.79383,
        0,
        0,
        0.77778
      ],
      57361: [
        0.41951,
        0.91951,
        0,
        0,
        0.77778
      ],
      57366: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      57367: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      57368: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      57369: [
        0.25142,
        0.75726,
        0,
        0,
        0.77778
      ],
      57370: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      57371: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ]
    },
    "Caligraphic-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      65: [
        0,
        0.68333,
        0,
        0.19445,
        0.79847
      ],
      66: [
        0,
        0.68333,
        0.03041,
        0.13889,
        0.65681
      ],
      67: [
        0,
        0.68333,
        0.05834,
        0.13889,
        0.52653
      ],
      68: [
        0,
        0.68333,
        0.02778,
        0.08334,
        0.77139
      ],
      69: [
        0,
        0.68333,
        0.08944,
        0.11111,
        0.52778
      ],
      70: [
        0,
        0.68333,
        0.09931,
        0.11111,
        0.71875
      ],
      71: [
        0.09722,
        0.68333,
        0.0593,
        0.11111,
        0.59487
      ],
      72: [
        0,
        0.68333,
        965e-5,
        0.11111,
        0.84452
      ],
      73: [
        0,
        0.68333,
        0.07382,
        0,
        0.54452
      ],
      74: [
        0.09722,
        0.68333,
        0.18472,
        0.16667,
        0.67778
      ],
      75: [
        0,
        0.68333,
        0.01445,
        0.05556,
        0.76195
      ],
      76: [
        0,
        0.68333,
        0,
        0.13889,
        0.68972
      ],
      77: [
        0,
        0.68333,
        0,
        0.13889,
        1.2009
      ],
      78: [
        0,
        0.68333,
        0.14736,
        0.08334,
        0.82049
      ],
      79: [
        0,
        0.68333,
        0.02778,
        0.11111,
        0.79611
      ],
      80: [
        0,
        0.68333,
        0.08222,
        0.08334,
        0.69556
      ],
      81: [
        0.09722,
        0.68333,
        0,
        0.11111,
        0.81667
      ],
      82: [
        0,
        0.68333,
        0,
        0.08334,
        0.8475
      ],
      83: [
        0,
        0.68333,
        0.075,
        0.13889,
        0.60556
      ],
      84: [
        0,
        0.68333,
        0.25417,
        0,
        0.54464
      ],
      85: [
        0,
        0.68333,
        0.09931,
        0.08334,
        0.62583
      ],
      86: [
        0,
        0.68333,
        0.08222,
        0,
        0.61278
      ],
      87: [
        0,
        0.68333,
        0.08222,
        0.08334,
        0.98778
      ],
      88: [
        0,
        0.68333,
        0.14643,
        0.13889,
        0.7133
      ],
      89: [
        0.09722,
        0.68333,
        0.08222,
        0.08334,
        0.66834
      ],
      90: [
        0,
        0.68333,
        0.07944,
        0.13889,
        0.72473
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ]
    },
    "Fraktur-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69141,
        0,
        0,
        0.29574
      ],
      34: [
        0,
        0.69141,
        0,
        0,
        0.21471
      ],
      38: [
        0,
        0.69141,
        0,
        0,
        0.73786
      ],
      39: [
        0,
        0.69141,
        0,
        0,
        0.21201
      ],
      40: [
        0.24982,
        0.74947,
        0,
        0,
        0.38865
      ],
      41: [
        0.24982,
        0.74947,
        0,
        0,
        0.38865
      ],
      42: [
        0,
        0.62119,
        0,
        0,
        0.27764
      ],
      43: [
        0.08319,
        0.58283,
        0,
        0,
        0.75623
      ],
      44: [
        0,
        0.10803,
        0,
        0,
        0.27764
      ],
      45: [
        0.08319,
        0.58283,
        0,
        0,
        0.75623
      ],
      46: [
        0,
        0.10803,
        0,
        0,
        0.27764
      ],
      47: [
        0.24982,
        0.74947,
        0,
        0,
        0.50181
      ],
      48: [
        0,
        0.47534,
        0,
        0,
        0.50181
      ],
      49: [
        0,
        0.47534,
        0,
        0,
        0.50181
      ],
      50: [
        0,
        0.47534,
        0,
        0,
        0.50181
      ],
      51: [
        0.18906,
        0.47534,
        0,
        0,
        0.50181
      ],
      52: [
        0.18906,
        0.47534,
        0,
        0,
        0.50181
      ],
      53: [
        0.18906,
        0.47534,
        0,
        0,
        0.50181
      ],
      54: [
        0,
        0.69141,
        0,
        0,
        0.50181
      ],
      55: [
        0.18906,
        0.47534,
        0,
        0,
        0.50181
      ],
      56: [
        0,
        0.69141,
        0,
        0,
        0.50181
      ],
      57: [
        0.18906,
        0.47534,
        0,
        0,
        0.50181
      ],
      58: [
        0,
        0.47534,
        0,
        0,
        0.21606
      ],
      59: [
        0.12604,
        0.47534,
        0,
        0,
        0.21606
      ],
      61: [
        -0.13099,
        0.36866,
        0,
        0,
        0.75623
      ],
      63: [
        0,
        0.69141,
        0,
        0,
        0.36245
      ],
      65: [
        0,
        0.69141,
        0,
        0,
        0.7176
      ],
      66: [
        0,
        0.69141,
        0,
        0,
        0.88397
      ],
      67: [
        0,
        0.69141,
        0,
        0,
        0.61254
      ],
      68: [
        0,
        0.69141,
        0,
        0,
        0.83158
      ],
      69: [
        0,
        0.69141,
        0,
        0,
        0.66278
      ],
      70: [
        0.12604,
        0.69141,
        0,
        0,
        0.61119
      ],
      71: [
        0,
        0.69141,
        0,
        0,
        0.78539
      ],
      72: [
        0.06302,
        0.69141,
        0,
        0,
        0.7203
      ],
      73: [
        0,
        0.69141,
        0,
        0,
        0.55448
      ],
      74: [
        0.12604,
        0.69141,
        0,
        0,
        0.55231
      ],
      75: [
        0,
        0.69141,
        0,
        0,
        0.66845
      ],
      76: [
        0,
        0.69141,
        0,
        0,
        0.66602
      ],
      77: [
        0,
        0.69141,
        0,
        0,
        1.04953
      ],
      78: [
        0,
        0.69141,
        0,
        0,
        0.83212
      ],
      79: [
        0,
        0.69141,
        0,
        0,
        0.82699
      ],
      80: [
        0.18906,
        0.69141,
        0,
        0,
        0.82753
      ],
      81: [
        0.03781,
        0.69141,
        0,
        0,
        0.82699
      ],
      82: [
        0,
        0.69141,
        0,
        0,
        0.82807
      ],
      83: [
        0,
        0.69141,
        0,
        0,
        0.82861
      ],
      84: [
        0,
        0.69141,
        0,
        0,
        0.66899
      ],
      85: [
        0,
        0.69141,
        0,
        0,
        0.64576
      ],
      86: [
        0,
        0.69141,
        0,
        0,
        0.83131
      ],
      87: [
        0,
        0.69141,
        0,
        0,
        1.04602
      ],
      88: [
        0,
        0.69141,
        0,
        0,
        0.71922
      ],
      89: [
        0.18906,
        0.69141,
        0,
        0,
        0.83293
      ],
      90: [
        0.12604,
        0.69141,
        0,
        0,
        0.60201
      ],
      91: [
        0.24982,
        0.74947,
        0,
        0,
        0.27764
      ],
      93: [
        0.24982,
        0.74947,
        0,
        0,
        0.27764
      ],
      94: [
        0,
        0.69141,
        0,
        0,
        0.49965
      ],
      97: [
        0,
        0.47534,
        0,
        0,
        0.50046
      ],
      98: [
        0,
        0.69141,
        0,
        0,
        0.51315
      ],
      99: [
        0,
        0.47534,
        0,
        0,
        0.38946
      ],
      100: [
        0,
        0.62119,
        0,
        0,
        0.49857
      ],
      101: [
        0,
        0.47534,
        0,
        0,
        0.40053
      ],
      102: [
        0.18906,
        0.69141,
        0,
        0,
        0.32626
      ],
      103: [
        0.18906,
        0.47534,
        0,
        0,
        0.5037
      ],
      104: [
        0.18906,
        0.69141,
        0,
        0,
        0.52126
      ],
      105: [
        0,
        0.69141,
        0,
        0,
        0.27899
      ],
      106: [
        0,
        0.69141,
        0,
        0,
        0.28088
      ],
      107: [
        0,
        0.69141,
        0,
        0,
        0.38946
      ],
      108: [
        0,
        0.69141,
        0,
        0,
        0.27953
      ],
      109: [
        0,
        0.47534,
        0,
        0,
        0.76676
      ],
      110: [
        0,
        0.47534,
        0,
        0,
        0.52666
      ],
      111: [
        0,
        0.47534,
        0,
        0,
        0.48885
      ],
      112: [
        0.18906,
        0.52396,
        0,
        0,
        0.50046
      ],
      113: [
        0.18906,
        0.47534,
        0,
        0,
        0.48912
      ],
      114: [
        0,
        0.47534,
        0,
        0,
        0.38919
      ],
      115: [
        0,
        0.47534,
        0,
        0,
        0.44266
      ],
      116: [
        0,
        0.62119,
        0,
        0,
        0.33301
      ],
      117: [
        0,
        0.47534,
        0,
        0,
        0.5172
      ],
      118: [
        0,
        0.52396,
        0,
        0,
        0.5118
      ],
      119: [
        0,
        0.52396,
        0,
        0,
        0.77351
      ],
      120: [
        0.18906,
        0.47534,
        0,
        0,
        0.38865
      ],
      121: [
        0.18906,
        0.47534,
        0,
        0,
        0.49884
      ],
      122: [
        0.18906,
        0.47534,
        0,
        0,
        0.39054
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      8216: [
        0,
        0.69141,
        0,
        0,
        0.21471
      ],
      8217: [
        0,
        0.69141,
        0,
        0,
        0.21471
      ],
      58112: [
        0,
        0.62119,
        0,
        0,
        0.49749
      ],
      58113: [
        0,
        0.62119,
        0,
        0,
        0.4983
      ],
      58114: [
        0.18906,
        0.69141,
        0,
        0,
        0.33328
      ],
      58115: [
        0.18906,
        0.69141,
        0,
        0,
        0.32923
      ],
      58116: [
        0.18906,
        0.47534,
        0,
        0,
        0.50343
      ],
      58117: [
        0,
        0.69141,
        0,
        0,
        0.33301
      ],
      58118: [
        0,
        0.62119,
        0,
        0,
        0.33409
      ],
      58119: [
        0,
        0.47534,
        0,
        0,
        0.50073
      ]
    },
    "Main-Bold": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69444,
        0,
        0,
        0.35
      ],
      34: [
        0,
        0.69444,
        0,
        0,
        0.60278
      ],
      35: [
        0.19444,
        0.69444,
        0,
        0,
        0.95833
      ],
      36: [
        0.05556,
        0.75,
        0,
        0,
        0.575
      ],
      37: [
        0.05556,
        0.75,
        0,
        0,
        0.95833
      ],
      38: [
        0,
        0.69444,
        0,
        0,
        0.89444
      ],
      39: [
        0,
        0.69444,
        0,
        0,
        0.31944
      ],
      40: [
        0.25,
        0.75,
        0,
        0,
        0.44722
      ],
      41: [
        0.25,
        0.75,
        0,
        0,
        0.44722
      ],
      42: [
        0,
        0.75,
        0,
        0,
        0.575
      ],
      43: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      44: [
        0.19444,
        0.15556,
        0,
        0,
        0.31944
      ],
      45: [
        0,
        0.44444,
        0,
        0,
        0.38333
      ],
      46: [
        0,
        0.15556,
        0,
        0,
        0.31944
      ],
      47: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      48: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      49: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      50: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      51: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      52: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      53: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      54: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      55: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      56: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      57: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      58: [
        0,
        0.44444,
        0,
        0,
        0.31944
      ],
      59: [
        0.19444,
        0.44444,
        0,
        0,
        0.31944
      ],
      60: [
        0.08556,
        0.58556,
        0,
        0,
        0.89444
      ],
      61: [
        -0.10889,
        0.39111,
        0,
        0,
        0.89444
      ],
      62: [
        0.08556,
        0.58556,
        0,
        0,
        0.89444
      ],
      63: [
        0,
        0.69444,
        0,
        0,
        0.54305
      ],
      64: [
        0,
        0.69444,
        0,
        0,
        0.89444
      ],
      65: [
        0,
        0.68611,
        0,
        0,
        0.86944
      ],
      66: [
        0,
        0.68611,
        0,
        0,
        0.81805
      ],
      67: [
        0,
        0.68611,
        0,
        0,
        0.83055
      ],
      68: [
        0,
        0.68611,
        0,
        0,
        0.88194
      ],
      69: [
        0,
        0.68611,
        0,
        0,
        0.75555
      ],
      70: [
        0,
        0.68611,
        0,
        0,
        0.72361
      ],
      71: [
        0,
        0.68611,
        0,
        0,
        0.90416
      ],
      72: [
        0,
        0.68611,
        0,
        0,
        0.9
      ],
      73: [
        0,
        0.68611,
        0,
        0,
        0.43611
      ],
      74: [
        0,
        0.68611,
        0,
        0,
        0.59444
      ],
      75: [
        0,
        0.68611,
        0,
        0,
        0.90138
      ],
      76: [
        0,
        0.68611,
        0,
        0,
        0.69166
      ],
      77: [
        0,
        0.68611,
        0,
        0,
        1.09166
      ],
      78: [
        0,
        0.68611,
        0,
        0,
        0.9
      ],
      79: [
        0,
        0.68611,
        0,
        0,
        0.86388
      ],
      80: [
        0,
        0.68611,
        0,
        0,
        0.78611
      ],
      81: [
        0.19444,
        0.68611,
        0,
        0,
        0.86388
      ],
      82: [
        0,
        0.68611,
        0,
        0,
        0.8625
      ],
      83: [
        0,
        0.68611,
        0,
        0,
        0.63889
      ],
      84: [
        0,
        0.68611,
        0,
        0,
        0.8
      ],
      85: [
        0,
        0.68611,
        0,
        0,
        0.88472
      ],
      86: [
        0,
        0.68611,
        0.01597,
        0,
        0.86944
      ],
      87: [
        0,
        0.68611,
        0.01597,
        0,
        1.18888
      ],
      88: [
        0,
        0.68611,
        0,
        0,
        0.86944
      ],
      89: [
        0,
        0.68611,
        0.02875,
        0,
        0.86944
      ],
      90: [
        0,
        0.68611,
        0,
        0,
        0.70277
      ],
      91: [
        0.25,
        0.75,
        0,
        0,
        0.31944
      ],
      92: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      93: [
        0.25,
        0.75,
        0,
        0,
        0.31944
      ],
      94: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      95: [
        0.31,
        0.13444,
        0.03194,
        0,
        0.575
      ],
      97: [
        0,
        0.44444,
        0,
        0,
        0.55902
      ],
      98: [
        0,
        0.69444,
        0,
        0,
        0.63889
      ],
      99: [
        0,
        0.44444,
        0,
        0,
        0.51111
      ],
      100: [
        0,
        0.69444,
        0,
        0,
        0.63889
      ],
      101: [
        0,
        0.44444,
        0,
        0,
        0.52708
      ],
      102: [
        0,
        0.69444,
        0.10903,
        0,
        0.35139
      ],
      103: [
        0.19444,
        0.44444,
        0.01597,
        0,
        0.575
      ],
      104: [
        0,
        0.69444,
        0,
        0,
        0.63889
      ],
      105: [
        0,
        0.69444,
        0,
        0,
        0.31944
      ],
      106: [
        0.19444,
        0.69444,
        0,
        0,
        0.35139
      ],
      107: [
        0,
        0.69444,
        0,
        0,
        0.60694
      ],
      108: [
        0,
        0.69444,
        0,
        0,
        0.31944
      ],
      109: [
        0,
        0.44444,
        0,
        0,
        0.95833
      ],
      110: [
        0,
        0.44444,
        0,
        0,
        0.63889
      ],
      111: [
        0,
        0.44444,
        0,
        0,
        0.575
      ],
      112: [
        0.19444,
        0.44444,
        0,
        0,
        0.63889
      ],
      113: [
        0.19444,
        0.44444,
        0,
        0,
        0.60694
      ],
      114: [
        0,
        0.44444,
        0,
        0,
        0.47361
      ],
      115: [
        0,
        0.44444,
        0,
        0,
        0.45361
      ],
      116: [
        0,
        0.63492,
        0,
        0,
        0.44722
      ],
      117: [
        0,
        0.44444,
        0,
        0,
        0.63889
      ],
      118: [
        0,
        0.44444,
        0.01597,
        0,
        0.60694
      ],
      119: [
        0,
        0.44444,
        0.01597,
        0,
        0.83055
      ],
      120: [
        0,
        0.44444,
        0,
        0,
        0.60694
      ],
      121: [
        0.19444,
        0.44444,
        0.01597,
        0,
        0.60694
      ],
      122: [
        0,
        0.44444,
        0,
        0,
        0.51111
      ],
      123: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      124: [
        0.25,
        0.75,
        0,
        0,
        0.31944
      ],
      125: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      126: [
        0.35,
        0.34444,
        0,
        0,
        0.575
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      163: [
        0,
        0.69444,
        0,
        0,
        0.86853
      ],
      168: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      172: [
        0,
        0.44444,
        0,
        0,
        0.76666
      ],
      176: [
        0,
        0.69444,
        0,
        0,
        0.86944
      ],
      177: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      184: [
        0.17014,
        0,
        0,
        0,
        0.51111
      ],
      198: [
        0,
        0.68611,
        0,
        0,
        1.04166
      ],
      215: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      216: [
        0.04861,
        0.73472,
        0,
        0,
        0.89444
      ],
      223: [
        0,
        0.69444,
        0,
        0,
        0.59722
      ],
      230: [
        0,
        0.44444,
        0,
        0,
        0.83055
      ],
      247: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      248: [
        0.09722,
        0.54167,
        0,
        0,
        0.575
      ],
      305: [
        0,
        0.44444,
        0,
        0,
        0.31944
      ],
      338: [
        0,
        0.68611,
        0,
        0,
        1.16944
      ],
      339: [
        0,
        0.44444,
        0,
        0,
        0.89444
      ],
      567: [
        0.19444,
        0.44444,
        0,
        0,
        0.35139
      ],
      710: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      711: [
        0,
        0.63194,
        0,
        0,
        0.575
      ],
      713: [
        0,
        0.59611,
        0,
        0,
        0.575
      ],
      714: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      715: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      728: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      729: [
        0,
        0.69444,
        0,
        0,
        0.31944
      ],
      730: [
        0,
        0.69444,
        0,
        0,
        0.86944
      ],
      732: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      733: [
        0,
        0.69444,
        0,
        0,
        0.575
      ],
      915: [
        0,
        0.68611,
        0,
        0,
        0.69166
      ],
      916: [
        0,
        0.68611,
        0,
        0,
        0.95833
      ],
      920: [
        0,
        0.68611,
        0,
        0,
        0.89444
      ],
      923: [
        0,
        0.68611,
        0,
        0,
        0.80555
      ],
      926: [
        0,
        0.68611,
        0,
        0,
        0.76666
      ],
      928: [
        0,
        0.68611,
        0,
        0,
        0.9
      ],
      931: [
        0,
        0.68611,
        0,
        0,
        0.83055
      ],
      933: [
        0,
        0.68611,
        0,
        0,
        0.89444
      ],
      934: [
        0,
        0.68611,
        0,
        0,
        0.83055
      ],
      936: [
        0,
        0.68611,
        0,
        0,
        0.89444
      ],
      937: [
        0,
        0.68611,
        0,
        0,
        0.83055
      ],
      8211: [
        0,
        0.44444,
        0.03194,
        0,
        0.575
      ],
      8212: [
        0,
        0.44444,
        0.03194,
        0,
        1.14999
      ],
      8216: [
        0,
        0.69444,
        0,
        0,
        0.31944
      ],
      8217: [
        0,
        0.69444,
        0,
        0,
        0.31944
      ],
      8220: [
        0,
        0.69444,
        0,
        0,
        0.60278
      ],
      8221: [
        0,
        0.69444,
        0,
        0,
        0.60278
      ],
      8224: [
        0.19444,
        0.69444,
        0,
        0,
        0.51111
      ],
      8225: [
        0.19444,
        0.69444,
        0,
        0,
        0.51111
      ],
      8242: [
        0,
        0.55556,
        0,
        0,
        0.34444
      ],
      8407: [
        0,
        0.72444,
        0.15486,
        0,
        0.575
      ],
      8463: [
        0,
        0.69444,
        0,
        0,
        0.66759
      ],
      8465: [
        0,
        0.69444,
        0,
        0,
        0.83055
      ],
      8467: [
        0,
        0.69444,
        0,
        0,
        0.47361
      ],
      8472: [
        0.19444,
        0.44444,
        0,
        0,
        0.74027
      ],
      8476: [
        0,
        0.69444,
        0,
        0,
        0.83055
      ],
      8501: [
        0,
        0.69444,
        0,
        0,
        0.70277
      ],
      8592: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8593: [
        0.19444,
        0.69444,
        0,
        0,
        0.575
      ],
      8594: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8595: [
        0.19444,
        0.69444,
        0,
        0,
        0.575
      ],
      8596: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8597: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      8598: [
        0.19444,
        0.69444,
        0,
        0,
        1.14999
      ],
      8599: [
        0.19444,
        0.69444,
        0,
        0,
        1.14999
      ],
      8600: [
        0.19444,
        0.69444,
        0,
        0,
        1.14999
      ],
      8601: [
        0.19444,
        0.69444,
        0,
        0,
        1.14999
      ],
      8636: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8637: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8640: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8641: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8656: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8657: [
        0.19444,
        0.69444,
        0,
        0,
        0.70277
      ],
      8658: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8659: [
        0.19444,
        0.69444,
        0,
        0,
        0.70277
      ],
      8660: [
        -0.10889,
        0.39111,
        0,
        0,
        1.14999
      ],
      8661: [
        0.25,
        0.75,
        0,
        0,
        0.70277
      ],
      8704: [
        0,
        0.69444,
        0,
        0,
        0.63889
      ],
      8706: [
        0,
        0.69444,
        0.06389,
        0,
        0.62847
      ],
      8707: [
        0,
        0.69444,
        0,
        0,
        0.63889
      ],
      8709: [
        0.05556,
        0.75,
        0,
        0,
        0.575
      ],
      8711: [
        0,
        0.68611,
        0,
        0,
        0.95833
      ],
      8712: [
        0.08556,
        0.58556,
        0,
        0,
        0.76666
      ],
      8715: [
        0.08556,
        0.58556,
        0,
        0,
        0.76666
      ],
      8722: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      8723: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      8725: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      8726: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      8727: [
        -0.02778,
        0.47222,
        0,
        0,
        0.575
      ],
      8728: [
        -0.02639,
        0.47361,
        0,
        0,
        0.575
      ],
      8729: [
        -0.02639,
        0.47361,
        0,
        0,
        0.575
      ],
      8730: [
        0.18,
        0.82,
        0,
        0,
        0.95833
      ],
      8733: [
        0,
        0.44444,
        0,
        0,
        0.89444
      ],
      8734: [
        0,
        0.44444,
        0,
        0,
        1.14999
      ],
      8736: [
        0,
        0.69224,
        0,
        0,
        0.72222
      ],
      8739: [
        0.25,
        0.75,
        0,
        0,
        0.31944
      ],
      8741: [
        0.25,
        0.75,
        0,
        0,
        0.575
      ],
      8743: [
        0,
        0.55556,
        0,
        0,
        0.76666
      ],
      8744: [
        0,
        0.55556,
        0,
        0,
        0.76666
      ],
      8745: [
        0,
        0.55556,
        0,
        0,
        0.76666
      ],
      8746: [
        0,
        0.55556,
        0,
        0,
        0.76666
      ],
      8747: [
        0.19444,
        0.69444,
        0.12778,
        0,
        0.56875
      ],
      8764: [
        -0.10889,
        0.39111,
        0,
        0,
        0.89444
      ],
      8768: [
        0.19444,
        0.69444,
        0,
        0,
        0.31944
      ],
      8771: [
        222e-5,
        0.50222,
        0,
        0,
        0.89444
      ],
      8773: [
        0.027,
        0.638,
        0,
        0,
        0.894
      ],
      8776: [
        0.02444,
        0.52444,
        0,
        0,
        0.89444
      ],
      8781: [
        222e-5,
        0.50222,
        0,
        0,
        0.89444
      ],
      8801: [
        222e-5,
        0.50222,
        0,
        0,
        0.89444
      ],
      8804: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      8805: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      8810: [
        0.08556,
        0.58556,
        0,
        0,
        1.14999
      ],
      8811: [
        0.08556,
        0.58556,
        0,
        0,
        1.14999
      ],
      8826: [
        0.08556,
        0.58556,
        0,
        0,
        0.89444
      ],
      8827: [
        0.08556,
        0.58556,
        0,
        0,
        0.89444
      ],
      8834: [
        0.08556,
        0.58556,
        0,
        0,
        0.89444
      ],
      8835: [
        0.08556,
        0.58556,
        0,
        0,
        0.89444
      ],
      8838: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      8839: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      8846: [
        0,
        0.55556,
        0,
        0,
        0.76666
      ],
      8849: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      8850: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      8851: [
        0,
        0.55556,
        0,
        0,
        0.76666
      ],
      8852: [
        0,
        0.55556,
        0,
        0,
        0.76666
      ],
      8853: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      8854: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      8855: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      8856: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      8857: [
        0.13333,
        0.63333,
        0,
        0,
        0.89444
      ],
      8866: [
        0,
        0.69444,
        0,
        0,
        0.70277
      ],
      8867: [
        0,
        0.69444,
        0,
        0,
        0.70277
      ],
      8868: [
        0,
        0.69444,
        0,
        0,
        0.89444
      ],
      8869: [
        0,
        0.69444,
        0,
        0,
        0.89444
      ],
      8900: [
        -0.02639,
        0.47361,
        0,
        0,
        0.575
      ],
      8901: [
        -0.02639,
        0.47361,
        0,
        0,
        0.31944
      ],
      8902: [
        -0.02778,
        0.47222,
        0,
        0,
        0.575
      ],
      8968: [
        0.25,
        0.75,
        0,
        0,
        0.51111
      ],
      8969: [
        0.25,
        0.75,
        0,
        0,
        0.51111
      ],
      8970: [
        0.25,
        0.75,
        0,
        0,
        0.51111
      ],
      8971: [
        0.25,
        0.75,
        0,
        0,
        0.51111
      ],
      8994: [
        -0.13889,
        0.36111,
        0,
        0,
        1.14999
      ],
      8995: [
        -0.13889,
        0.36111,
        0,
        0,
        1.14999
      ],
      9651: [
        0.19444,
        0.69444,
        0,
        0,
        1.02222
      ],
      9657: [
        -0.02778,
        0.47222,
        0,
        0,
        0.575
      ],
      9661: [
        0.19444,
        0.69444,
        0,
        0,
        1.02222
      ],
      9667: [
        -0.02778,
        0.47222,
        0,
        0,
        0.575
      ],
      9711: [
        0.19444,
        0.69444,
        0,
        0,
        1.14999
      ],
      9824: [
        0.12963,
        0.69444,
        0,
        0,
        0.89444
      ],
      9825: [
        0.12963,
        0.69444,
        0,
        0,
        0.89444
      ],
      9826: [
        0.12963,
        0.69444,
        0,
        0,
        0.89444
      ],
      9827: [
        0.12963,
        0.69444,
        0,
        0,
        0.89444
      ],
      9837: [
        0,
        0.75,
        0,
        0,
        0.44722
      ],
      9838: [
        0.19444,
        0.69444,
        0,
        0,
        0.44722
      ],
      9839: [
        0.19444,
        0.69444,
        0,
        0,
        0.44722
      ],
      10216: [
        0.25,
        0.75,
        0,
        0,
        0.44722
      ],
      10217: [
        0.25,
        0.75,
        0,
        0,
        0.44722
      ],
      10815: [
        0,
        0.68611,
        0,
        0,
        0.9
      ],
      10927: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      10928: [
        0.19667,
        0.69667,
        0,
        0,
        0.89444
      ],
      57376: [
        0.19444,
        0.69444,
        0,
        0,
        0
      ]
    },
    "Main-BoldItalic": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69444,
        0.11417,
        0,
        0.38611
      ],
      34: [
        0,
        0.69444,
        0.07939,
        0,
        0.62055
      ],
      35: [
        0.19444,
        0.69444,
        0.06833,
        0,
        0.94444
      ],
      37: [
        0.05556,
        0.75,
        0.12861,
        0,
        0.94444
      ],
      38: [
        0,
        0.69444,
        0.08528,
        0,
        0.88555
      ],
      39: [
        0,
        0.69444,
        0.12945,
        0,
        0.35555
      ],
      40: [
        0.25,
        0.75,
        0.15806,
        0,
        0.47333
      ],
      41: [
        0.25,
        0.75,
        0.03306,
        0,
        0.47333
      ],
      42: [
        0,
        0.75,
        0.14333,
        0,
        0.59111
      ],
      43: [
        0.10333,
        0.60333,
        0.03306,
        0,
        0.88555
      ],
      44: [
        0.19444,
        0.14722,
        0,
        0,
        0.35555
      ],
      45: [
        0,
        0.44444,
        0.02611,
        0,
        0.41444
      ],
      46: [
        0,
        0.14722,
        0,
        0,
        0.35555
      ],
      47: [
        0.25,
        0.75,
        0.15806,
        0,
        0.59111
      ],
      48: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      49: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      50: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      51: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      52: [
        0.19444,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      53: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      54: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      55: [
        0.19444,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      56: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      57: [
        0,
        0.64444,
        0.13167,
        0,
        0.59111
      ],
      58: [
        0,
        0.44444,
        0.06695,
        0,
        0.35555
      ],
      59: [
        0.19444,
        0.44444,
        0.06695,
        0,
        0.35555
      ],
      61: [
        -0.10889,
        0.39111,
        0.06833,
        0,
        0.88555
      ],
      63: [
        0,
        0.69444,
        0.11472,
        0,
        0.59111
      ],
      64: [
        0,
        0.69444,
        0.09208,
        0,
        0.88555
      ],
      65: [
        0,
        0.68611,
        0,
        0,
        0.86555
      ],
      66: [
        0,
        0.68611,
        0.0992,
        0,
        0.81666
      ],
      67: [
        0,
        0.68611,
        0.14208,
        0,
        0.82666
      ],
      68: [
        0,
        0.68611,
        0.09062,
        0,
        0.87555
      ],
      69: [
        0,
        0.68611,
        0.11431,
        0,
        0.75666
      ],
      70: [
        0,
        0.68611,
        0.12903,
        0,
        0.72722
      ],
      71: [
        0,
        0.68611,
        0.07347,
        0,
        0.89527
      ],
      72: [
        0,
        0.68611,
        0.17208,
        0,
        0.8961
      ],
      73: [
        0,
        0.68611,
        0.15681,
        0,
        0.47166
      ],
      74: [
        0,
        0.68611,
        0.145,
        0,
        0.61055
      ],
      75: [
        0,
        0.68611,
        0.14208,
        0,
        0.89499
      ],
      76: [
        0,
        0.68611,
        0,
        0,
        0.69777
      ],
      77: [
        0,
        0.68611,
        0.17208,
        0,
        1.07277
      ],
      78: [
        0,
        0.68611,
        0.17208,
        0,
        0.8961
      ],
      79: [
        0,
        0.68611,
        0.09062,
        0,
        0.85499
      ],
      80: [
        0,
        0.68611,
        0.0992,
        0,
        0.78721
      ],
      81: [
        0.19444,
        0.68611,
        0.09062,
        0,
        0.85499
      ],
      82: [
        0,
        0.68611,
        0.02559,
        0,
        0.85944
      ],
      83: [
        0,
        0.68611,
        0.11264,
        0,
        0.64999
      ],
      84: [
        0,
        0.68611,
        0.12903,
        0,
        0.7961
      ],
      85: [
        0,
        0.68611,
        0.17208,
        0,
        0.88083
      ],
      86: [
        0,
        0.68611,
        0.18625,
        0,
        0.86555
      ],
      87: [
        0,
        0.68611,
        0.18625,
        0,
        1.15999
      ],
      88: [
        0,
        0.68611,
        0.15681,
        0,
        0.86555
      ],
      89: [
        0,
        0.68611,
        0.19803,
        0,
        0.86555
      ],
      90: [
        0,
        0.68611,
        0.14208,
        0,
        0.70888
      ],
      91: [
        0.25,
        0.75,
        0.1875,
        0,
        0.35611
      ],
      93: [
        0.25,
        0.75,
        0.09972,
        0,
        0.35611
      ],
      94: [
        0,
        0.69444,
        0.06709,
        0,
        0.59111
      ],
      95: [
        0.31,
        0.13444,
        0.09811,
        0,
        0.59111
      ],
      97: [
        0,
        0.44444,
        0.09426,
        0,
        0.59111
      ],
      98: [
        0,
        0.69444,
        0.07861,
        0,
        0.53222
      ],
      99: [
        0,
        0.44444,
        0.05222,
        0,
        0.53222
      ],
      100: [
        0,
        0.69444,
        0.10861,
        0,
        0.59111
      ],
      101: [
        0,
        0.44444,
        0.085,
        0,
        0.53222
      ],
      102: [
        0.19444,
        0.69444,
        0.21778,
        0,
        0.4
      ],
      103: [
        0.19444,
        0.44444,
        0.105,
        0,
        0.53222
      ],
      104: [
        0,
        0.69444,
        0.09426,
        0,
        0.59111
      ],
      105: [
        0,
        0.69326,
        0.11387,
        0,
        0.35555
      ],
      106: [
        0.19444,
        0.69326,
        0.1672,
        0,
        0.35555
      ],
      107: [
        0,
        0.69444,
        0.11111,
        0,
        0.53222
      ],
      108: [
        0,
        0.69444,
        0.10861,
        0,
        0.29666
      ],
      109: [
        0,
        0.44444,
        0.09426,
        0,
        0.94444
      ],
      110: [
        0,
        0.44444,
        0.09426,
        0,
        0.64999
      ],
      111: [
        0,
        0.44444,
        0.07861,
        0,
        0.59111
      ],
      112: [
        0.19444,
        0.44444,
        0.07861,
        0,
        0.59111
      ],
      113: [
        0.19444,
        0.44444,
        0.105,
        0,
        0.53222
      ],
      114: [
        0,
        0.44444,
        0.11111,
        0,
        0.50167
      ],
      115: [
        0,
        0.44444,
        0.08167,
        0,
        0.48694
      ],
      116: [
        0,
        0.63492,
        0.09639,
        0,
        0.385
      ],
      117: [
        0,
        0.44444,
        0.09426,
        0,
        0.62055
      ],
      118: [
        0,
        0.44444,
        0.11111,
        0,
        0.53222
      ],
      119: [
        0,
        0.44444,
        0.11111,
        0,
        0.76777
      ],
      120: [
        0,
        0.44444,
        0.12583,
        0,
        0.56055
      ],
      121: [
        0.19444,
        0.44444,
        0.105,
        0,
        0.56166
      ],
      122: [
        0,
        0.44444,
        0.13889,
        0,
        0.49055
      ],
      126: [
        0.35,
        0.34444,
        0.11472,
        0,
        0.59111
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      168: [
        0,
        0.69444,
        0.11473,
        0,
        0.59111
      ],
      176: [
        0,
        0.69444,
        0,
        0,
        0.94888
      ],
      184: [
        0.17014,
        0,
        0,
        0,
        0.53222
      ],
      198: [
        0,
        0.68611,
        0.11431,
        0,
        1.02277
      ],
      216: [
        0.04861,
        0.73472,
        0.09062,
        0,
        0.88555
      ],
      223: [
        0.19444,
        0.69444,
        0.09736,
        0,
        0.665
      ],
      230: [
        0,
        0.44444,
        0.085,
        0,
        0.82666
      ],
      248: [
        0.09722,
        0.54167,
        0.09458,
        0,
        0.59111
      ],
      305: [
        0,
        0.44444,
        0.09426,
        0,
        0.35555
      ],
      338: [
        0,
        0.68611,
        0.11431,
        0,
        1.14054
      ],
      339: [
        0,
        0.44444,
        0.085,
        0,
        0.82666
      ],
      567: [
        0.19444,
        0.44444,
        0.04611,
        0,
        0.385
      ],
      710: [
        0,
        0.69444,
        0.06709,
        0,
        0.59111
      ],
      711: [
        0,
        0.63194,
        0.08271,
        0,
        0.59111
      ],
      713: [
        0,
        0.59444,
        0.10444,
        0,
        0.59111
      ],
      714: [
        0,
        0.69444,
        0.08528,
        0,
        0.59111
      ],
      715: [
        0,
        0.69444,
        0,
        0,
        0.59111
      ],
      728: [
        0,
        0.69444,
        0.10333,
        0,
        0.59111
      ],
      729: [
        0,
        0.69444,
        0.12945,
        0,
        0.35555
      ],
      730: [
        0,
        0.69444,
        0,
        0,
        0.94888
      ],
      732: [
        0,
        0.69444,
        0.11472,
        0,
        0.59111
      ],
      733: [
        0,
        0.69444,
        0.11472,
        0,
        0.59111
      ],
      915: [
        0,
        0.68611,
        0.12903,
        0,
        0.69777
      ],
      916: [
        0,
        0.68611,
        0,
        0,
        0.94444
      ],
      920: [
        0,
        0.68611,
        0.09062,
        0,
        0.88555
      ],
      923: [
        0,
        0.68611,
        0,
        0,
        0.80666
      ],
      926: [
        0,
        0.68611,
        0.15092,
        0,
        0.76777
      ],
      928: [
        0,
        0.68611,
        0.17208,
        0,
        0.8961
      ],
      931: [
        0,
        0.68611,
        0.11431,
        0,
        0.82666
      ],
      933: [
        0,
        0.68611,
        0.10778,
        0,
        0.88555
      ],
      934: [
        0,
        0.68611,
        0.05632,
        0,
        0.82666
      ],
      936: [
        0,
        0.68611,
        0.10778,
        0,
        0.88555
      ],
      937: [
        0,
        0.68611,
        0.0992,
        0,
        0.82666
      ],
      8211: [
        0,
        0.44444,
        0.09811,
        0,
        0.59111
      ],
      8212: [
        0,
        0.44444,
        0.09811,
        0,
        1.18221
      ],
      8216: [
        0,
        0.69444,
        0.12945,
        0,
        0.35555
      ],
      8217: [
        0,
        0.69444,
        0.12945,
        0,
        0.35555
      ],
      8220: [
        0,
        0.69444,
        0.16772,
        0,
        0.62055
      ],
      8221: [
        0,
        0.69444,
        0.07939,
        0,
        0.62055
      ]
    },
    "Main-Italic": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69444,
        0.12417,
        0,
        0.30667
      ],
      34: [
        0,
        0.69444,
        0.06961,
        0,
        0.51444
      ],
      35: [
        0.19444,
        0.69444,
        0.06616,
        0,
        0.81777
      ],
      37: [
        0.05556,
        0.75,
        0.13639,
        0,
        0.81777
      ],
      38: [
        0,
        0.69444,
        0.09694,
        0,
        0.76666
      ],
      39: [
        0,
        0.69444,
        0.12417,
        0,
        0.30667
      ],
      40: [
        0.25,
        0.75,
        0.16194,
        0,
        0.40889
      ],
      41: [
        0.25,
        0.75,
        0.03694,
        0,
        0.40889
      ],
      42: [
        0,
        0.75,
        0.14917,
        0,
        0.51111
      ],
      43: [
        0.05667,
        0.56167,
        0.03694,
        0,
        0.76666
      ],
      44: [
        0.19444,
        0.10556,
        0,
        0,
        0.30667
      ],
      45: [
        0,
        0.43056,
        0.02826,
        0,
        0.35778
      ],
      46: [
        0,
        0.10556,
        0,
        0,
        0.30667
      ],
      47: [
        0.25,
        0.75,
        0.16194,
        0,
        0.51111
      ],
      48: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      49: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      50: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      51: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      52: [
        0.19444,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      53: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      54: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      55: [
        0.19444,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      56: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      57: [
        0,
        0.64444,
        0.13556,
        0,
        0.51111
      ],
      58: [
        0,
        0.43056,
        0.0582,
        0,
        0.30667
      ],
      59: [
        0.19444,
        0.43056,
        0.0582,
        0,
        0.30667
      ],
      61: [
        -0.13313,
        0.36687,
        0.06616,
        0,
        0.76666
      ],
      63: [
        0,
        0.69444,
        0.1225,
        0,
        0.51111
      ],
      64: [
        0,
        0.69444,
        0.09597,
        0,
        0.76666
      ],
      65: [
        0,
        0.68333,
        0,
        0,
        0.74333
      ],
      66: [
        0,
        0.68333,
        0.10257,
        0,
        0.70389
      ],
      67: [
        0,
        0.68333,
        0.14528,
        0,
        0.71555
      ],
      68: [
        0,
        0.68333,
        0.09403,
        0,
        0.755
      ],
      69: [
        0,
        0.68333,
        0.12028,
        0,
        0.67833
      ],
      70: [
        0,
        0.68333,
        0.13305,
        0,
        0.65277
      ],
      71: [
        0,
        0.68333,
        0.08722,
        0,
        0.77361
      ],
      72: [
        0,
        0.68333,
        0.16389,
        0,
        0.74333
      ],
      73: [
        0,
        0.68333,
        0.15806,
        0,
        0.38555
      ],
      74: [
        0,
        0.68333,
        0.14028,
        0,
        0.525
      ],
      75: [
        0,
        0.68333,
        0.14528,
        0,
        0.76888
      ],
      76: [
        0,
        0.68333,
        0,
        0,
        0.62722
      ],
      77: [
        0,
        0.68333,
        0.16389,
        0,
        0.89666
      ],
      78: [
        0,
        0.68333,
        0.16389,
        0,
        0.74333
      ],
      79: [
        0,
        0.68333,
        0.09403,
        0,
        0.76666
      ],
      80: [
        0,
        0.68333,
        0.10257,
        0,
        0.67833
      ],
      81: [
        0.19444,
        0.68333,
        0.09403,
        0,
        0.76666
      ],
      82: [
        0,
        0.68333,
        0.03868,
        0,
        0.72944
      ],
      83: [
        0,
        0.68333,
        0.11972,
        0,
        0.56222
      ],
      84: [
        0,
        0.68333,
        0.13305,
        0,
        0.71555
      ],
      85: [
        0,
        0.68333,
        0.16389,
        0,
        0.74333
      ],
      86: [
        0,
        0.68333,
        0.18361,
        0,
        0.74333
      ],
      87: [
        0,
        0.68333,
        0.18361,
        0,
        0.99888
      ],
      88: [
        0,
        0.68333,
        0.15806,
        0,
        0.74333
      ],
      89: [
        0,
        0.68333,
        0.19383,
        0,
        0.74333
      ],
      90: [
        0,
        0.68333,
        0.14528,
        0,
        0.61333
      ],
      91: [
        0.25,
        0.75,
        0.1875,
        0,
        0.30667
      ],
      93: [
        0.25,
        0.75,
        0.10528,
        0,
        0.30667
      ],
      94: [
        0,
        0.69444,
        0.06646,
        0,
        0.51111
      ],
      95: [
        0.31,
        0.12056,
        0.09208,
        0,
        0.51111
      ],
      97: [
        0,
        0.43056,
        0.07671,
        0,
        0.51111
      ],
      98: [
        0,
        0.69444,
        0.06312,
        0,
        0.46
      ],
      99: [
        0,
        0.43056,
        0.05653,
        0,
        0.46
      ],
      100: [
        0,
        0.69444,
        0.10333,
        0,
        0.51111
      ],
      101: [
        0,
        0.43056,
        0.07514,
        0,
        0.46
      ],
      102: [
        0.19444,
        0.69444,
        0.21194,
        0,
        0.30667
      ],
      103: [
        0.19444,
        0.43056,
        0.08847,
        0,
        0.46
      ],
      104: [
        0,
        0.69444,
        0.07671,
        0,
        0.51111
      ],
      105: [
        0,
        0.65536,
        0.1019,
        0,
        0.30667
      ],
      106: [
        0.19444,
        0.65536,
        0.14467,
        0,
        0.30667
      ],
      107: [
        0,
        0.69444,
        0.10764,
        0,
        0.46
      ],
      108: [
        0,
        0.69444,
        0.10333,
        0,
        0.25555
      ],
      109: [
        0,
        0.43056,
        0.07671,
        0,
        0.81777
      ],
      110: [
        0,
        0.43056,
        0.07671,
        0,
        0.56222
      ],
      111: [
        0,
        0.43056,
        0.06312,
        0,
        0.51111
      ],
      112: [
        0.19444,
        0.43056,
        0.06312,
        0,
        0.51111
      ],
      113: [
        0.19444,
        0.43056,
        0.08847,
        0,
        0.46
      ],
      114: [
        0,
        0.43056,
        0.10764,
        0,
        0.42166
      ],
      115: [
        0,
        0.43056,
        0.08208,
        0,
        0.40889
      ],
      116: [
        0,
        0.61508,
        0.09486,
        0,
        0.33222
      ],
      117: [
        0,
        0.43056,
        0.07671,
        0,
        0.53666
      ],
      118: [
        0,
        0.43056,
        0.10764,
        0,
        0.46
      ],
      119: [
        0,
        0.43056,
        0.10764,
        0,
        0.66444
      ],
      120: [
        0,
        0.43056,
        0.12042,
        0,
        0.46389
      ],
      121: [
        0.19444,
        0.43056,
        0.08847,
        0,
        0.48555
      ],
      122: [
        0,
        0.43056,
        0.12292,
        0,
        0.40889
      ],
      126: [
        0.35,
        0.31786,
        0.11585,
        0,
        0.51111
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      168: [
        0,
        0.66786,
        0.10474,
        0,
        0.51111
      ],
      176: [
        0,
        0.69444,
        0,
        0,
        0.83129
      ],
      184: [
        0.17014,
        0,
        0,
        0,
        0.46
      ],
      198: [
        0,
        0.68333,
        0.12028,
        0,
        0.88277
      ],
      216: [
        0.04861,
        0.73194,
        0.09403,
        0,
        0.76666
      ],
      223: [
        0.19444,
        0.69444,
        0.10514,
        0,
        0.53666
      ],
      230: [
        0,
        0.43056,
        0.07514,
        0,
        0.71555
      ],
      248: [
        0.09722,
        0.52778,
        0.09194,
        0,
        0.51111
      ],
      338: [
        0,
        0.68333,
        0.12028,
        0,
        0.98499
      ],
      339: [
        0,
        0.43056,
        0.07514,
        0,
        0.71555
      ],
      710: [
        0,
        0.69444,
        0.06646,
        0,
        0.51111
      ],
      711: [
        0,
        0.62847,
        0.08295,
        0,
        0.51111
      ],
      713: [
        0,
        0.56167,
        0.10333,
        0,
        0.51111
      ],
      714: [
        0,
        0.69444,
        0.09694,
        0,
        0.51111
      ],
      715: [
        0,
        0.69444,
        0,
        0,
        0.51111
      ],
      728: [
        0,
        0.69444,
        0.10806,
        0,
        0.51111
      ],
      729: [
        0,
        0.66786,
        0.11752,
        0,
        0.30667
      ],
      730: [
        0,
        0.69444,
        0,
        0,
        0.83129
      ],
      732: [
        0,
        0.66786,
        0.11585,
        0,
        0.51111
      ],
      733: [
        0,
        0.69444,
        0.1225,
        0,
        0.51111
      ],
      915: [
        0,
        0.68333,
        0.13305,
        0,
        0.62722
      ],
      916: [
        0,
        0.68333,
        0,
        0,
        0.81777
      ],
      920: [
        0,
        0.68333,
        0.09403,
        0,
        0.76666
      ],
      923: [
        0,
        0.68333,
        0,
        0,
        0.69222
      ],
      926: [
        0,
        0.68333,
        0.15294,
        0,
        0.66444
      ],
      928: [
        0,
        0.68333,
        0.16389,
        0,
        0.74333
      ],
      931: [
        0,
        0.68333,
        0.12028,
        0,
        0.71555
      ],
      933: [
        0,
        0.68333,
        0.11111,
        0,
        0.76666
      ],
      934: [
        0,
        0.68333,
        0.05986,
        0,
        0.71555
      ],
      936: [
        0,
        0.68333,
        0.11111,
        0,
        0.76666
      ],
      937: [
        0,
        0.68333,
        0.10257,
        0,
        0.71555
      ],
      8211: [
        0,
        0.43056,
        0.09208,
        0,
        0.51111
      ],
      8212: [
        0,
        0.43056,
        0.09208,
        0,
        1.02222
      ],
      8216: [
        0,
        0.69444,
        0.12417,
        0,
        0.30667
      ],
      8217: [
        0,
        0.69444,
        0.12417,
        0,
        0.30667
      ],
      8220: [
        0,
        0.69444,
        0.1685,
        0,
        0.51444
      ],
      8221: [
        0,
        0.69444,
        0.06961,
        0,
        0.51444
      ],
      8463: [
        0,
        0.68889,
        0,
        0,
        0.54028
      ]
    },
    "Main-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      34: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      35: [
        0.19444,
        0.69444,
        0,
        0,
        0.83334
      ],
      36: [
        0.05556,
        0.75,
        0,
        0,
        0.5
      ],
      37: [
        0.05556,
        0.75,
        0,
        0,
        0.83334
      ],
      38: [
        0,
        0.69444,
        0,
        0,
        0.77778
      ],
      39: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      40: [
        0.25,
        0.75,
        0,
        0,
        0.38889
      ],
      41: [
        0.25,
        0.75,
        0,
        0,
        0.38889
      ],
      42: [
        0,
        0.75,
        0,
        0,
        0.5
      ],
      43: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      44: [
        0.19444,
        0.10556,
        0,
        0,
        0.27778
      ],
      45: [
        0,
        0.43056,
        0,
        0,
        0.33333
      ],
      46: [
        0,
        0.10556,
        0,
        0,
        0.27778
      ],
      47: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      48: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      49: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      50: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      51: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      52: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      53: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      54: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      55: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      56: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      57: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      58: [
        0,
        0.43056,
        0,
        0,
        0.27778
      ],
      59: [
        0.19444,
        0.43056,
        0,
        0,
        0.27778
      ],
      60: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      61: [
        -0.13313,
        0.36687,
        0,
        0,
        0.77778
      ],
      62: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      63: [
        0,
        0.69444,
        0,
        0,
        0.47222
      ],
      64: [
        0,
        0.69444,
        0,
        0,
        0.77778
      ],
      65: [
        0,
        0.68333,
        0,
        0,
        0.75
      ],
      66: [
        0,
        0.68333,
        0,
        0,
        0.70834
      ],
      67: [
        0,
        0.68333,
        0,
        0,
        0.72222
      ],
      68: [
        0,
        0.68333,
        0,
        0,
        0.76389
      ],
      69: [
        0,
        0.68333,
        0,
        0,
        0.68056
      ],
      70: [
        0,
        0.68333,
        0,
        0,
        0.65278
      ],
      71: [
        0,
        0.68333,
        0,
        0,
        0.78472
      ],
      72: [
        0,
        0.68333,
        0,
        0,
        0.75
      ],
      73: [
        0,
        0.68333,
        0,
        0,
        0.36111
      ],
      74: [
        0,
        0.68333,
        0,
        0,
        0.51389
      ],
      75: [
        0,
        0.68333,
        0,
        0,
        0.77778
      ],
      76: [
        0,
        0.68333,
        0,
        0,
        0.625
      ],
      77: [
        0,
        0.68333,
        0,
        0,
        0.91667
      ],
      78: [
        0,
        0.68333,
        0,
        0,
        0.75
      ],
      79: [
        0,
        0.68333,
        0,
        0,
        0.77778
      ],
      80: [
        0,
        0.68333,
        0,
        0,
        0.68056
      ],
      81: [
        0.19444,
        0.68333,
        0,
        0,
        0.77778
      ],
      82: [
        0,
        0.68333,
        0,
        0,
        0.73611
      ],
      83: [
        0,
        0.68333,
        0,
        0,
        0.55556
      ],
      84: [
        0,
        0.68333,
        0,
        0,
        0.72222
      ],
      85: [
        0,
        0.68333,
        0,
        0,
        0.75
      ],
      86: [
        0,
        0.68333,
        0.01389,
        0,
        0.75
      ],
      87: [
        0,
        0.68333,
        0.01389,
        0,
        1.02778
      ],
      88: [
        0,
        0.68333,
        0,
        0,
        0.75
      ],
      89: [
        0,
        0.68333,
        0.025,
        0,
        0.75
      ],
      90: [
        0,
        0.68333,
        0,
        0,
        0.61111
      ],
      91: [
        0.25,
        0.75,
        0,
        0,
        0.27778
      ],
      92: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      93: [
        0.25,
        0.75,
        0,
        0,
        0.27778
      ],
      94: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      95: [
        0.31,
        0.12056,
        0.02778,
        0,
        0.5
      ],
      97: [
        0,
        0.43056,
        0,
        0,
        0.5
      ],
      98: [
        0,
        0.69444,
        0,
        0,
        0.55556
      ],
      99: [
        0,
        0.43056,
        0,
        0,
        0.44445
      ],
      100: [
        0,
        0.69444,
        0,
        0,
        0.55556
      ],
      101: [
        0,
        0.43056,
        0,
        0,
        0.44445
      ],
      102: [
        0,
        0.69444,
        0.07778,
        0,
        0.30556
      ],
      103: [
        0.19444,
        0.43056,
        0.01389,
        0,
        0.5
      ],
      104: [
        0,
        0.69444,
        0,
        0,
        0.55556
      ],
      105: [
        0,
        0.66786,
        0,
        0,
        0.27778
      ],
      106: [
        0.19444,
        0.66786,
        0,
        0,
        0.30556
      ],
      107: [
        0,
        0.69444,
        0,
        0,
        0.52778
      ],
      108: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      109: [
        0,
        0.43056,
        0,
        0,
        0.83334
      ],
      110: [
        0,
        0.43056,
        0,
        0,
        0.55556
      ],
      111: [
        0,
        0.43056,
        0,
        0,
        0.5
      ],
      112: [
        0.19444,
        0.43056,
        0,
        0,
        0.55556
      ],
      113: [
        0.19444,
        0.43056,
        0,
        0,
        0.52778
      ],
      114: [
        0,
        0.43056,
        0,
        0,
        0.39167
      ],
      115: [
        0,
        0.43056,
        0,
        0,
        0.39445
      ],
      116: [
        0,
        0.61508,
        0,
        0,
        0.38889
      ],
      117: [
        0,
        0.43056,
        0,
        0,
        0.55556
      ],
      118: [
        0,
        0.43056,
        0.01389,
        0,
        0.52778
      ],
      119: [
        0,
        0.43056,
        0.01389,
        0,
        0.72222
      ],
      120: [
        0,
        0.43056,
        0,
        0,
        0.52778
      ],
      121: [
        0.19444,
        0.43056,
        0.01389,
        0,
        0.52778
      ],
      122: [
        0,
        0.43056,
        0,
        0,
        0.44445
      ],
      123: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      124: [
        0.25,
        0.75,
        0,
        0,
        0.27778
      ],
      125: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      126: [
        0.35,
        0.31786,
        0,
        0,
        0.5
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      163: [
        0,
        0.69444,
        0,
        0,
        0.76909
      ],
      167: [
        0.19444,
        0.69444,
        0,
        0,
        0.44445
      ],
      168: [
        0,
        0.66786,
        0,
        0,
        0.5
      ],
      172: [
        0,
        0.43056,
        0,
        0,
        0.66667
      ],
      176: [
        0,
        0.69444,
        0,
        0,
        0.75
      ],
      177: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      182: [
        0.19444,
        0.69444,
        0,
        0,
        0.61111
      ],
      184: [
        0.17014,
        0,
        0,
        0,
        0.44445
      ],
      198: [
        0,
        0.68333,
        0,
        0,
        0.90278
      ],
      215: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      216: [
        0.04861,
        0.73194,
        0,
        0,
        0.77778
      ],
      223: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      230: [
        0,
        0.43056,
        0,
        0,
        0.72222
      ],
      247: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      248: [
        0.09722,
        0.52778,
        0,
        0,
        0.5
      ],
      305: [
        0,
        0.43056,
        0,
        0,
        0.27778
      ],
      338: [
        0,
        0.68333,
        0,
        0,
        1.01389
      ],
      339: [
        0,
        0.43056,
        0,
        0,
        0.77778
      ],
      567: [
        0.19444,
        0.43056,
        0,
        0,
        0.30556
      ],
      710: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      711: [
        0,
        0.62847,
        0,
        0,
        0.5
      ],
      713: [
        0,
        0.56778,
        0,
        0,
        0.5
      ],
      714: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      715: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      728: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      729: [
        0,
        0.66786,
        0,
        0,
        0.27778
      ],
      730: [
        0,
        0.69444,
        0,
        0,
        0.75
      ],
      732: [
        0,
        0.66786,
        0,
        0,
        0.5
      ],
      733: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      915: [
        0,
        0.68333,
        0,
        0,
        0.625
      ],
      916: [
        0,
        0.68333,
        0,
        0,
        0.83334
      ],
      920: [
        0,
        0.68333,
        0,
        0,
        0.77778
      ],
      923: [
        0,
        0.68333,
        0,
        0,
        0.69445
      ],
      926: [
        0,
        0.68333,
        0,
        0,
        0.66667
      ],
      928: [
        0,
        0.68333,
        0,
        0,
        0.75
      ],
      931: [
        0,
        0.68333,
        0,
        0,
        0.72222
      ],
      933: [
        0,
        0.68333,
        0,
        0,
        0.77778
      ],
      934: [
        0,
        0.68333,
        0,
        0,
        0.72222
      ],
      936: [
        0,
        0.68333,
        0,
        0,
        0.77778
      ],
      937: [
        0,
        0.68333,
        0,
        0,
        0.72222
      ],
      8211: [
        0,
        0.43056,
        0.02778,
        0,
        0.5
      ],
      8212: [
        0,
        0.43056,
        0.02778,
        0,
        1
      ],
      8216: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      8217: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      8220: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      8221: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      8224: [
        0.19444,
        0.69444,
        0,
        0,
        0.44445
      ],
      8225: [
        0.19444,
        0.69444,
        0,
        0,
        0.44445
      ],
      8230: [
        0,
        0.123,
        0,
        0,
        1.172
      ],
      8242: [
        0,
        0.55556,
        0,
        0,
        0.275
      ],
      8407: [
        0,
        0.71444,
        0.15382,
        0,
        0.5
      ],
      8463: [
        0,
        0.68889,
        0,
        0,
        0.54028
      ],
      8465: [
        0,
        0.69444,
        0,
        0,
        0.72222
      ],
      8467: [
        0,
        0.69444,
        0,
        0.11111,
        0.41667
      ],
      8472: [
        0.19444,
        0.43056,
        0,
        0.11111,
        0.63646
      ],
      8476: [
        0,
        0.69444,
        0,
        0,
        0.72222
      ],
      8501: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      8592: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8593: [
        0.19444,
        0.69444,
        0,
        0,
        0.5
      ],
      8594: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8595: [
        0.19444,
        0.69444,
        0,
        0,
        0.5
      ],
      8596: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8597: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      8598: [
        0.19444,
        0.69444,
        0,
        0,
        1
      ],
      8599: [
        0.19444,
        0.69444,
        0,
        0,
        1
      ],
      8600: [
        0.19444,
        0.69444,
        0,
        0,
        1
      ],
      8601: [
        0.19444,
        0.69444,
        0,
        0,
        1
      ],
      8614: [
        0.011,
        0.511,
        0,
        0,
        1
      ],
      8617: [
        0.011,
        0.511,
        0,
        0,
        1.126
      ],
      8618: [
        0.011,
        0.511,
        0,
        0,
        1.126
      ],
      8636: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8637: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8640: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8641: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8652: [
        0.011,
        0.671,
        0,
        0,
        1
      ],
      8656: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8657: [
        0.19444,
        0.69444,
        0,
        0,
        0.61111
      ],
      8658: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8659: [
        0.19444,
        0.69444,
        0,
        0,
        0.61111
      ],
      8660: [
        -0.13313,
        0.36687,
        0,
        0,
        1
      ],
      8661: [
        0.25,
        0.75,
        0,
        0,
        0.61111
      ],
      8704: [
        0,
        0.69444,
        0,
        0,
        0.55556
      ],
      8706: [
        0,
        0.69444,
        0.05556,
        0.08334,
        0.5309
      ],
      8707: [
        0,
        0.69444,
        0,
        0,
        0.55556
      ],
      8709: [
        0.05556,
        0.75,
        0,
        0,
        0.5
      ],
      8711: [
        0,
        0.68333,
        0,
        0,
        0.83334
      ],
      8712: [
        0.0391,
        0.5391,
        0,
        0,
        0.66667
      ],
      8715: [
        0.0391,
        0.5391,
        0,
        0,
        0.66667
      ],
      8722: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      8723: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      8725: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      8726: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      8727: [
        -0.03472,
        0.46528,
        0,
        0,
        0.5
      ],
      8728: [
        -0.05555,
        0.44445,
        0,
        0,
        0.5
      ],
      8729: [
        -0.05555,
        0.44445,
        0,
        0,
        0.5
      ],
      8730: [
        0.2,
        0.8,
        0,
        0,
        0.83334
      ],
      8733: [
        0,
        0.43056,
        0,
        0,
        0.77778
      ],
      8734: [
        0,
        0.43056,
        0,
        0,
        1
      ],
      8736: [
        0,
        0.69224,
        0,
        0,
        0.72222
      ],
      8739: [
        0.25,
        0.75,
        0,
        0,
        0.27778
      ],
      8741: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      8743: [
        0,
        0.55556,
        0,
        0,
        0.66667
      ],
      8744: [
        0,
        0.55556,
        0,
        0,
        0.66667
      ],
      8745: [
        0,
        0.55556,
        0,
        0,
        0.66667
      ],
      8746: [
        0,
        0.55556,
        0,
        0,
        0.66667
      ],
      8747: [
        0.19444,
        0.69444,
        0.11111,
        0,
        0.41667
      ],
      8764: [
        -0.13313,
        0.36687,
        0,
        0,
        0.77778
      ],
      8768: [
        0.19444,
        0.69444,
        0,
        0,
        0.27778
      ],
      8771: [
        -0.03625,
        0.46375,
        0,
        0,
        0.77778
      ],
      8773: [
        -0.022,
        0.589,
        0,
        0,
        0.778
      ],
      8776: [
        -0.01688,
        0.48312,
        0,
        0,
        0.77778
      ],
      8781: [
        -0.03625,
        0.46375,
        0,
        0,
        0.77778
      ],
      8784: [
        -0.133,
        0.673,
        0,
        0,
        0.778
      ],
      8801: [
        -0.03625,
        0.46375,
        0,
        0,
        0.77778
      ],
      8804: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8805: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8810: [
        0.0391,
        0.5391,
        0,
        0,
        1
      ],
      8811: [
        0.0391,
        0.5391,
        0,
        0,
        1
      ],
      8826: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      8827: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      8834: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      8835: [
        0.0391,
        0.5391,
        0,
        0,
        0.77778
      ],
      8838: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8839: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8846: [
        0,
        0.55556,
        0,
        0,
        0.66667
      ],
      8849: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8850: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      8851: [
        0,
        0.55556,
        0,
        0,
        0.66667
      ],
      8852: [
        0,
        0.55556,
        0,
        0,
        0.66667
      ],
      8853: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      8854: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      8855: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      8856: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      8857: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      8866: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      8867: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      8868: [
        0,
        0.69444,
        0,
        0,
        0.77778
      ],
      8869: [
        0,
        0.69444,
        0,
        0,
        0.77778
      ],
      8872: [
        0.249,
        0.75,
        0,
        0,
        0.867
      ],
      8900: [
        -0.05555,
        0.44445,
        0,
        0,
        0.5
      ],
      8901: [
        -0.05555,
        0.44445,
        0,
        0,
        0.27778
      ],
      8902: [
        -0.03472,
        0.46528,
        0,
        0,
        0.5
      ],
      8904: [
        5e-3,
        0.505,
        0,
        0,
        0.9
      ],
      8942: [
        0.03,
        0.903,
        0,
        0,
        0.278
      ],
      8943: [
        -0.19,
        0.313,
        0,
        0,
        1.172
      ],
      8945: [
        -0.1,
        0.823,
        0,
        0,
        1.282
      ],
      8968: [
        0.25,
        0.75,
        0,
        0,
        0.44445
      ],
      8969: [
        0.25,
        0.75,
        0,
        0,
        0.44445
      ],
      8970: [
        0.25,
        0.75,
        0,
        0,
        0.44445
      ],
      8971: [
        0.25,
        0.75,
        0,
        0,
        0.44445
      ],
      8994: [
        -0.14236,
        0.35764,
        0,
        0,
        1
      ],
      8995: [
        -0.14236,
        0.35764,
        0,
        0,
        1
      ],
      9136: [
        0.244,
        0.744,
        0,
        0,
        0.412
      ],
      9137: [
        0.244,
        0.745,
        0,
        0,
        0.412
      ],
      9651: [
        0.19444,
        0.69444,
        0,
        0,
        0.88889
      ],
      9657: [
        -0.03472,
        0.46528,
        0,
        0,
        0.5
      ],
      9661: [
        0.19444,
        0.69444,
        0,
        0,
        0.88889
      ],
      9667: [
        -0.03472,
        0.46528,
        0,
        0,
        0.5
      ],
      9711: [
        0.19444,
        0.69444,
        0,
        0,
        1
      ],
      9824: [
        0.12963,
        0.69444,
        0,
        0,
        0.77778
      ],
      9825: [
        0.12963,
        0.69444,
        0,
        0,
        0.77778
      ],
      9826: [
        0.12963,
        0.69444,
        0,
        0,
        0.77778
      ],
      9827: [
        0.12963,
        0.69444,
        0,
        0,
        0.77778
      ],
      9837: [
        0,
        0.75,
        0,
        0,
        0.38889
      ],
      9838: [
        0.19444,
        0.69444,
        0,
        0,
        0.38889
      ],
      9839: [
        0.19444,
        0.69444,
        0,
        0,
        0.38889
      ],
      10216: [
        0.25,
        0.75,
        0,
        0,
        0.38889
      ],
      10217: [
        0.25,
        0.75,
        0,
        0,
        0.38889
      ],
      10222: [
        0.244,
        0.744,
        0,
        0,
        0.412
      ],
      10223: [
        0.244,
        0.745,
        0,
        0,
        0.412
      ],
      10229: [
        0.011,
        0.511,
        0,
        0,
        1.609
      ],
      10230: [
        0.011,
        0.511,
        0,
        0,
        1.638
      ],
      10231: [
        0.011,
        0.511,
        0,
        0,
        1.859
      ],
      10232: [
        0.024,
        0.525,
        0,
        0,
        1.609
      ],
      10233: [
        0.024,
        0.525,
        0,
        0,
        1.638
      ],
      10234: [
        0.024,
        0.525,
        0,
        0,
        1.858
      ],
      10236: [
        0.011,
        0.511,
        0,
        0,
        1.638
      ],
      10815: [
        0,
        0.68333,
        0,
        0,
        0.75
      ],
      10927: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      10928: [
        0.13597,
        0.63597,
        0,
        0,
        0.77778
      ],
      57376: [
        0.19444,
        0.69444,
        0,
        0,
        0
      ]
    },
    "Math-BoldItalic": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      48: [
        0,
        0.44444,
        0,
        0,
        0.575
      ],
      49: [
        0,
        0.44444,
        0,
        0,
        0.575
      ],
      50: [
        0,
        0.44444,
        0,
        0,
        0.575
      ],
      51: [
        0.19444,
        0.44444,
        0,
        0,
        0.575
      ],
      52: [
        0.19444,
        0.44444,
        0,
        0,
        0.575
      ],
      53: [
        0.19444,
        0.44444,
        0,
        0,
        0.575
      ],
      54: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      55: [
        0.19444,
        0.44444,
        0,
        0,
        0.575
      ],
      56: [
        0,
        0.64444,
        0,
        0,
        0.575
      ],
      57: [
        0.19444,
        0.44444,
        0,
        0,
        0.575
      ],
      65: [
        0,
        0.68611,
        0,
        0,
        0.86944
      ],
      66: [
        0,
        0.68611,
        0.04835,
        0,
        0.8664
      ],
      67: [
        0,
        0.68611,
        0.06979,
        0,
        0.81694
      ],
      68: [
        0,
        0.68611,
        0.03194,
        0,
        0.93812
      ],
      69: [
        0,
        0.68611,
        0.05451,
        0,
        0.81007
      ],
      70: [
        0,
        0.68611,
        0.15972,
        0,
        0.68889
      ],
      71: [
        0,
        0.68611,
        0,
        0,
        0.88673
      ],
      72: [
        0,
        0.68611,
        0.08229,
        0,
        0.98229
      ],
      73: [
        0,
        0.68611,
        0.07778,
        0,
        0.51111
      ],
      74: [
        0,
        0.68611,
        0.10069,
        0,
        0.63125
      ],
      75: [
        0,
        0.68611,
        0.06979,
        0,
        0.97118
      ],
      76: [
        0,
        0.68611,
        0,
        0,
        0.75555
      ],
      77: [
        0,
        0.68611,
        0.11424,
        0,
        1.14201
      ],
      78: [
        0,
        0.68611,
        0.11424,
        0,
        0.95034
      ],
      79: [
        0,
        0.68611,
        0.03194,
        0,
        0.83666
      ],
      80: [
        0,
        0.68611,
        0.15972,
        0,
        0.72309
      ],
      81: [
        0.19444,
        0.68611,
        0,
        0,
        0.86861
      ],
      82: [
        0,
        0.68611,
        421e-5,
        0,
        0.87235
      ],
      83: [
        0,
        0.68611,
        0.05382,
        0,
        0.69271
      ],
      84: [
        0,
        0.68611,
        0.15972,
        0,
        0.63663
      ],
      85: [
        0,
        0.68611,
        0.11424,
        0,
        0.80027
      ],
      86: [
        0,
        0.68611,
        0.25555,
        0,
        0.67778
      ],
      87: [
        0,
        0.68611,
        0.15972,
        0,
        1.09305
      ],
      88: [
        0,
        0.68611,
        0.07778,
        0,
        0.94722
      ],
      89: [
        0,
        0.68611,
        0.25555,
        0,
        0.67458
      ],
      90: [
        0,
        0.68611,
        0.06979,
        0,
        0.77257
      ],
      97: [
        0,
        0.44444,
        0,
        0,
        0.63287
      ],
      98: [
        0,
        0.69444,
        0,
        0,
        0.52083
      ],
      99: [
        0,
        0.44444,
        0,
        0,
        0.51342
      ],
      100: [
        0,
        0.69444,
        0,
        0,
        0.60972
      ],
      101: [
        0,
        0.44444,
        0,
        0,
        0.55361
      ],
      102: [
        0.19444,
        0.69444,
        0.11042,
        0,
        0.56806
      ],
      103: [
        0.19444,
        0.44444,
        0.03704,
        0,
        0.5449
      ],
      104: [
        0,
        0.69444,
        0,
        0,
        0.66759
      ],
      105: [
        0,
        0.69326,
        0,
        0,
        0.4048
      ],
      106: [
        0.19444,
        0.69326,
        0.0622,
        0,
        0.47083
      ],
      107: [
        0,
        0.69444,
        0.01852,
        0,
        0.6037
      ],
      108: [
        0,
        0.69444,
        88e-4,
        0,
        0.34815
      ],
      109: [
        0,
        0.44444,
        0,
        0,
        1.0324
      ],
      110: [
        0,
        0.44444,
        0,
        0,
        0.71296
      ],
      111: [
        0,
        0.44444,
        0,
        0,
        0.58472
      ],
      112: [
        0.19444,
        0.44444,
        0,
        0,
        0.60092
      ],
      113: [
        0.19444,
        0.44444,
        0.03704,
        0,
        0.54213
      ],
      114: [
        0,
        0.44444,
        0.03194,
        0,
        0.5287
      ],
      115: [
        0,
        0.44444,
        0,
        0,
        0.53125
      ],
      116: [
        0,
        0.63492,
        0,
        0,
        0.41528
      ],
      117: [
        0,
        0.44444,
        0,
        0,
        0.68102
      ],
      118: [
        0,
        0.44444,
        0.03704,
        0,
        0.56666
      ],
      119: [
        0,
        0.44444,
        0.02778,
        0,
        0.83148
      ],
      120: [
        0,
        0.44444,
        0,
        0,
        0.65903
      ],
      121: [
        0.19444,
        0.44444,
        0.03704,
        0,
        0.59028
      ],
      122: [
        0,
        0.44444,
        0.04213,
        0,
        0.55509
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      915: [
        0,
        0.68611,
        0.15972,
        0,
        0.65694
      ],
      916: [
        0,
        0.68611,
        0,
        0,
        0.95833
      ],
      920: [
        0,
        0.68611,
        0.03194,
        0,
        0.86722
      ],
      923: [
        0,
        0.68611,
        0,
        0,
        0.80555
      ],
      926: [
        0,
        0.68611,
        0.07458,
        0,
        0.84125
      ],
      928: [
        0,
        0.68611,
        0.08229,
        0,
        0.98229
      ],
      931: [
        0,
        0.68611,
        0.05451,
        0,
        0.88507
      ],
      933: [
        0,
        0.68611,
        0.15972,
        0,
        0.67083
      ],
      934: [
        0,
        0.68611,
        0,
        0,
        0.76666
      ],
      936: [
        0,
        0.68611,
        0.11653,
        0,
        0.71402
      ],
      937: [
        0,
        0.68611,
        0.04835,
        0,
        0.8789
      ],
      945: [
        0,
        0.44444,
        0,
        0,
        0.76064
      ],
      946: [
        0.19444,
        0.69444,
        0.03403,
        0,
        0.65972
      ],
      947: [
        0.19444,
        0.44444,
        0.06389,
        0,
        0.59003
      ],
      948: [
        0,
        0.69444,
        0.03819,
        0,
        0.52222
      ],
      949: [
        0,
        0.44444,
        0,
        0,
        0.52882
      ],
      950: [
        0.19444,
        0.69444,
        0.06215,
        0,
        0.50833
      ],
      951: [
        0.19444,
        0.44444,
        0.03704,
        0,
        0.6
      ],
      952: [
        0,
        0.69444,
        0.03194,
        0,
        0.5618
      ],
      953: [
        0,
        0.44444,
        0,
        0,
        0.41204
      ],
      954: [
        0,
        0.44444,
        0,
        0,
        0.66759
      ],
      955: [
        0,
        0.69444,
        0,
        0,
        0.67083
      ],
      956: [
        0.19444,
        0.44444,
        0,
        0,
        0.70787
      ],
      957: [
        0,
        0.44444,
        0.06898,
        0,
        0.57685
      ],
      958: [
        0.19444,
        0.69444,
        0.03021,
        0,
        0.50833
      ],
      959: [
        0,
        0.44444,
        0,
        0,
        0.58472
      ],
      960: [
        0,
        0.44444,
        0.03704,
        0,
        0.68241
      ],
      961: [
        0.19444,
        0.44444,
        0,
        0,
        0.6118
      ],
      962: [
        0.09722,
        0.44444,
        0.07917,
        0,
        0.42361
      ],
      963: [
        0,
        0.44444,
        0.03704,
        0,
        0.68588
      ],
      964: [
        0,
        0.44444,
        0.13472,
        0,
        0.52083
      ],
      965: [
        0,
        0.44444,
        0.03704,
        0,
        0.63055
      ],
      966: [
        0.19444,
        0.44444,
        0,
        0,
        0.74722
      ],
      967: [
        0.19444,
        0.44444,
        0,
        0,
        0.71805
      ],
      968: [
        0.19444,
        0.69444,
        0.03704,
        0,
        0.75833
      ],
      969: [
        0,
        0.44444,
        0.03704,
        0,
        0.71782
      ],
      977: [
        0,
        0.69444,
        0,
        0,
        0.69155
      ],
      981: [
        0.19444,
        0.69444,
        0,
        0,
        0.7125
      ],
      982: [
        0,
        0.44444,
        0.03194,
        0,
        0.975
      ],
      1009: [
        0.19444,
        0.44444,
        0,
        0,
        0.6118
      ],
      1013: [
        0,
        0.44444,
        0,
        0,
        0.48333
      ],
      57649: [
        0,
        0.44444,
        0,
        0,
        0.39352
      ],
      57911: [
        0.19444,
        0.44444,
        0,
        0,
        0.43889
      ]
    },
    "Math-Italic": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      48: [
        0,
        0.43056,
        0,
        0,
        0.5
      ],
      49: [
        0,
        0.43056,
        0,
        0,
        0.5
      ],
      50: [
        0,
        0.43056,
        0,
        0,
        0.5
      ],
      51: [
        0.19444,
        0.43056,
        0,
        0,
        0.5
      ],
      52: [
        0.19444,
        0.43056,
        0,
        0,
        0.5
      ],
      53: [
        0.19444,
        0.43056,
        0,
        0,
        0.5
      ],
      54: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      55: [
        0.19444,
        0.43056,
        0,
        0,
        0.5
      ],
      56: [
        0,
        0.64444,
        0,
        0,
        0.5
      ],
      57: [
        0.19444,
        0.43056,
        0,
        0,
        0.5
      ],
      65: [
        0,
        0.68333,
        0,
        0.13889,
        0.75
      ],
      66: [
        0,
        0.68333,
        0.05017,
        0.08334,
        0.75851
      ],
      67: [
        0,
        0.68333,
        0.07153,
        0.08334,
        0.71472
      ],
      68: [
        0,
        0.68333,
        0.02778,
        0.05556,
        0.82792
      ],
      69: [
        0,
        0.68333,
        0.05764,
        0.08334,
        0.7382
      ],
      70: [
        0,
        0.68333,
        0.13889,
        0.08334,
        0.64306
      ],
      71: [
        0,
        0.68333,
        0,
        0.08334,
        0.78625
      ],
      72: [
        0,
        0.68333,
        0.08125,
        0.05556,
        0.83125
      ],
      73: [
        0,
        0.68333,
        0.07847,
        0.11111,
        0.43958
      ],
      74: [
        0,
        0.68333,
        0.09618,
        0.16667,
        0.55451
      ],
      75: [
        0,
        0.68333,
        0.07153,
        0.05556,
        0.84931
      ],
      76: [
        0,
        0.68333,
        0,
        0.02778,
        0.68056
      ],
      77: [
        0,
        0.68333,
        0.10903,
        0.08334,
        0.97014
      ],
      78: [
        0,
        0.68333,
        0.10903,
        0.08334,
        0.80347
      ],
      79: [
        0,
        0.68333,
        0.02778,
        0.08334,
        0.76278
      ],
      80: [
        0,
        0.68333,
        0.13889,
        0.08334,
        0.64201
      ],
      81: [
        0.19444,
        0.68333,
        0,
        0.08334,
        0.79056
      ],
      82: [
        0,
        0.68333,
        773e-5,
        0.08334,
        0.75929
      ],
      83: [
        0,
        0.68333,
        0.05764,
        0.08334,
        0.6132
      ],
      84: [
        0,
        0.68333,
        0.13889,
        0.08334,
        0.58438
      ],
      85: [
        0,
        0.68333,
        0.10903,
        0.02778,
        0.68278
      ],
      86: [
        0,
        0.68333,
        0.22222,
        0,
        0.58333
      ],
      87: [
        0,
        0.68333,
        0.13889,
        0,
        0.94445
      ],
      88: [
        0,
        0.68333,
        0.07847,
        0.08334,
        0.82847
      ],
      89: [
        0,
        0.68333,
        0.22222,
        0,
        0.58056
      ],
      90: [
        0,
        0.68333,
        0.07153,
        0.08334,
        0.68264
      ],
      97: [
        0,
        0.43056,
        0,
        0,
        0.52859
      ],
      98: [
        0,
        0.69444,
        0,
        0,
        0.42917
      ],
      99: [
        0,
        0.43056,
        0,
        0.05556,
        0.43276
      ],
      100: [
        0,
        0.69444,
        0,
        0.16667,
        0.52049
      ],
      101: [
        0,
        0.43056,
        0,
        0.05556,
        0.46563
      ],
      102: [
        0.19444,
        0.69444,
        0.10764,
        0.16667,
        0.48959
      ],
      103: [
        0.19444,
        0.43056,
        0.03588,
        0.02778,
        0.47697
      ],
      104: [
        0,
        0.69444,
        0,
        0,
        0.57616
      ],
      105: [
        0,
        0.65952,
        0,
        0,
        0.34451
      ],
      106: [
        0.19444,
        0.65952,
        0.05724,
        0,
        0.41181
      ],
      107: [
        0,
        0.69444,
        0.03148,
        0,
        0.5206
      ],
      108: [
        0,
        0.69444,
        0.01968,
        0.08334,
        0.29838
      ],
      109: [
        0,
        0.43056,
        0,
        0,
        0.87801
      ],
      110: [
        0,
        0.43056,
        0,
        0,
        0.60023
      ],
      111: [
        0,
        0.43056,
        0,
        0.05556,
        0.48472
      ],
      112: [
        0.19444,
        0.43056,
        0,
        0.08334,
        0.50313
      ],
      113: [
        0.19444,
        0.43056,
        0.03588,
        0.08334,
        0.44641
      ],
      114: [
        0,
        0.43056,
        0.02778,
        0.05556,
        0.45116
      ],
      115: [
        0,
        0.43056,
        0,
        0.05556,
        0.46875
      ],
      116: [
        0,
        0.61508,
        0,
        0.08334,
        0.36111
      ],
      117: [
        0,
        0.43056,
        0,
        0.02778,
        0.57246
      ],
      118: [
        0,
        0.43056,
        0.03588,
        0.02778,
        0.48472
      ],
      119: [
        0,
        0.43056,
        0.02691,
        0.08334,
        0.71592
      ],
      120: [
        0,
        0.43056,
        0,
        0.02778,
        0.57153
      ],
      121: [
        0.19444,
        0.43056,
        0.03588,
        0.05556,
        0.49028
      ],
      122: [
        0,
        0.43056,
        0.04398,
        0.05556,
        0.46505
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      915: [
        0,
        0.68333,
        0.13889,
        0.08334,
        0.61528
      ],
      916: [
        0,
        0.68333,
        0,
        0.16667,
        0.83334
      ],
      920: [
        0,
        0.68333,
        0.02778,
        0.08334,
        0.76278
      ],
      923: [
        0,
        0.68333,
        0,
        0.16667,
        0.69445
      ],
      926: [
        0,
        0.68333,
        0.07569,
        0.08334,
        0.74236
      ],
      928: [
        0,
        0.68333,
        0.08125,
        0.05556,
        0.83125
      ],
      931: [
        0,
        0.68333,
        0.05764,
        0.08334,
        0.77986
      ],
      933: [
        0,
        0.68333,
        0.13889,
        0.05556,
        0.58333
      ],
      934: [
        0,
        0.68333,
        0,
        0.08334,
        0.66667
      ],
      936: [
        0,
        0.68333,
        0.11,
        0.05556,
        0.61222
      ],
      937: [
        0,
        0.68333,
        0.05017,
        0.08334,
        0.7724
      ],
      945: [
        0,
        0.43056,
        37e-4,
        0.02778,
        0.6397
      ],
      946: [
        0.19444,
        0.69444,
        0.05278,
        0.08334,
        0.56563
      ],
      947: [
        0.19444,
        0.43056,
        0.05556,
        0,
        0.51773
      ],
      948: [
        0,
        0.69444,
        0.03785,
        0.05556,
        0.44444
      ],
      949: [
        0,
        0.43056,
        0,
        0.08334,
        0.46632
      ],
      950: [
        0.19444,
        0.69444,
        0.07378,
        0.08334,
        0.4375
      ],
      951: [
        0.19444,
        0.43056,
        0.03588,
        0.05556,
        0.49653
      ],
      952: [
        0,
        0.69444,
        0.02778,
        0.08334,
        0.46944
      ],
      953: [
        0,
        0.43056,
        0,
        0.05556,
        0.35394
      ],
      954: [
        0,
        0.43056,
        0,
        0,
        0.57616
      ],
      955: [
        0,
        0.69444,
        0,
        0,
        0.58334
      ],
      956: [
        0.19444,
        0.43056,
        0,
        0.02778,
        0.60255
      ],
      957: [
        0,
        0.43056,
        0.06366,
        0.02778,
        0.49398
      ],
      958: [
        0.19444,
        0.69444,
        0.04601,
        0.11111,
        0.4375
      ],
      959: [
        0,
        0.43056,
        0,
        0.05556,
        0.48472
      ],
      960: [
        0,
        0.43056,
        0.03588,
        0,
        0.57003
      ],
      961: [
        0.19444,
        0.43056,
        0,
        0.08334,
        0.51702
      ],
      962: [
        0.09722,
        0.43056,
        0.07986,
        0.08334,
        0.36285
      ],
      963: [
        0,
        0.43056,
        0.03588,
        0,
        0.57141
      ],
      964: [
        0,
        0.43056,
        0.1132,
        0.02778,
        0.43715
      ],
      965: [
        0,
        0.43056,
        0.03588,
        0.02778,
        0.54028
      ],
      966: [
        0.19444,
        0.43056,
        0,
        0.08334,
        0.65417
      ],
      967: [
        0.19444,
        0.43056,
        0,
        0.05556,
        0.62569
      ],
      968: [
        0.19444,
        0.69444,
        0.03588,
        0.11111,
        0.65139
      ],
      969: [
        0,
        0.43056,
        0.03588,
        0,
        0.62245
      ],
      977: [
        0,
        0.69444,
        0,
        0.08334,
        0.59144
      ],
      981: [
        0.19444,
        0.69444,
        0,
        0.08334,
        0.59583
      ],
      982: [
        0,
        0.43056,
        0.02778,
        0,
        0.82813
      ],
      1009: [
        0.19444,
        0.43056,
        0,
        0.08334,
        0.51702
      ],
      1013: [
        0,
        0.43056,
        0,
        0.05556,
        0.4059
      ],
      57649: [
        0,
        0.43056,
        0,
        0.02778,
        0.32246
      ],
      57911: [
        0.19444,
        0.43056,
        0,
        0.08334,
        0.38403
      ]
    },
    "SansSerif-Bold": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69444,
        0,
        0,
        0.36667
      ],
      34: [
        0,
        0.69444,
        0,
        0,
        0.55834
      ],
      35: [
        0.19444,
        0.69444,
        0,
        0,
        0.91667
      ],
      36: [
        0.05556,
        0.75,
        0,
        0,
        0.55
      ],
      37: [
        0.05556,
        0.75,
        0,
        0,
        1.02912
      ],
      38: [
        0,
        0.69444,
        0,
        0,
        0.83056
      ],
      39: [
        0,
        0.69444,
        0,
        0,
        0.30556
      ],
      40: [
        0.25,
        0.75,
        0,
        0,
        0.42778
      ],
      41: [
        0.25,
        0.75,
        0,
        0,
        0.42778
      ],
      42: [
        0,
        0.75,
        0,
        0,
        0.55
      ],
      43: [
        0.11667,
        0.61667,
        0,
        0,
        0.85556
      ],
      44: [
        0.10556,
        0.13056,
        0,
        0,
        0.30556
      ],
      45: [
        0,
        0.45833,
        0,
        0,
        0.36667
      ],
      46: [
        0,
        0.13056,
        0,
        0,
        0.30556
      ],
      47: [
        0.25,
        0.75,
        0,
        0,
        0.55
      ],
      48: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      49: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      50: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      51: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      52: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      53: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      54: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      55: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      56: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      57: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      58: [
        0,
        0.45833,
        0,
        0,
        0.30556
      ],
      59: [
        0.10556,
        0.45833,
        0,
        0,
        0.30556
      ],
      61: [
        -0.09375,
        0.40625,
        0,
        0,
        0.85556
      ],
      63: [
        0,
        0.69444,
        0,
        0,
        0.51945
      ],
      64: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      65: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      66: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      67: [
        0,
        0.69444,
        0,
        0,
        0.70278
      ],
      68: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      69: [
        0,
        0.69444,
        0,
        0,
        0.64167
      ],
      70: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      71: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      72: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      73: [
        0,
        0.69444,
        0,
        0,
        0.33056
      ],
      74: [
        0,
        0.69444,
        0,
        0,
        0.51945
      ],
      75: [
        0,
        0.69444,
        0,
        0,
        0.76389
      ],
      76: [
        0,
        0.69444,
        0,
        0,
        0.58056
      ],
      77: [
        0,
        0.69444,
        0,
        0,
        0.97778
      ],
      78: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      79: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      80: [
        0,
        0.69444,
        0,
        0,
        0.70278
      ],
      81: [
        0.10556,
        0.69444,
        0,
        0,
        0.79445
      ],
      82: [
        0,
        0.69444,
        0,
        0,
        0.70278
      ],
      83: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      84: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      85: [
        0,
        0.69444,
        0,
        0,
        0.76389
      ],
      86: [
        0,
        0.69444,
        0.01528,
        0,
        0.73334
      ],
      87: [
        0,
        0.69444,
        0.01528,
        0,
        1.03889
      ],
      88: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      89: [
        0,
        0.69444,
        0.0275,
        0,
        0.73334
      ],
      90: [
        0,
        0.69444,
        0,
        0,
        0.67223
      ],
      91: [
        0.25,
        0.75,
        0,
        0,
        0.34306
      ],
      93: [
        0.25,
        0.75,
        0,
        0,
        0.34306
      ],
      94: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      95: [
        0.35,
        0.10833,
        0.03056,
        0,
        0.55
      ],
      97: [
        0,
        0.45833,
        0,
        0,
        0.525
      ],
      98: [
        0,
        0.69444,
        0,
        0,
        0.56111
      ],
      99: [
        0,
        0.45833,
        0,
        0,
        0.48889
      ],
      100: [
        0,
        0.69444,
        0,
        0,
        0.56111
      ],
      101: [
        0,
        0.45833,
        0,
        0,
        0.51111
      ],
      102: [
        0,
        0.69444,
        0.07639,
        0,
        0.33611
      ],
      103: [
        0.19444,
        0.45833,
        0.01528,
        0,
        0.55
      ],
      104: [
        0,
        0.69444,
        0,
        0,
        0.56111
      ],
      105: [
        0,
        0.69444,
        0,
        0,
        0.25556
      ],
      106: [
        0.19444,
        0.69444,
        0,
        0,
        0.28611
      ],
      107: [
        0,
        0.69444,
        0,
        0,
        0.53056
      ],
      108: [
        0,
        0.69444,
        0,
        0,
        0.25556
      ],
      109: [
        0,
        0.45833,
        0,
        0,
        0.86667
      ],
      110: [
        0,
        0.45833,
        0,
        0,
        0.56111
      ],
      111: [
        0,
        0.45833,
        0,
        0,
        0.55
      ],
      112: [
        0.19444,
        0.45833,
        0,
        0,
        0.56111
      ],
      113: [
        0.19444,
        0.45833,
        0,
        0,
        0.56111
      ],
      114: [
        0,
        0.45833,
        0.01528,
        0,
        0.37222
      ],
      115: [
        0,
        0.45833,
        0,
        0,
        0.42167
      ],
      116: [
        0,
        0.58929,
        0,
        0,
        0.40417
      ],
      117: [
        0,
        0.45833,
        0,
        0,
        0.56111
      ],
      118: [
        0,
        0.45833,
        0.01528,
        0,
        0.5
      ],
      119: [
        0,
        0.45833,
        0.01528,
        0,
        0.74445
      ],
      120: [
        0,
        0.45833,
        0,
        0,
        0.5
      ],
      121: [
        0.19444,
        0.45833,
        0.01528,
        0,
        0.5
      ],
      122: [
        0,
        0.45833,
        0,
        0,
        0.47639
      ],
      126: [
        0.35,
        0.34444,
        0,
        0,
        0.55
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      168: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      176: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      180: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      184: [
        0.17014,
        0,
        0,
        0,
        0.48889
      ],
      305: [
        0,
        0.45833,
        0,
        0,
        0.25556
      ],
      567: [
        0.19444,
        0.45833,
        0,
        0,
        0.28611
      ],
      710: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      711: [
        0,
        0.63542,
        0,
        0,
        0.55
      ],
      713: [
        0,
        0.63778,
        0,
        0,
        0.55
      ],
      728: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      729: [
        0,
        0.69444,
        0,
        0,
        0.30556
      ],
      730: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      732: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      733: [
        0,
        0.69444,
        0,
        0,
        0.55
      ],
      915: [
        0,
        0.69444,
        0,
        0,
        0.58056
      ],
      916: [
        0,
        0.69444,
        0,
        0,
        0.91667
      ],
      920: [
        0,
        0.69444,
        0,
        0,
        0.85556
      ],
      923: [
        0,
        0.69444,
        0,
        0,
        0.67223
      ],
      926: [
        0,
        0.69444,
        0,
        0,
        0.73334
      ],
      928: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      931: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      933: [
        0,
        0.69444,
        0,
        0,
        0.85556
      ],
      934: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      936: [
        0,
        0.69444,
        0,
        0,
        0.85556
      ],
      937: [
        0,
        0.69444,
        0,
        0,
        0.79445
      ],
      8211: [
        0,
        0.45833,
        0.03056,
        0,
        0.55
      ],
      8212: [
        0,
        0.45833,
        0.03056,
        0,
        1.10001
      ],
      8216: [
        0,
        0.69444,
        0,
        0,
        0.30556
      ],
      8217: [
        0,
        0.69444,
        0,
        0,
        0.30556
      ],
      8220: [
        0,
        0.69444,
        0,
        0,
        0.55834
      ],
      8221: [
        0,
        0.69444,
        0,
        0,
        0.55834
      ]
    },
    "SansSerif-Italic": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69444,
        0.05733,
        0,
        0.31945
      ],
      34: [
        0,
        0.69444,
        316e-5,
        0,
        0.5
      ],
      35: [
        0.19444,
        0.69444,
        0.05087,
        0,
        0.83334
      ],
      36: [
        0.05556,
        0.75,
        0.11156,
        0,
        0.5
      ],
      37: [
        0.05556,
        0.75,
        0.03126,
        0,
        0.83334
      ],
      38: [
        0,
        0.69444,
        0.03058,
        0,
        0.75834
      ],
      39: [
        0,
        0.69444,
        0.07816,
        0,
        0.27778
      ],
      40: [
        0.25,
        0.75,
        0.13164,
        0,
        0.38889
      ],
      41: [
        0.25,
        0.75,
        0.02536,
        0,
        0.38889
      ],
      42: [
        0,
        0.75,
        0.11775,
        0,
        0.5
      ],
      43: [
        0.08333,
        0.58333,
        0.02536,
        0,
        0.77778
      ],
      44: [
        0.125,
        0.08333,
        0,
        0,
        0.27778
      ],
      45: [
        0,
        0.44444,
        0.01946,
        0,
        0.33333
      ],
      46: [
        0,
        0.08333,
        0,
        0,
        0.27778
      ],
      47: [
        0.25,
        0.75,
        0.13164,
        0,
        0.5
      ],
      48: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      49: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      50: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      51: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      52: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      53: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      54: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      55: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      56: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      57: [
        0,
        0.65556,
        0.11156,
        0,
        0.5
      ],
      58: [
        0,
        0.44444,
        0.02502,
        0,
        0.27778
      ],
      59: [
        0.125,
        0.44444,
        0.02502,
        0,
        0.27778
      ],
      61: [
        -0.13,
        0.37,
        0.05087,
        0,
        0.77778
      ],
      63: [
        0,
        0.69444,
        0.11809,
        0,
        0.47222
      ],
      64: [
        0,
        0.69444,
        0.07555,
        0,
        0.66667
      ],
      65: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      66: [
        0,
        0.69444,
        0.08293,
        0,
        0.66667
      ],
      67: [
        0,
        0.69444,
        0.11983,
        0,
        0.63889
      ],
      68: [
        0,
        0.69444,
        0.07555,
        0,
        0.72223
      ],
      69: [
        0,
        0.69444,
        0.11983,
        0,
        0.59722
      ],
      70: [
        0,
        0.69444,
        0.13372,
        0,
        0.56945
      ],
      71: [
        0,
        0.69444,
        0.11983,
        0,
        0.66667
      ],
      72: [
        0,
        0.69444,
        0.08094,
        0,
        0.70834
      ],
      73: [
        0,
        0.69444,
        0.13372,
        0,
        0.27778
      ],
      74: [
        0,
        0.69444,
        0.08094,
        0,
        0.47222
      ],
      75: [
        0,
        0.69444,
        0.11983,
        0,
        0.69445
      ],
      76: [
        0,
        0.69444,
        0,
        0,
        0.54167
      ],
      77: [
        0,
        0.69444,
        0.08094,
        0,
        0.875
      ],
      78: [
        0,
        0.69444,
        0.08094,
        0,
        0.70834
      ],
      79: [
        0,
        0.69444,
        0.07555,
        0,
        0.73611
      ],
      80: [
        0,
        0.69444,
        0.08293,
        0,
        0.63889
      ],
      81: [
        0.125,
        0.69444,
        0.07555,
        0,
        0.73611
      ],
      82: [
        0,
        0.69444,
        0.08293,
        0,
        0.64584
      ],
      83: [
        0,
        0.69444,
        0.09205,
        0,
        0.55556
      ],
      84: [
        0,
        0.69444,
        0.13372,
        0,
        0.68056
      ],
      85: [
        0,
        0.69444,
        0.08094,
        0,
        0.6875
      ],
      86: [
        0,
        0.69444,
        0.1615,
        0,
        0.66667
      ],
      87: [
        0,
        0.69444,
        0.1615,
        0,
        0.94445
      ],
      88: [
        0,
        0.69444,
        0.13372,
        0,
        0.66667
      ],
      89: [
        0,
        0.69444,
        0.17261,
        0,
        0.66667
      ],
      90: [
        0,
        0.69444,
        0.11983,
        0,
        0.61111
      ],
      91: [
        0.25,
        0.75,
        0.15942,
        0,
        0.28889
      ],
      93: [
        0.25,
        0.75,
        0.08719,
        0,
        0.28889
      ],
      94: [
        0,
        0.69444,
        0.0799,
        0,
        0.5
      ],
      95: [
        0.35,
        0.09444,
        0.08616,
        0,
        0.5
      ],
      97: [
        0,
        0.44444,
        981e-5,
        0,
        0.48056
      ],
      98: [
        0,
        0.69444,
        0.03057,
        0,
        0.51667
      ],
      99: [
        0,
        0.44444,
        0.08336,
        0,
        0.44445
      ],
      100: [
        0,
        0.69444,
        0.09483,
        0,
        0.51667
      ],
      101: [
        0,
        0.44444,
        0.06778,
        0,
        0.44445
      ],
      102: [
        0,
        0.69444,
        0.21705,
        0,
        0.30556
      ],
      103: [
        0.19444,
        0.44444,
        0.10836,
        0,
        0.5
      ],
      104: [
        0,
        0.69444,
        0.01778,
        0,
        0.51667
      ],
      105: [
        0,
        0.67937,
        0.09718,
        0,
        0.23889
      ],
      106: [
        0.19444,
        0.67937,
        0.09162,
        0,
        0.26667
      ],
      107: [
        0,
        0.69444,
        0.08336,
        0,
        0.48889
      ],
      108: [
        0,
        0.69444,
        0.09483,
        0,
        0.23889
      ],
      109: [
        0,
        0.44444,
        0.01778,
        0,
        0.79445
      ],
      110: [
        0,
        0.44444,
        0.01778,
        0,
        0.51667
      ],
      111: [
        0,
        0.44444,
        0.06613,
        0,
        0.5
      ],
      112: [
        0.19444,
        0.44444,
        0.0389,
        0,
        0.51667
      ],
      113: [
        0.19444,
        0.44444,
        0.04169,
        0,
        0.51667
      ],
      114: [
        0,
        0.44444,
        0.10836,
        0,
        0.34167
      ],
      115: [
        0,
        0.44444,
        0.0778,
        0,
        0.38333
      ],
      116: [
        0,
        0.57143,
        0.07225,
        0,
        0.36111
      ],
      117: [
        0,
        0.44444,
        0.04169,
        0,
        0.51667
      ],
      118: [
        0,
        0.44444,
        0.10836,
        0,
        0.46111
      ],
      119: [
        0,
        0.44444,
        0.10836,
        0,
        0.68334
      ],
      120: [
        0,
        0.44444,
        0.09169,
        0,
        0.46111
      ],
      121: [
        0.19444,
        0.44444,
        0.10836,
        0,
        0.46111
      ],
      122: [
        0,
        0.44444,
        0.08752,
        0,
        0.43472
      ],
      126: [
        0.35,
        0.32659,
        0.08826,
        0,
        0.5
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      168: [
        0,
        0.67937,
        0.06385,
        0,
        0.5
      ],
      176: [
        0,
        0.69444,
        0,
        0,
        0.73752
      ],
      184: [
        0.17014,
        0,
        0,
        0,
        0.44445
      ],
      305: [
        0,
        0.44444,
        0.04169,
        0,
        0.23889
      ],
      567: [
        0.19444,
        0.44444,
        0.04169,
        0,
        0.26667
      ],
      710: [
        0,
        0.69444,
        0.0799,
        0,
        0.5
      ],
      711: [
        0,
        0.63194,
        0.08432,
        0,
        0.5
      ],
      713: [
        0,
        0.60889,
        0.08776,
        0,
        0.5
      ],
      714: [
        0,
        0.69444,
        0.09205,
        0,
        0.5
      ],
      715: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      728: [
        0,
        0.69444,
        0.09483,
        0,
        0.5
      ],
      729: [
        0,
        0.67937,
        0.07774,
        0,
        0.27778
      ],
      730: [
        0,
        0.69444,
        0,
        0,
        0.73752
      ],
      732: [
        0,
        0.67659,
        0.08826,
        0,
        0.5
      ],
      733: [
        0,
        0.69444,
        0.09205,
        0,
        0.5
      ],
      915: [
        0,
        0.69444,
        0.13372,
        0,
        0.54167
      ],
      916: [
        0,
        0.69444,
        0,
        0,
        0.83334
      ],
      920: [
        0,
        0.69444,
        0.07555,
        0,
        0.77778
      ],
      923: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      926: [
        0,
        0.69444,
        0.12816,
        0,
        0.66667
      ],
      928: [
        0,
        0.69444,
        0.08094,
        0,
        0.70834
      ],
      931: [
        0,
        0.69444,
        0.11983,
        0,
        0.72222
      ],
      933: [
        0,
        0.69444,
        0.09031,
        0,
        0.77778
      ],
      934: [
        0,
        0.69444,
        0.04603,
        0,
        0.72222
      ],
      936: [
        0,
        0.69444,
        0.09031,
        0,
        0.77778
      ],
      937: [
        0,
        0.69444,
        0.08293,
        0,
        0.72222
      ],
      8211: [
        0,
        0.44444,
        0.08616,
        0,
        0.5
      ],
      8212: [
        0,
        0.44444,
        0.08616,
        0,
        1
      ],
      8216: [
        0,
        0.69444,
        0.07816,
        0,
        0.27778
      ],
      8217: [
        0,
        0.69444,
        0.07816,
        0,
        0.27778
      ],
      8220: [
        0,
        0.69444,
        0.14205,
        0,
        0.5
      ],
      8221: [
        0,
        0.69444,
        316e-5,
        0,
        0.5
      ]
    },
    "SansSerif-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      33: [
        0,
        0.69444,
        0,
        0,
        0.31945
      ],
      34: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      35: [
        0.19444,
        0.69444,
        0,
        0,
        0.83334
      ],
      36: [
        0.05556,
        0.75,
        0,
        0,
        0.5
      ],
      37: [
        0.05556,
        0.75,
        0,
        0,
        0.83334
      ],
      38: [
        0,
        0.69444,
        0,
        0,
        0.75834
      ],
      39: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      40: [
        0.25,
        0.75,
        0,
        0,
        0.38889
      ],
      41: [
        0.25,
        0.75,
        0,
        0,
        0.38889
      ],
      42: [
        0,
        0.75,
        0,
        0,
        0.5
      ],
      43: [
        0.08333,
        0.58333,
        0,
        0,
        0.77778
      ],
      44: [
        0.125,
        0.08333,
        0,
        0,
        0.27778
      ],
      45: [
        0,
        0.44444,
        0,
        0,
        0.33333
      ],
      46: [
        0,
        0.08333,
        0,
        0,
        0.27778
      ],
      47: [
        0.25,
        0.75,
        0,
        0,
        0.5
      ],
      48: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      49: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      50: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      51: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      52: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      53: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      54: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      55: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      56: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      57: [
        0,
        0.65556,
        0,
        0,
        0.5
      ],
      58: [
        0,
        0.44444,
        0,
        0,
        0.27778
      ],
      59: [
        0.125,
        0.44444,
        0,
        0,
        0.27778
      ],
      61: [
        -0.13,
        0.37,
        0,
        0,
        0.77778
      ],
      63: [
        0,
        0.69444,
        0,
        0,
        0.47222
      ],
      64: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      65: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      66: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      67: [
        0,
        0.69444,
        0,
        0,
        0.63889
      ],
      68: [
        0,
        0.69444,
        0,
        0,
        0.72223
      ],
      69: [
        0,
        0.69444,
        0,
        0,
        0.59722
      ],
      70: [
        0,
        0.69444,
        0,
        0,
        0.56945
      ],
      71: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      72: [
        0,
        0.69444,
        0,
        0,
        0.70834
      ],
      73: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      74: [
        0,
        0.69444,
        0,
        0,
        0.47222
      ],
      75: [
        0,
        0.69444,
        0,
        0,
        0.69445
      ],
      76: [
        0,
        0.69444,
        0,
        0,
        0.54167
      ],
      77: [
        0,
        0.69444,
        0,
        0,
        0.875
      ],
      78: [
        0,
        0.69444,
        0,
        0,
        0.70834
      ],
      79: [
        0,
        0.69444,
        0,
        0,
        0.73611
      ],
      80: [
        0,
        0.69444,
        0,
        0,
        0.63889
      ],
      81: [
        0.125,
        0.69444,
        0,
        0,
        0.73611
      ],
      82: [
        0,
        0.69444,
        0,
        0,
        0.64584
      ],
      83: [
        0,
        0.69444,
        0,
        0,
        0.55556
      ],
      84: [
        0,
        0.69444,
        0,
        0,
        0.68056
      ],
      85: [
        0,
        0.69444,
        0,
        0,
        0.6875
      ],
      86: [
        0,
        0.69444,
        0.01389,
        0,
        0.66667
      ],
      87: [
        0,
        0.69444,
        0.01389,
        0,
        0.94445
      ],
      88: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      89: [
        0,
        0.69444,
        0.025,
        0,
        0.66667
      ],
      90: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      91: [
        0.25,
        0.75,
        0,
        0,
        0.28889
      ],
      93: [
        0.25,
        0.75,
        0,
        0,
        0.28889
      ],
      94: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      95: [
        0.35,
        0.09444,
        0.02778,
        0,
        0.5
      ],
      97: [
        0,
        0.44444,
        0,
        0,
        0.48056
      ],
      98: [
        0,
        0.69444,
        0,
        0,
        0.51667
      ],
      99: [
        0,
        0.44444,
        0,
        0,
        0.44445
      ],
      100: [
        0,
        0.69444,
        0,
        0,
        0.51667
      ],
      101: [
        0,
        0.44444,
        0,
        0,
        0.44445
      ],
      102: [
        0,
        0.69444,
        0.06944,
        0,
        0.30556
      ],
      103: [
        0.19444,
        0.44444,
        0.01389,
        0,
        0.5
      ],
      104: [
        0,
        0.69444,
        0,
        0,
        0.51667
      ],
      105: [
        0,
        0.67937,
        0,
        0,
        0.23889
      ],
      106: [
        0.19444,
        0.67937,
        0,
        0,
        0.26667
      ],
      107: [
        0,
        0.69444,
        0,
        0,
        0.48889
      ],
      108: [
        0,
        0.69444,
        0,
        0,
        0.23889
      ],
      109: [
        0,
        0.44444,
        0,
        0,
        0.79445
      ],
      110: [
        0,
        0.44444,
        0,
        0,
        0.51667
      ],
      111: [
        0,
        0.44444,
        0,
        0,
        0.5
      ],
      112: [
        0.19444,
        0.44444,
        0,
        0,
        0.51667
      ],
      113: [
        0.19444,
        0.44444,
        0,
        0,
        0.51667
      ],
      114: [
        0,
        0.44444,
        0.01389,
        0,
        0.34167
      ],
      115: [
        0,
        0.44444,
        0,
        0,
        0.38333
      ],
      116: [
        0,
        0.57143,
        0,
        0,
        0.36111
      ],
      117: [
        0,
        0.44444,
        0,
        0,
        0.51667
      ],
      118: [
        0,
        0.44444,
        0.01389,
        0,
        0.46111
      ],
      119: [
        0,
        0.44444,
        0.01389,
        0,
        0.68334
      ],
      120: [
        0,
        0.44444,
        0,
        0,
        0.46111
      ],
      121: [
        0.19444,
        0.44444,
        0.01389,
        0,
        0.46111
      ],
      122: [
        0,
        0.44444,
        0,
        0,
        0.43472
      ],
      126: [
        0.35,
        0.32659,
        0,
        0,
        0.5
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      168: [
        0,
        0.67937,
        0,
        0,
        0.5
      ],
      176: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      184: [
        0.17014,
        0,
        0,
        0,
        0.44445
      ],
      305: [
        0,
        0.44444,
        0,
        0,
        0.23889
      ],
      567: [
        0.19444,
        0.44444,
        0,
        0,
        0.26667
      ],
      710: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      711: [
        0,
        0.63194,
        0,
        0,
        0.5
      ],
      713: [
        0,
        0.60889,
        0,
        0,
        0.5
      ],
      714: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      715: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      728: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      729: [
        0,
        0.67937,
        0,
        0,
        0.27778
      ],
      730: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      732: [
        0,
        0.67659,
        0,
        0,
        0.5
      ],
      733: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      915: [
        0,
        0.69444,
        0,
        0,
        0.54167
      ],
      916: [
        0,
        0.69444,
        0,
        0,
        0.83334
      ],
      920: [
        0,
        0.69444,
        0,
        0,
        0.77778
      ],
      923: [
        0,
        0.69444,
        0,
        0,
        0.61111
      ],
      926: [
        0,
        0.69444,
        0,
        0,
        0.66667
      ],
      928: [
        0,
        0.69444,
        0,
        0,
        0.70834
      ],
      931: [
        0,
        0.69444,
        0,
        0,
        0.72222
      ],
      933: [
        0,
        0.69444,
        0,
        0,
        0.77778
      ],
      934: [
        0,
        0.69444,
        0,
        0,
        0.72222
      ],
      936: [
        0,
        0.69444,
        0,
        0,
        0.77778
      ],
      937: [
        0,
        0.69444,
        0,
        0,
        0.72222
      ],
      8211: [
        0,
        0.44444,
        0.02778,
        0,
        0.5
      ],
      8212: [
        0,
        0.44444,
        0.02778,
        0,
        1
      ],
      8216: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      8217: [
        0,
        0.69444,
        0,
        0,
        0.27778
      ],
      8220: [
        0,
        0.69444,
        0,
        0,
        0.5
      ],
      8221: [
        0,
        0.69444,
        0,
        0,
        0.5
      ]
    },
    "Script-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      65: [
        0,
        0.7,
        0.22925,
        0,
        0.80253
      ],
      66: [
        0,
        0.7,
        0.04087,
        0,
        0.90757
      ],
      67: [
        0,
        0.7,
        0.1689,
        0,
        0.66619
      ],
      68: [
        0,
        0.7,
        0.09371,
        0,
        0.77443
      ],
      69: [
        0,
        0.7,
        0.18583,
        0,
        0.56162
      ],
      70: [
        0,
        0.7,
        0.13634,
        0,
        0.89544
      ],
      71: [
        0,
        0.7,
        0.17322,
        0,
        0.60961
      ],
      72: [
        0,
        0.7,
        0.29694,
        0,
        0.96919
      ],
      73: [
        0,
        0.7,
        0.19189,
        0,
        0.80907
      ],
      74: [
        0.27778,
        0.7,
        0.19189,
        0,
        1.05159
      ],
      75: [
        0,
        0.7,
        0.31259,
        0,
        0.91364
      ],
      76: [
        0,
        0.7,
        0.19189,
        0,
        0.87373
      ],
      77: [
        0,
        0.7,
        0.15981,
        0,
        1.08031
      ],
      78: [
        0,
        0.7,
        0.3525,
        0,
        0.9015
      ],
      79: [
        0,
        0.7,
        0.08078,
        0,
        0.73787
      ],
      80: [
        0,
        0.7,
        0.08078,
        0,
        1.01262
      ],
      81: [
        0,
        0.7,
        0.03305,
        0,
        0.88282
      ],
      82: [
        0,
        0.7,
        0.06259,
        0,
        0.85
      ],
      83: [
        0,
        0.7,
        0.19189,
        0,
        0.86767
      ],
      84: [
        0,
        0.7,
        0.29087,
        0,
        0.74697
      ],
      85: [
        0,
        0.7,
        0.25815,
        0,
        0.79996
      ],
      86: [
        0,
        0.7,
        0.27523,
        0,
        0.62204
      ],
      87: [
        0,
        0.7,
        0.27523,
        0,
        0.80532
      ],
      88: [
        0,
        0.7,
        0.26006,
        0,
        0.94445
      ],
      89: [
        0,
        0.7,
        0.2939,
        0,
        0.70961
      ],
      90: [
        0,
        0.7,
        0.24037,
        0,
        0.8212
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ]
    },
    "Size1-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      40: [
        0.35001,
        0.85,
        0,
        0,
        0.45834
      ],
      41: [
        0.35001,
        0.85,
        0,
        0,
        0.45834
      ],
      47: [
        0.35001,
        0.85,
        0,
        0,
        0.57778
      ],
      91: [
        0.35001,
        0.85,
        0,
        0,
        0.41667
      ],
      92: [
        0.35001,
        0.85,
        0,
        0,
        0.57778
      ],
      93: [
        0.35001,
        0.85,
        0,
        0,
        0.41667
      ],
      123: [
        0.35001,
        0.85,
        0,
        0,
        0.58334
      ],
      125: [
        0.35001,
        0.85,
        0,
        0,
        0.58334
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      710: [
        0,
        0.72222,
        0,
        0,
        0.55556
      ],
      732: [
        0,
        0.72222,
        0,
        0,
        0.55556
      ],
      770: [
        0,
        0.72222,
        0,
        0,
        0.55556
      ],
      771: [
        0,
        0.72222,
        0,
        0,
        0.55556
      ],
      8214: [
        -99e-5,
        0.601,
        0,
        0,
        0.77778
      ],
      8593: [
        1e-5,
        0.6,
        0,
        0,
        0.66667
      ],
      8595: [
        1e-5,
        0.6,
        0,
        0,
        0.66667
      ],
      8657: [
        1e-5,
        0.6,
        0,
        0,
        0.77778
      ],
      8659: [
        1e-5,
        0.6,
        0,
        0,
        0.77778
      ],
      8719: [
        0.25001,
        0.75,
        0,
        0,
        0.94445
      ],
      8720: [
        0.25001,
        0.75,
        0,
        0,
        0.94445
      ],
      8721: [
        0.25001,
        0.75,
        0,
        0,
        1.05556
      ],
      8730: [
        0.35001,
        0.85,
        0,
        0,
        1
      ],
      8739: [
        -599e-5,
        0.606,
        0,
        0,
        0.33333
      ],
      8741: [
        -599e-5,
        0.606,
        0,
        0,
        0.55556
      ],
      8747: [
        0.30612,
        0.805,
        0.19445,
        0,
        0.47222
      ],
      8748: [
        0.306,
        0.805,
        0.19445,
        0,
        0.47222
      ],
      8749: [
        0.306,
        0.805,
        0.19445,
        0,
        0.47222
      ],
      8750: [
        0.30612,
        0.805,
        0.19445,
        0,
        0.47222
      ],
      8896: [
        0.25001,
        0.75,
        0,
        0,
        0.83334
      ],
      8897: [
        0.25001,
        0.75,
        0,
        0,
        0.83334
      ],
      8898: [
        0.25001,
        0.75,
        0,
        0,
        0.83334
      ],
      8899: [
        0.25001,
        0.75,
        0,
        0,
        0.83334
      ],
      8968: [
        0.35001,
        0.85,
        0,
        0,
        0.47222
      ],
      8969: [
        0.35001,
        0.85,
        0,
        0,
        0.47222
      ],
      8970: [
        0.35001,
        0.85,
        0,
        0,
        0.47222
      ],
      8971: [
        0.35001,
        0.85,
        0,
        0,
        0.47222
      ],
      9168: [
        -99e-5,
        0.601,
        0,
        0,
        0.66667
      ],
      10216: [
        0.35001,
        0.85,
        0,
        0,
        0.47222
      ],
      10217: [
        0.35001,
        0.85,
        0,
        0,
        0.47222
      ],
      10752: [
        0.25001,
        0.75,
        0,
        0,
        1.11111
      ],
      10753: [
        0.25001,
        0.75,
        0,
        0,
        1.11111
      ],
      10754: [
        0.25001,
        0.75,
        0,
        0,
        1.11111
      ],
      10756: [
        0.25001,
        0.75,
        0,
        0,
        0.83334
      ],
      10758: [
        0.25001,
        0.75,
        0,
        0,
        0.83334
      ]
    },
    "Size2-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      40: [
        0.65002,
        1.15,
        0,
        0,
        0.59722
      ],
      41: [
        0.65002,
        1.15,
        0,
        0,
        0.59722
      ],
      47: [
        0.65002,
        1.15,
        0,
        0,
        0.81111
      ],
      91: [
        0.65002,
        1.15,
        0,
        0,
        0.47222
      ],
      92: [
        0.65002,
        1.15,
        0,
        0,
        0.81111
      ],
      93: [
        0.65002,
        1.15,
        0,
        0,
        0.47222
      ],
      123: [
        0.65002,
        1.15,
        0,
        0,
        0.66667
      ],
      125: [
        0.65002,
        1.15,
        0,
        0,
        0.66667
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      710: [
        0,
        0.75,
        0,
        0,
        1
      ],
      732: [
        0,
        0.75,
        0,
        0,
        1
      ],
      770: [
        0,
        0.75,
        0,
        0,
        1
      ],
      771: [
        0,
        0.75,
        0,
        0,
        1
      ],
      8719: [
        0.55001,
        1.05,
        0,
        0,
        1.27778
      ],
      8720: [
        0.55001,
        1.05,
        0,
        0,
        1.27778
      ],
      8721: [
        0.55001,
        1.05,
        0,
        0,
        1.44445
      ],
      8730: [
        0.65002,
        1.15,
        0,
        0,
        1
      ],
      8747: [
        0.86225,
        1.36,
        0.44445,
        0,
        0.55556
      ],
      8748: [
        0.862,
        1.36,
        0.44445,
        0,
        0.55556
      ],
      8749: [
        0.862,
        1.36,
        0.44445,
        0,
        0.55556
      ],
      8750: [
        0.86225,
        1.36,
        0.44445,
        0,
        0.55556
      ],
      8896: [
        0.55001,
        1.05,
        0,
        0,
        1.11111
      ],
      8897: [
        0.55001,
        1.05,
        0,
        0,
        1.11111
      ],
      8898: [
        0.55001,
        1.05,
        0,
        0,
        1.11111
      ],
      8899: [
        0.55001,
        1.05,
        0,
        0,
        1.11111
      ],
      8968: [
        0.65002,
        1.15,
        0,
        0,
        0.52778
      ],
      8969: [
        0.65002,
        1.15,
        0,
        0,
        0.52778
      ],
      8970: [
        0.65002,
        1.15,
        0,
        0,
        0.52778
      ],
      8971: [
        0.65002,
        1.15,
        0,
        0,
        0.52778
      ],
      10216: [
        0.65002,
        1.15,
        0,
        0,
        0.61111
      ],
      10217: [
        0.65002,
        1.15,
        0,
        0,
        0.61111
      ],
      10752: [
        0.55001,
        1.05,
        0,
        0,
        1.51112
      ],
      10753: [
        0.55001,
        1.05,
        0,
        0,
        1.51112
      ],
      10754: [
        0.55001,
        1.05,
        0,
        0,
        1.51112
      ],
      10756: [
        0.55001,
        1.05,
        0,
        0,
        1.11111
      ],
      10758: [
        0.55001,
        1.05,
        0,
        0,
        1.11111
      ]
    },
    "Size3-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      40: [
        0.95003,
        1.45,
        0,
        0,
        0.73611
      ],
      41: [
        0.95003,
        1.45,
        0,
        0,
        0.73611
      ],
      47: [
        0.95003,
        1.45,
        0,
        0,
        1.04445
      ],
      91: [
        0.95003,
        1.45,
        0,
        0,
        0.52778
      ],
      92: [
        0.95003,
        1.45,
        0,
        0,
        1.04445
      ],
      93: [
        0.95003,
        1.45,
        0,
        0,
        0.52778
      ],
      123: [
        0.95003,
        1.45,
        0,
        0,
        0.75
      ],
      125: [
        0.95003,
        1.45,
        0,
        0,
        0.75
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      710: [
        0,
        0.75,
        0,
        0,
        1.44445
      ],
      732: [
        0,
        0.75,
        0,
        0,
        1.44445
      ],
      770: [
        0,
        0.75,
        0,
        0,
        1.44445
      ],
      771: [
        0,
        0.75,
        0,
        0,
        1.44445
      ],
      8730: [
        0.95003,
        1.45,
        0,
        0,
        1
      ],
      8968: [
        0.95003,
        1.45,
        0,
        0,
        0.58334
      ],
      8969: [
        0.95003,
        1.45,
        0,
        0,
        0.58334
      ],
      8970: [
        0.95003,
        1.45,
        0,
        0,
        0.58334
      ],
      8971: [
        0.95003,
        1.45,
        0,
        0,
        0.58334
      ],
      10216: [
        0.95003,
        1.45,
        0,
        0,
        0.75
      ],
      10217: [
        0.95003,
        1.45,
        0,
        0,
        0.75
      ]
    },
    "Size4-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.25
      ],
      40: [
        1.25003,
        1.75,
        0,
        0,
        0.79167
      ],
      41: [
        1.25003,
        1.75,
        0,
        0,
        0.79167
      ],
      47: [
        1.25003,
        1.75,
        0,
        0,
        1.27778
      ],
      91: [
        1.25003,
        1.75,
        0,
        0,
        0.58334
      ],
      92: [
        1.25003,
        1.75,
        0,
        0,
        1.27778
      ],
      93: [
        1.25003,
        1.75,
        0,
        0,
        0.58334
      ],
      123: [
        1.25003,
        1.75,
        0,
        0,
        0.80556
      ],
      125: [
        1.25003,
        1.75,
        0,
        0,
        0.80556
      ],
      160: [
        0,
        0,
        0,
        0,
        0.25
      ],
      710: [
        0,
        0.825,
        0,
        0,
        1.8889
      ],
      732: [
        0,
        0.825,
        0,
        0,
        1.8889
      ],
      770: [
        0,
        0.825,
        0,
        0,
        1.8889
      ],
      771: [
        0,
        0.825,
        0,
        0,
        1.8889
      ],
      8730: [
        1.25003,
        1.75,
        0,
        0,
        1
      ],
      8968: [
        1.25003,
        1.75,
        0,
        0,
        0.63889
      ],
      8969: [
        1.25003,
        1.75,
        0,
        0,
        0.63889
      ],
      8970: [
        1.25003,
        1.75,
        0,
        0,
        0.63889
      ],
      8971: [
        1.25003,
        1.75,
        0,
        0,
        0.63889
      ],
      9115: [
        0.64502,
        1.155,
        0,
        0,
        0.875
      ],
      9116: [
        1e-5,
        0.6,
        0,
        0,
        0.875
      ],
      9117: [
        0.64502,
        1.155,
        0,
        0,
        0.875
      ],
      9118: [
        0.64502,
        1.155,
        0,
        0,
        0.875
      ],
      9119: [
        1e-5,
        0.6,
        0,
        0,
        0.875
      ],
      9120: [
        0.64502,
        1.155,
        0,
        0,
        0.875
      ],
      9121: [
        0.64502,
        1.155,
        0,
        0,
        0.66667
      ],
      9122: [
        -99e-5,
        0.601,
        0,
        0,
        0.66667
      ],
      9123: [
        0.64502,
        1.155,
        0,
        0,
        0.66667
      ],
      9124: [
        0.64502,
        1.155,
        0,
        0,
        0.66667
      ],
      9125: [
        -99e-5,
        0.601,
        0,
        0,
        0.66667
      ],
      9126: [
        0.64502,
        1.155,
        0,
        0,
        0.66667
      ],
      9127: [
        1e-5,
        0.9,
        0,
        0,
        0.88889
      ],
      9128: [
        0.65002,
        1.15,
        0,
        0,
        0.88889
      ],
      9129: [
        0.90001,
        0,
        0,
        0,
        0.88889
      ],
      9130: [
        0,
        0.3,
        0,
        0,
        0.88889
      ],
      9131: [
        1e-5,
        0.9,
        0,
        0,
        0.88889
      ],
      9132: [
        0.65002,
        1.15,
        0,
        0,
        0.88889
      ],
      9133: [
        0.90001,
        0,
        0,
        0,
        0.88889
      ],
      9143: [
        0.88502,
        0.915,
        0,
        0,
        1.05556
      ],
      10216: [
        1.25003,
        1.75,
        0,
        0,
        0.80556
      ],
      10217: [
        1.25003,
        1.75,
        0,
        0,
        0.80556
      ],
      57344: [
        -499e-5,
        0.605,
        0,
        0,
        1.05556
      ],
      57345: [
        -499e-5,
        0.605,
        0,
        0,
        1.05556
      ],
      57680: [
        0,
        0.12,
        0,
        0,
        0.45
      ],
      57681: [
        0,
        0.12,
        0,
        0,
        0.45
      ],
      57682: [
        0,
        0.12,
        0,
        0,
        0.45
      ],
      57683: [
        0,
        0.12,
        0,
        0,
        0.45
      ]
    },
    "Typewriter-Regular": {
      32: [
        0,
        0,
        0,
        0,
        0.525
      ],
      33: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      34: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      35: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      36: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      37: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      38: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      39: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      40: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      41: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      42: [
        0,
        0.52083,
        0,
        0,
        0.525
      ],
      43: [
        -0.08056,
        0.53055,
        0,
        0,
        0.525
      ],
      44: [
        0.13889,
        0.125,
        0,
        0,
        0.525
      ],
      45: [
        -0.08056,
        0.53055,
        0,
        0,
        0.525
      ],
      46: [
        0,
        0.125,
        0,
        0,
        0.525
      ],
      47: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      48: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      49: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      50: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      51: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      52: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      53: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      54: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      55: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      56: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      57: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      58: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      59: [
        0.13889,
        0.43056,
        0,
        0,
        0.525
      ],
      60: [
        -0.05556,
        0.55556,
        0,
        0,
        0.525
      ],
      61: [
        -0.19549,
        0.41562,
        0,
        0,
        0.525
      ],
      62: [
        -0.05556,
        0.55556,
        0,
        0,
        0.525
      ],
      63: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      64: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      65: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      66: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      67: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      68: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      69: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      70: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      71: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      72: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      73: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      74: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      75: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      76: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      77: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      78: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      79: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      80: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      81: [
        0.13889,
        0.61111,
        0,
        0,
        0.525
      ],
      82: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      83: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      84: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      85: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      86: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      87: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      88: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      89: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      90: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      91: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      92: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      93: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      94: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      95: [
        0.09514,
        0,
        0,
        0,
        0.525
      ],
      96: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      97: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      98: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      99: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      100: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      101: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      102: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      103: [
        0.22222,
        0.43056,
        0,
        0,
        0.525
      ],
      104: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      105: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      106: [
        0.22222,
        0.61111,
        0,
        0,
        0.525
      ],
      107: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      108: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      109: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      110: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      111: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      112: [
        0.22222,
        0.43056,
        0,
        0,
        0.525
      ],
      113: [
        0.22222,
        0.43056,
        0,
        0,
        0.525
      ],
      114: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      115: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      116: [
        0,
        0.55358,
        0,
        0,
        0.525
      ],
      117: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      118: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      119: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      120: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      121: [
        0.22222,
        0.43056,
        0,
        0,
        0.525
      ],
      122: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      123: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      124: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      125: [
        0.08333,
        0.69444,
        0,
        0,
        0.525
      ],
      126: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      127: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      160: [
        0,
        0,
        0,
        0,
        0.525
      ],
      176: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      184: [
        0.19445,
        0,
        0,
        0,
        0.525
      ],
      305: [
        0,
        0.43056,
        0,
        0,
        0.525
      ],
      567: [
        0.22222,
        0.43056,
        0,
        0,
        0.525
      ],
      711: [
        0,
        0.56597,
        0,
        0,
        0.525
      ],
      713: [
        0,
        0.56555,
        0,
        0,
        0.525
      ],
      714: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      715: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      728: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      730: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      770: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      771: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      776: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      915: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      916: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      920: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      923: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      926: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      928: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      931: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      933: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      934: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      936: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      937: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      8216: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      8217: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      8242: [
        0,
        0.61111,
        0,
        0,
        0.525
      ],
      9251: [
        0.11111,
        0.21944,
        0,
        0,
        0.525
      ]
    }
  }, Re = {
    slant: [
      0.25,
      0.25,
      0.25
    ],
    space: [
      0,
      0,
      0
    ],
    stretch: [
      0,
      0,
      0
    ],
    shrink: [
      0,
      0,
      0
    ],
    xHeight: [
      0.431,
      0.431,
      0.431
    ],
    quad: [
      1,
      1.171,
      1.472
    ],
    extraSpace: [
      0,
      0,
      0
    ],
    num1: [
      0.677,
      0.732,
      0.925
    ],
    num2: [
      0.394,
      0.384,
      0.387
    ],
    num3: [
      0.444,
      0.471,
      0.504
    ],
    denom1: [
      0.686,
      0.752,
      1.025
    ],
    denom2: [
      0.345,
      0.344,
      0.532
    ],
    sup1: [
      0.413,
      0.503,
      0.504
    ],
    sup2: [
      0.363,
      0.431,
      0.404
    ],
    sup3: [
      0.289,
      0.286,
      0.294
    ],
    sub1: [
      0.15,
      0.143,
      0.2
    ],
    sub2: [
      0.247,
      0.286,
      0.4
    ],
    supDrop: [
      0.386,
      0.353,
      0.494
    ],
    subDrop: [
      0.05,
      0.071,
      0.1
    ],
    delim1: [
      2.39,
      1.7,
      1.98
    ],
    delim2: [
      1.01,
      1.157,
      1.42
    ],
    axisHeight: [
      0.25,
      0.25,
      0.25
    ],
    defaultRuleThickness: [
      0.04,
      0.049,
      0.049
    ],
    bigOpSpacing1: [
      0.111,
      0.111,
      0.111
    ],
    bigOpSpacing2: [
      0.166,
      0.166,
      0.166
    ],
    bigOpSpacing3: [
      0.2,
      0.2,
      0.2
    ],
    bigOpSpacing4: [
      0.6,
      0.611,
      0.611
    ],
    bigOpSpacing5: [
      0.1,
      0.143,
      0.143
    ],
    sqrtRuleThickness: [
      0.04,
      0.04,
      0.04
    ],
    ptPerEm: [
      10,
      10,
      10
    ],
    doubleRuleSep: [
      0.2,
      0.2,
      0.2
    ],
    arrayRuleWidth: [
      0.04,
      0.04,
      0.04
    ],
    fboxsep: [
      0.3,
      0.3,
      0.3
    ],
    fboxrule: [
      0.04,
      0.04,
      0.04
    ]
  }, Tr = {
    \u00C5: "A",
    \u00D0: "D",
    \u00DE: "o",
    \u00E5: "a",
    \u00F0: "d",
    \u00FE: "o",
    \u0410: "A",
    \u0411: "B",
    \u0412: "B",
    \u0413: "F",
    \u0414: "A",
    \u0415: "E",
    \u0416: "K",
    \u0417: "3",
    \u0418: "N",
    \u0419: "N",
    \u041A: "K",
    \u041B: "N",
    \u041C: "M",
    \u041D: "H",
    \u041E: "O",
    \u041F: "N",
    \u0420: "P",
    \u0421: "C",
    \u0422: "T",
    \u0423: "y",
    \u0424: "O",
    \u0425: "X",
    \u0426: "U",
    \u0427: "h",
    \u0428: "W",
    \u0429: "W",
    \u042A: "B",
    \u042B: "X",
    \u042C: "B",
    \u042D: "3",
    \u042E: "X",
    \u042F: "R",
    \u0430: "a",
    \u0431: "b",
    \u0432: "a",
    \u0433: "r",
    \u0434: "y",
    \u0435: "e",
    \u0436: "m",
    \u0437: "e",
    \u0438: "n",
    \u0439: "n",
    \u043A: "n",
    \u043B: "n",
    \u043C: "m",
    \u043D: "n",
    \u043E: "o",
    \u043F: "n",
    \u0440: "p",
    \u0441: "c",
    \u0442: "o",
    \u0443: "y",
    \u0444: "b",
    \u0445: "x",
    \u0446: "n",
    \u0447: "n",
    \u0448: "w",
    \u0449: "w",
    \u044A: "a",
    \u044B: "m",
    \u044C: "a",
    \u044D: "e",
    \u044E: "m",
    \u044F: "r"
  };
  function ri(r, e) {
    T0[r] = e;
  }
  function _t(r, e, t) {
    if (!T0[e]) throw new Error("Font metrics not found for font: " + e + ".");
    var a = r.charCodeAt(0), n = T0[e][a];
    if (!n && r[0] in Tr && (a = Tr[r[0]].charCodeAt(0), n = T0[e][a]), !n && t === "text" && ka(a) && (n = T0[e][77]), n) return {
      depth: n[0],
      height: n[1],
      italic: n[2],
      skew: n[3],
      width: n[4]
    };
  }
  var gt = {};
  function ai(r) {
    var e;
    if (r >= 5 ? e = 0 : r >= 3 ? e = 1 : e = 2, !gt[e]) {
      var t = gt[e] = {
        cssEmPerMu: Re.quad[e] / 18
      };
      for (var a in Re) Re.hasOwnProperty(a) && (t[a] = Re[a][e]);
    }
    return gt[e];
  }
  var ni = [
    [
      1,
      1,
      1
    ],
    [
      2,
      1,
      1
    ],
    [
      3,
      1,
      1
    ],
    [
      4,
      2,
      1
    ],
    [
      5,
      2,
      1
    ],
    [
      6,
      3,
      1
    ],
    [
      7,
      4,
      2
    ],
    [
      8,
      6,
      3
    ],
    [
      9,
      7,
      6
    ],
    [
      10,
      8,
      7
    ],
    [
      11,
      10,
      9
    ]
  ], Br = [
    0.5,
    0.6,
    0.7,
    0.8,
    0.9,
    1,
    1.2,
    1.44,
    1.728,
    2.074,
    2.488
  ], Cr = function(e, t) {
    return t.size < 2 ? e : ni[e - 1][t.size - 1];
  };
  class R0 {
    constructor(e) {
      this.style = void 0, this.color = void 0, this.size = void 0, this.textSize = void 0, this.phantom = void 0, this.font = void 0, this.fontFamily = void 0, this.fontWeight = void 0, this.fontShape = void 0, this.sizeMultiplier = void 0, this.maxSize = void 0, this.minRuleThickness = void 0, this._fontMetrics = void 0, this.style = e.style, this.color = e.color, this.size = e.size || R0.BASESIZE, this.textSize = e.textSize || this.size, this.phantom = !!e.phantom, this.font = e.font || "", this.fontFamily = e.fontFamily || "", this.fontWeight = e.fontWeight || "", this.fontShape = e.fontShape || "", this.sizeMultiplier = Br[this.size - 1], this.maxSize = e.maxSize, this.minRuleThickness = e.minRuleThickness, this._fontMetrics = void 0;
    }
    extend(e) {
      var t = {
        style: this.style,
        size: this.size,
        textSize: this.textSize,
        color: this.color,
        phantom: this.phantom,
        font: this.font,
        fontFamily: this.fontFamily,
        fontWeight: this.fontWeight,
        fontShape: this.fontShape,
        maxSize: this.maxSize,
        minRuleThickness: this.minRuleThickness
      };
      for (var a in e) e.hasOwnProperty(a) && (t[a] = e[a]);
      return new R0(t);
    }
    havingStyle(e) {
      return this.style === e ? this : this.extend({
        style: e,
        size: Cr(this.textSize, e)
      });
    }
    havingCrampedStyle() {
      return this.havingStyle(this.style.cramp());
    }
    havingSize(e) {
      return this.size === e && this.textSize === e ? this : this.extend({
        style: this.style.text(),
        size: e,
        textSize: e,
        sizeMultiplier: Br[e - 1]
      });
    }
    havingBaseStyle(e) {
      e = e || this.style.text();
      var t = Cr(R0.BASESIZE, e);
      return this.size === t && this.textSize === R0.BASESIZE && this.style === e ? this : this.extend({
        style: e,
        size: t
      });
    }
    havingBaseSizing() {
      var e;
      switch (this.style.id) {
        case 4:
        case 5:
          e = 3;
          break;
        case 6:
        case 7:
          e = 1;
          break;
        default:
          e = 6;
      }
      return this.extend({
        style: this.style.text(),
        size: e
      });
    }
    withColor(e) {
      return this.extend({
        color: e
      });
    }
    withPhantom() {
      return this.extend({
        phantom: true
      });
    }
    withFont(e) {
      return this.extend({
        font: e
      });
    }
    withTextFontFamily(e) {
      return this.extend({
        fontFamily: e,
        font: ""
      });
    }
    withTextFontWeight(e) {
      return this.extend({
        fontWeight: e,
        font: ""
      });
    }
    withTextFontShape(e) {
      return this.extend({
        fontShape: e,
        font: ""
      });
    }
    sizingClasses(e) {
      return e.size !== this.size ? [
        "sizing",
        "reset-size" + e.size,
        "size" + this.size
      ] : [];
    }
    baseSizingClasses() {
      return this.size !== R0.BASESIZE ? [
        "sizing",
        "reset-size" + this.size,
        "size" + R0.BASESIZE
      ] : [];
    }
    fontMetrics() {
      return this._fontMetrics || (this._fontMetrics = ai(this.size)), this._fontMetrics;
    }
    getColor() {
      return this.phantom ? "transparent" : this.color;
    }
  }
  R0.BASESIZE = 6;
  var Lt = {
    pt: 1,
    mm: 7227 / 2540,
    cm: 7227 / 254,
    in: 72.27,
    bp: 803 / 800,
    pc: 12,
    dd: 1238 / 1157,
    cc: 14856 / 1157,
    nd: 685 / 642,
    nc: 1370 / 107,
    sp: 1 / 65536,
    px: 803 / 800
  }, ii = {
    ex: true,
    em: true,
    mu: true
  }, Sa = function(e) {
    return typeof e != "string" && (e = e.unit), e in Lt || e in ii || e === "ex";
  }, K = function(e, t) {
    var a;
    if (e.unit in Lt) a = Lt[e.unit] / t.fontMetrics().ptPerEm / t.sizeMultiplier;
    else if (e.unit === "mu") a = t.fontMetrics().cssEmPerMu;
    else {
      var n;
      if (t.style.isTight() ? n = t.havingStyle(t.style.text()) : n = t, e.unit === "ex") a = n.fontMetrics().xHeight;
      else if (e.unit === "em") a = n.fontMetrics().quad;
      else throw new T("Invalid unit: '" + e.unit + "'");
      n !== t && (a *= n.sizeMultiplier / t.sizeMultiplier);
    }
    return Math.min(e.number * a, t.maxSize);
  }, C = function(e) {
    return +e.toFixed(4) + "em";
  }, K0 = function(e) {
    return e.filter((t) => t).join(" ");
  }, Ma = function(e, t, a) {
    if (this.classes = e || [], this.attributes = {}, this.height = 0, this.depth = 0, this.maxFontSize = 0, this.style = a || {}, t) {
      t.style.isTight() && this.classes.push("mtight");
      var n = t.getColor();
      n && (this.style.color = n);
    }
  }, za = function(e) {
    var t = document.createElement(e);
    t.className = K0(this.classes);
    for (var a in this.style) this.style.hasOwnProperty(a) && (t.style[a] = this.style[a]);
    for (var n in this.attributes) this.attributes.hasOwnProperty(n) && t.setAttribute(n, this.attributes[n]);
    for (var i = 0; i < this.children.length; i++) t.appendChild(this.children[i].toNode());
    return t;
  }, si = /[\s"'>/=\x00-\x1f]/, Aa = function(e) {
    var t = "<" + e;
    this.classes.length && (t += ' class="' + Y.escape(K0(this.classes)) + '"');
    var a = "";
    for (var n in this.style) this.style.hasOwnProperty(n) && (a += Y.hyphenate(n) + ":" + this.style[n] + ";");
    a && (t += ' style="' + Y.escape(a) + '"');
    for (var i in this.attributes) if (this.attributes.hasOwnProperty(i)) {
      if (si.test(i)) throw new T("Invalid attribute name '" + i + "'");
      t += " " + i + '="' + Y.escape(this.attributes[i]) + '"';
    }
    t += ">";
    for (var o = 0; o < this.children.length; o++) t += this.children[o].toMarkup();
    return t += "</" + e + ">", t;
  };
  class Be {
    constructor(e, t, a, n) {
      this.children = void 0, this.attributes = void 0, this.classes = void 0, this.height = void 0, this.depth = void 0, this.width = void 0, this.maxFontSize = void 0, this.style = void 0, Ma.call(this, e, a, n), this.children = t || [];
    }
    setAttribute(e, t) {
      this.attributes[e] = t;
    }
    hasClass(e) {
      return this.classes.includes(e);
    }
    toNode() {
      return za.call(this, "span");
    }
    toMarkup() {
      return Aa.call(this, "span");
    }
  }
  class Jt {
    constructor(e, t, a, n) {
      this.children = void 0, this.attributes = void 0, this.classes = void 0, this.height = void 0, this.depth = void 0, this.maxFontSize = void 0, this.style = void 0, Ma.call(this, t, n), this.children = a || [], this.setAttribute("href", e);
    }
    setAttribute(e, t) {
      this.attributes[e] = t;
    }
    hasClass(e) {
      return this.classes.includes(e);
    }
    toNode() {
      return za.call(this, "a");
    }
    toMarkup() {
      return Aa.call(this, "a");
    }
  }
  class li {
    constructor(e, t, a) {
      this.src = void 0, this.alt = void 0, this.classes = void 0, this.height = void 0, this.depth = void 0, this.maxFontSize = void 0, this.style = void 0, this.alt = t, this.src = e, this.classes = [
        "mord"
      ], this.style = a;
    }
    hasClass(e) {
      return this.classes.includes(e);
    }
    toNode() {
      var e = document.createElement("img");
      e.src = this.src, e.alt = this.alt, e.className = "mord";
      for (var t in this.style) this.style.hasOwnProperty(t) && (e.style[t] = this.style[t]);
      return e;
    }
    toMarkup() {
      var e = '<img src="' + Y.escape(this.src) + '"' + (' alt="' + Y.escape(this.alt) + '"'), t = "";
      for (var a in this.style) this.style.hasOwnProperty(a) && (t += Y.hyphenate(a) + ":" + this.style[a] + ";");
      return t && (e += ' style="' + Y.escape(t) + '"'), e += "'/>", e;
    }
  }
  var oi = {
    \u00EE: "\u0131\u0302",
    \u00EF: "\u0131\u0308",
    \u00ED: "\u0131\u0301",
    \u00EC: "\u0131\u0300"
  };
  class b0 {
    constructor(e, t, a, n, i, o, u, m) {
      this.text = void 0, this.height = void 0, this.depth = void 0, this.italic = void 0, this.skew = void 0, this.width = void 0, this.maxFontSize = void 0, this.classes = void 0, this.style = void 0, this.text = e, this.height = t || 0, this.depth = a || 0, this.italic = n || 0, this.skew = i || 0, this.width = o || 0, this.classes = u || [], this.style = m || {}, this.maxFontSize = 0;
      var d = Yn(this.text.charCodeAt(0));
      d && this.classes.push(d + "_fallback"), /[îïíì]/.test(this.text) && (this.text = oi[this.text]);
    }
    hasClass(e) {
      return this.classes.includes(e);
    }
    toNode() {
      var e = document.createTextNode(this.text), t = null;
      this.italic > 0 && (t = document.createElement("span"), t.style.marginRight = C(this.italic)), this.classes.length > 0 && (t = t || document.createElement("span"), t.className = K0(this.classes));
      for (var a in this.style) this.style.hasOwnProperty(a) && (t = t || document.createElement("span"), t.style[a] = this.style[a]);
      return t ? (t.appendChild(e), t) : e;
    }
    toMarkup() {
      var e = false, t = "<span";
      this.classes.length && (e = true, t += ' class="', t += Y.escape(K0(this.classes)), t += '"');
      var a = "";
      this.italic > 0 && (a += "margin-right:" + this.italic + "em;");
      for (var n in this.style) this.style.hasOwnProperty(n) && (a += Y.hyphenate(n) + ":" + this.style[n] + ";");
      a && (e = true, t += ' style="' + Y.escape(a) + '"');
      var i = Y.escape(this.text);
      return e ? (t += ">", t += i, t += "</span>", t) : i;
    }
  }
  class H0 {
    constructor(e, t) {
      this.children = void 0, this.attributes = void 0, this.children = e || [], this.attributes = t || {};
    }
    toNode() {
      var e = "http://www.w3.org/2000/svg", t = document.createElementNS(e, "svg");
      for (var a in this.attributes) Object.prototype.hasOwnProperty.call(this.attributes, a) && t.setAttribute(a, this.attributes[a]);
      for (var n = 0; n < this.children.length; n++) t.appendChild(this.children[n].toNode());
      return t;
    }
    toMarkup() {
      var e = '<svg xmlns="http://www.w3.org/2000/svg"';
      for (var t in this.attributes) Object.prototype.hasOwnProperty.call(this.attributes, t) && (e += " " + t + '="' + Y.escape(this.attributes[t]) + '"');
      e += ">";
      for (var a = 0; a < this.children.length; a++) e += this.children[a].toMarkup();
      return e += "</svg>", e;
    }
  }
  class _0 {
    constructor(e, t) {
      this.pathName = void 0, this.alternate = void 0, this.pathName = e, this.alternate = t;
    }
    toNode() {
      var e = "http://www.w3.org/2000/svg", t = document.createElementNS(e, "path");
      return this.alternate ? t.setAttribute("d", this.alternate) : t.setAttribute("d", Ar[this.pathName]), t;
    }
    toMarkup() {
      return this.alternate ? '<path d="' + Y.escape(this.alternate) + '"/>' : '<path d="' + Y.escape(Ar[this.pathName]) + '"/>';
    }
  }
  class Pt {
    constructor(e) {
      this.attributes = void 0, this.attributes = e || {};
    }
    toNode() {
      var e = "http://www.w3.org/2000/svg", t = document.createElementNS(e, "line");
      for (var a in this.attributes) Object.prototype.hasOwnProperty.call(this.attributes, a) && t.setAttribute(a, this.attributes[a]);
      return t;
    }
    toMarkup() {
      var e = "<line";
      for (var t in this.attributes) Object.prototype.hasOwnProperty.call(this.attributes, t) && (e += " " + t + '="' + Y.escape(this.attributes[t]) + '"');
      return e += "/>", e;
    }
  }
  function Dr(r) {
    if (r instanceof b0) return r;
    throw new Error("Expected symbolNode but got " + String(r) + ".");
  }
  function ui(r) {
    if (r instanceof Be) return r;
    throw new Error("Expected span<HtmlDomNode> but got " + String(r) + ".");
  }
  var hi = {
    bin: 1,
    close: 1,
    inner: 1,
    open: 1,
    punct: 1,
    rel: 1
  }, mi = {
    "accent-token": 1,
    mathord: 1,
    "op-token": 1,
    spacing: 1,
    textord: 1
  }, W = {
    math: {},
    text: {}
  };
  function s(r, e, t, a, n, i) {
    W[r][n] = {
      font: e,
      group: t,
      replace: a
    }, i && a && (W[r][a] = W[r][n]);
  }
  var l = "math", M = "text", h = "main", f = "ams", Z = "accent-token", q = "bin", l0 = "close", pe = "inner", R = "mathord", Q = "op-token", f0 = "open", tt = "punct", v = "rel", V0 = "spacing", g = "textord";
  s(l, h, v, "\u2261", "\\equiv", true);
  s(l, h, v, "\u227A", "\\prec", true);
  s(l, h, v, "\u227B", "\\succ", true);
  s(l, h, v, "\u223C", "\\sim", true);
  s(l, h, v, "\u22A5", "\\perp");
  s(l, h, v, "\u2AAF", "\\preceq", true);
  s(l, h, v, "\u2AB0", "\\succeq", true);
  s(l, h, v, "\u2243", "\\simeq", true);
  s(l, h, v, "\u2223", "\\mid", true);
  s(l, h, v, "\u226A", "\\ll", true);
  s(l, h, v, "\u226B", "\\gg", true);
  s(l, h, v, "\u224D", "\\asymp", true);
  s(l, h, v, "\u2225", "\\parallel");
  s(l, h, v, "\u22C8", "\\bowtie", true);
  s(l, h, v, "\u2323", "\\smile", true);
  s(l, h, v, "\u2291", "\\sqsubseteq", true);
  s(l, h, v, "\u2292", "\\sqsupseteq", true);
  s(l, h, v, "\u2250", "\\doteq", true);
  s(l, h, v, "\u2322", "\\frown", true);
  s(l, h, v, "\u220B", "\\ni", true);
  s(l, h, v, "\u221D", "\\propto", true);
  s(l, h, v, "\u22A2", "\\vdash", true);
  s(l, h, v, "\u22A3", "\\dashv", true);
  s(l, h, v, "\u220B", "\\owns");
  s(l, h, tt, ".", "\\ldotp");
  s(l, h, tt, "\u22C5", "\\cdotp");
  s(l, h, g, "#", "\\#");
  s(M, h, g, "#", "\\#");
  s(l, h, g, "&", "\\&");
  s(M, h, g, "&", "\\&");
  s(l, h, g, "\u2135", "\\aleph", true);
  s(l, h, g, "\u2200", "\\forall", true);
  s(l, h, g, "\u210F", "\\hbar", true);
  s(l, h, g, "\u2203", "\\exists", true);
  s(l, h, g, "\u2207", "\\nabla", true);
  s(l, h, g, "\u266D", "\\flat", true);
  s(l, h, g, "\u2113", "\\ell", true);
  s(l, h, g, "\u266E", "\\natural", true);
  s(l, h, g, "\u2663", "\\clubsuit", true);
  s(l, h, g, "\u2118", "\\wp", true);
  s(l, h, g, "\u266F", "\\sharp", true);
  s(l, h, g, "\u2662", "\\diamondsuit", true);
  s(l, h, g, "\u211C", "\\Re", true);
  s(l, h, g, "\u2661", "\\heartsuit", true);
  s(l, h, g, "\u2111", "\\Im", true);
  s(l, h, g, "\u2660", "\\spadesuit", true);
  s(l, h, g, "\xA7", "\\S", true);
  s(M, h, g, "\xA7", "\\S");
  s(l, h, g, "\xB6", "\\P", true);
  s(M, h, g, "\xB6", "\\P");
  s(l, h, g, "\u2020", "\\dag");
  s(M, h, g, "\u2020", "\\dag");
  s(M, h, g, "\u2020", "\\textdagger");
  s(l, h, g, "\u2021", "\\ddag");
  s(M, h, g, "\u2021", "\\ddag");
  s(M, h, g, "\u2021", "\\textdaggerdbl");
  s(l, h, l0, "\u23B1", "\\rmoustache", true);
  s(l, h, f0, "\u23B0", "\\lmoustache", true);
  s(l, h, l0, "\u27EF", "\\rgroup", true);
  s(l, h, f0, "\u27EE", "\\lgroup", true);
  s(l, h, q, "\u2213", "\\mp", true);
  s(l, h, q, "\u2296", "\\ominus", true);
  s(l, h, q, "\u228E", "\\uplus", true);
  s(l, h, q, "\u2293", "\\sqcap", true);
  s(l, h, q, "\u2217", "\\ast");
  s(l, h, q, "\u2294", "\\sqcup", true);
  s(l, h, q, "\u25EF", "\\bigcirc", true);
  s(l, h, q, "\u2219", "\\bullet", true);
  s(l, h, q, "\u2021", "\\ddagger");
  s(l, h, q, "\u2240", "\\wr", true);
  s(l, h, q, "\u2A3F", "\\amalg");
  s(l, h, q, "&", "\\And");
  s(l, h, v, "\u27F5", "\\longleftarrow", true);
  s(l, h, v, "\u21D0", "\\Leftarrow", true);
  s(l, h, v, "\u27F8", "\\Longleftarrow", true);
  s(l, h, v, "\u27F6", "\\longrightarrow", true);
  s(l, h, v, "\u21D2", "\\Rightarrow", true);
  s(l, h, v, "\u27F9", "\\Longrightarrow", true);
  s(l, h, v, "\u2194", "\\leftrightarrow", true);
  s(l, h, v, "\u27F7", "\\longleftrightarrow", true);
  s(l, h, v, "\u21D4", "\\Leftrightarrow", true);
  s(l, h, v, "\u27FA", "\\Longleftrightarrow", true);
  s(l, h, v, "\u21A6", "\\mapsto", true);
  s(l, h, v, "\u27FC", "\\longmapsto", true);
  s(l, h, v, "\u2197", "\\nearrow", true);
  s(l, h, v, "\u21A9", "\\hookleftarrow", true);
  s(l, h, v, "\u21AA", "\\hookrightarrow", true);
  s(l, h, v, "\u2198", "\\searrow", true);
  s(l, h, v, "\u21BC", "\\leftharpoonup", true);
  s(l, h, v, "\u21C0", "\\rightharpoonup", true);
  s(l, h, v, "\u2199", "\\swarrow", true);
  s(l, h, v, "\u21BD", "\\leftharpoondown", true);
  s(l, h, v, "\u21C1", "\\rightharpoondown", true);
  s(l, h, v, "\u2196", "\\nwarrow", true);
  s(l, h, v, "\u21CC", "\\rightleftharpoons", true);
  s(l, f, v, "\u226E", "\\nless", true);
  s(l, f, v, "\uE010", "\\@nleqslant");
  s(l, f, v, "\uE011", "\\@nleqq");
  s(l, f, v, "\u2A87", "\\lneq", true);
  s(l, f, v, "\u2268", "\\lneqq", true);
  s(l, f, v, "\uE00C", "\\@lvertneqq");
  s(l, f, v, "\u22E6", "\\lnsim", true);
  s(l, f, v, "\u2A89", "\\lnapprox", true);
  s(l, f, v, "\u2280", "\\nprec", true);
  s(l, f, v, "\u22E0", "\\npreceq", true);
  s(l, f, v, "\u22E8", "\\precnsim", true);
  s(l, f, v, "\u2AB9", "\\precnapprox", true);
  s(l, f, v, "\u2241", "\\nsim", true);
  s(l, f, v, "\uE006", "\\@nshortmid");
  s(l, f, v, "\u2224", "\\nmid", true);
  s(l, f, v, "\u22AC", "\\nvdash", true);
  s(l, f, v, "\u22AD", "\\nvDash", true);
  s(l, f, v, "\u22EA", "\\ntriangleleft");
  s(l, f, v, "\u22EC", "\\ntrianglelefteq", true);
  s(l, f, v, "\u228A", "\\subsetneq", true);
  s(l, f, v, "\uE01A", "\\@varsubsetneq");
  s(l, f, v, "\u2ACB", "\\subsetneqq", true);
  s(l, f, v, "\uE017", "\\@varsubsetneqq");
  s(l, f, v, "\u226F", "\\ngtr", true);
  s(l, f, v, "\uE00F", "\\@ngeqslant");
  s(l, f, v, "\uE00E", "\\@ngeqq");
  s(l, f, v, "\u2A88", "\\gneq", true);
  s(l, f, v, "\u2269", "\\gneqq", true);
  s(l, f, v, "\uE00D", "\\@gvertneqq");
  s(l, f, v, "\u22E7", "\\gnsim", true);
  s(l, f, v, "\u2A8A", "\\gnapprox", true);
  s(l, f, v, "\u2281", "\\nsucc", true);
  s(l, f, v, "\u22E1", "\\nsucceq", true);
  s(l, f, v, "\u22E9", "\\succnsim", true);
  s(l, f, v, "\u2ABA", "\\succnapprox", true);
  s(l, f, v, "\u2246", "\\ncong", true);
  s(l, f, v, "\uE007", "\\@nshortparallel");
  s(l, f, v, "\u2226", "\\nparallel", true);
  s(l, f, v, "\u22AF", "\\nVDash", true);
  s(l, f, v, "\u22EB", "\\ntriangleright");
  s(l, f, v, "\u22ED", "\\ntrianglerighteq", true);
  s(l, f, v, "\uE018", "\\@nsupseteqq");
  s(l, f, v, "\u228B", "\\supsetneq", true);
  s(l, f, v, "\uE01B", "\\@varsupsetneq");
  s(l, f, v, "\u2ACC", "\\supsetneqq", true);
  s(l, f, v, "\uE019", "\\@varsupsetneqq");
  s(l, f, v, "\u22AE", "\\nVdash", true);
  s(l, f, v, "\u2AB5", "\\precneqq", true);
  s(l, f, v, "\u2AB6", "\\succneqq", true);
  s(l, f, v, "\uE016", "\\@nsubseteqq");
  s(l, f, q, "\u22B4", "\\unlhd");
  s(l, f, q, "\u22B5", "\\unrhd");
  s(l, f, v, "\u219A", "\\nleftarrow", true);
  s(l, f, v, "\u219B", "\\nrightarrow", true);
  s(l, f, v, "\u21CD", "\\nLeftarrow", true);
  s(l, f, v, "\u21CF", "\\nRightarrow", true);
  s(l, f, v, "\u21AE", "\\nleftrightarrow", true);
  s(l, f, v, "\u21CE", "\\nLeftrightarrow", true);
  s(l, f, v, "\u25B3", "\\vartriangle");
  s(l, f, g, "\u210F", "\\hslash");
  s(l, f, g, "\u25BD", "\\triangledown");
  s(l, f, g, "\u25CA", "\\lozenge");
  s(l, f, g, "\u24C8", "\\circledS");
  s(l, f, g, "\xAE", "\\circledR");
  s(M, f, g, "\xAE", "\\circledR");
  s(l, f, g, "\u2221", "\\measuredangle", true);
  s(l, f, g, "\u2204", "\\nexists");
  s(l, f, g, "\u2127", "\\mho");
  s(l, f, g, "\u2132", "\\Finv", true);
  s(l, f, g, "\u2141", "\\Game", true);
  s(l, f, g, "\u2035", "\\backprime");
  s(l, f, g, "\u25B2", "\\blacktriangle");
  s(l, f, g, "\u25BC", "\\blacktriangledown");
  s(l, f, g, "\u25A0", "\\blacksquare");
  s(l, f, g, "\u29EB", "\\blacklozenge");
  s(l, f, g, "\u2605", "\\bigstar");
  s(l, f, g, "\u2222", "\\sphericalangle", true);
  s(l, f, g, "\u2201", "\\complement", true);
  s(l, f, g, "\xF0", "\\eth", true);
  s(M, h, g, "\xF0", "\xF0");
  s(l, f, g, "\u2571", "\\diagup");
  s(l, f, g, "\u2572", "\\diagdown");
  s(l, f, g, "\u25A1", "\\square");
  s(l, f, g, "\u25A1", "\\Box");
  s(l, f, g, "\u25CA", "\\Diamond");
  s(l, f, g, "\xA5", "\\yen", true);
  s(M, f, g, "\xA5", "\\yen", true);
  s(l, f, g, "\u2713", "\\checkmark", true);
  s(M, f, g, "\u2713", "\\checkmark");
  s(l, f, g, "\u2136", "\\beth", true);
  s(l, f, g, "\u2138", "\\daleth", true);
  s(l, f, g, "\u2137", "\\gimel", true);
  s(l, f, g, "\u03DD", "\\digamma", true);
  s(l, f, g, "\u03F0", "\\varkappa");
  s(l, f, f0, "\u250C", "\\@ulcorner", true);
  s(l, f, l0, "\u2510", "\\@urcorner", true);
  s(l, f, f0, "\u2514", "\\@llcorner", true);
  s(l, f, l0, "\u2518", "\\@lrcorner", true);
  s(l, f, v, "\u2266", "\\leqq", true);
  s(l, f, v, "\u2A7D", "\\leqslant", true);
  s(l, f, v, "\u2A95", "\\eqslantless", true);
  s(l, f, v, "\u2272", "\\lesssim", true);
  s(l, f, v, "\u2A85", "\\lessapprox", true);
  s(l, f, v, "\u224A", "\\approxeq", true);
  s(l, f, q, "\u22D6", "\\lessdot");
  s(l, f, v, "\u22D8", "\\lll", true);
  s(l, f, v, "\u2276", "\\lessgtr", true);
  s(l, f, v, "\u22DA", "\\lesseqgtr", true);
  s(l, f, v, "\u2A8B", "\\lesseqqgtr", true);
  s(l, f, v, "\u2251", "\\doteqdot");
  s(l, f, v, "\u2253", "\\risingdotseq", true);
  s(l, f, v, "\u2252", "\\fallingdotseq", true);
  s(l, f, v, "\u223D", "\\backsim", true);
  s(l, f, v, "\u22CD", "\\backsimeq", true);
  s(l, f, v, "\u2AC5", "\\subseteqq", true);
  s(l, f, v, "\u22D0", "\\Subset", true);
  s(l, f, v, "\u228F", "\\sqsubset", true);
  s(l, f, v, "\u227C", "\\preccurlyeq", true);
  s(l, f, v, "\u22DE", "\\curlyeqprec", true);
  s(l, f, v, "\u227E", "\\precsim", true);
  s(l, f, v, "\u2AB7", "\\precapprox", true);
  s(l, f, v, "\u22B2", "\\vartriangleleft");
  s(l, f, v, "\u22B4", "\\trianglelefteq");
  s(l, f, v, "\u22A8", "\\vDash", true);
  s(l, f, v, "\u22AA", "\\Vvdash", true);
  s(l, f, v, "\u2323", "\\smallsmile");
  s(l, f, v, "\u2322", "\\smallfrown");
  s(l, f, v, "\u224F", "\\bumpeq", true);
  s(l, f, v, "\u224E", "\\Bumpeq", true);
  s(l, f, v, "\u2267", "\\geqq", true);
  s(l, f, v, "\u2A7E", "\\geqslant", true);
  s(l, f, v, "\u2A96", "\\eqslantgtr", true);
  s(l, f, v, "\u2273", "\\gtrsim", true);
  s(l, f, v, "\u2A86", "\\gtrapprox", true);
  s(l, f, q, "\u22D7", "\\gtrdot");
  s(l, f, v, "\u22D9", "\\ggg", true);
  s(l, f, v, "\u2277", "\\gtrless", true);
  s(l, f, v, "\u22DB", "\\gtreqless", true);
  s(l, f, v, "\u2A8C", "\\gtreqqless", true);
  s(l, f, v, "\u2256", "\\eqcirc", true);
  s(l, f, v, "\u2257", "\\circeq", true);
  s(l, f, v, "\u225C", "\\triangleq", true);
  s(l, f, v, "\u223C", "\\thicksim");
  s(l, f, v, "\u2248", "\\thickapprox");
  s(l, f, v, "\u2AC6", "\\supseteqq", true);
  s(l, f, v, "\u22D1", "\\Supset", true);
  s(l, f, v, "\u2290", "\\sqsupset", true);
  s(l, f, v, "\u227D", "\\succcurlyeq", true);
  s(l, f, v, "\u22DF", "\\curlyeqsucc", true);
  s(l, f, v, "\u227F", "\\succsim", true);
  s(l, f, v, "\u2AB8", "\\succapprox", true);
  s(l, f, v, "\u22B3", "\\vartriangleright");
  s(l, f, v, "\u22B5", "\\trianglerighteq");
  s(l, f, v, "\u22A9", "\\Vdash", true);
  s(l, f, v, "\u2223", "\\shortmid");
  s(l, f, v, "\u2225", "\\shortparallel");
  s(l, f, v, "\u226C", "\\between", true);
  s(l, f, v, "\u22D4", "\\pitchfork", true);
  s(l, f, v, "\u221D", "\\varpropto");
  s(l, f, v, "\u25C0", "\\blacktriangleleft");
  s(l, f, v, "\u2234", "\\therefore", true);
  s(l, f, v, "\u220D", "\\backepsilon");
  s(l, f, v, "\u25B6", "\\blacktriangleright");
  s(l, f, v, "\u2235", "\\because", true);
  s(l, f, v, "\u22D8", "\\llless");
  s(l, f, v, "\u22D9", "\\gggtr");
  s(l, f, q, "\u22B2", "\\lhd");
  s(l, f, q, "\u22B3", "\\rhd");
  s(l, f, v, "\u2242", "\\eqsim", true);
  s(l, h, v, "\u22C8", "\\Join");
  s(l, f, v, "\u2251", "\\Doteq", true);
  s(l, f, q, "\u2214", "\\dotplus", true);
  s(l, f, q, "\u2216", "\\smallsetminus");
  s(l, f, q, "\u22D2", "\\Cap", true);
  s(l, f, q, "\u22D3", "\\Cup", true);
  s(l, f, q, "\u2A5E", "\\doublebarwedge", true);
  s(l, f, q, "\u229F", "\\boxminus", true);
  s(l, f, q, "\u229E", "\\boxplus", true);
  s(l, f, q, "\u22C7", "\\divideontimes", true);
  s(l, f, q, "\u22C9", "\\ltimes", true);
  s(l, f, q, "\u22CA", "\\rtimes", true);
  s(l, f, q, "\u22CB", "\\leftthreetimes", true);
  s(l, f, q, "\u22CC", "\\rightthreetimes", true);
  s(l, f, q, "\u22CF", "\\curlywedge", true);
  s(l, f, q, "\u22CE", "\\curlyvee", true);
  s(l, f, q, "\u229D", "\\circleddash", true);
  s(l, f, q, "\u229B", "\\circledast", true);
  s(l, f, q, "\u22C5", "\\centerdot");
  s(l, f, q, "\u22BA", "\\intercal", true);
  s(l, f, q, "\u22D2", "\\doublecap");
  s(l, f, q, "\u22D3", "\\doublecup");
  s(l, f, q, "\u22A0", "\\boxtimes", true);
  s(l, f, v, "\u21E2", "\\dashrightarrow", true);
  s(l, f, v, "\u21E0", "\\dashleftarrow", true);
  s(l, f, v, "\u21C7", "\\leftleftarrows", true);
  s(l, f, v, "\u21C6", "\\leftrightarrows", true);
  s(l, f, v, "\u21DA", "\\Lleftarrow", true);
  s(l, f, v, "\u219E", "\\twoheadleftarrow", true);
  s(l, f, v, "\u21A2", "\\leftarrowtail", true);
  s(l, f, v, "\u21AB", "\\looparrowleft", true);
  s(l, f, v, "\u21CB", "\\leftrightharpoons", true);
  s(l, f, v, "\u21B6", "\\curvearrowleft", true);
  s(l, f, v, "\u21BA", "\\circlearrowleft", true);
  s(l, f, v, "\u21B0", "\\Lsh", true);
  s(l, f, v, "\u21C8", "\\upuparrows", true);
  s(l, f, v, "\u21BF", "\\upharpoonleft", true);
  s(l, f, v, "\u21C3", "\\downharpoonleft", true);
  s(l, h, v, "\u22B6", "\\origof", true);
  s(l, h, v, "\u22B7", "\\imageof", true);
  s(l, f, v, "\u22B8", "\\multimap", true);
  s(l, f, v, "\u21AD", "\\leftrightsquigarrow", true);
  s(l, f, v, "\u21C9", "\\rightrightarrows", true);
  s(l, f, v, "\u21C4", "\\rightleftarrows", true);
  s(l, f, v, "\u21A0", "\\twoheadrightarrow", true);
  s(l, f, v, "\u21A3", "\\rightarrowtail", true);
  s(l, f, v, "\u21AC", "\\looparrowright", true);
  s(l, f, v, "\u21B7", "\\curvearrowright", true);
  s(l, f, v, "\u21BB", "\\circlearrowright", true);
  s(l, f, v, "\u21B1", "\\Rsh", true);
  s(l, f, v, "\u21CA", "\\downdownarrows", true);
  s(l, f, v, "\u21BE", "\\upharpoonright", true);
  s(l, f, v, "\u21C2", "\\downharpoonright", true);
  s(l, f, v, "\u21DD", "\\rightsquigarrow", true);
  s(l, f, v, "\u21DD", "\\leadsto");
  s(l, f, v, "\u21DB", "\\Rrightarrow", true);
  s(l, f, v, "\u21BE", "\\restriction");
  s(l, h, g, "\u2018", "`");
  s(l, h, g, "$", "\\$");
  s(M, h, g, "$", "\\$");
  s(M, h, g, "$", "\\textdollar");
  s(l, h, g, "%", "\\%");
  s(M, h, g, "%", "\\%");
  s(l, h, g, "_", "\\_");
  s(M, h, g, "_", "\\_");
  s(M, h, g, "_", "\\textunderscore");
  s(l, h, g, "\u2220", "\\angle", true);
  s(l, h, g, "\u221E", "\\infty", true);
  s(l, h, g, "\u2032", "\\prime");
  s(l, h, g, "\u25B3", "\\triangle");
  s(l, h, g, "\u0393", "\\Gamma", true);
  s(l, h, g, "\u0394", "\\Delta", true);
  s(l, h, g, "\u0398", "\\Theta", true);
  s(l, h, g, "\u039B", "\\Lambda", true);
  s(l, h, g, "\u039E", "\\Xi", true);
  s(l, h, g, "\u03A0", "\\Pi", true);
  s(l, h, g, "\u03A3", "\\Sigma", true);
  s(l, h, g, "\u03A5", "\\Upsilon", true);
  s(l, h, g, "\u03A6", "\\Phi", true);
  s(l, h, g, "\u03A8", "\\Psi", true);
  s(l, h, g, "\u03A9", "\\Omega", true);
  s(l, h, g, "A", "\u0391");
  s(l, h, g, "B", "\u0392");
  s(l, h, g, "E", "\u0395");
  s(l, h, g, "Z", "\u0396");
  s(l, h, g, "H", "\u0397");
  s(l, h, g, "I", "\u0399");
  s(l, h, g, "K", "\u039A");
  s(l, h, g, "M", "\u039C");
  s(l, h, g, "N", "\u039D");
  s(l, h, g, "O", "\u039F");
  s(l, h, g, "P", "\u03A1");
  s(l, h, g, "T", "\u03A4");
  s(l, h, g, "X", "\u03A7");
  s(l, h, g, "\xAC", "\\neg", true);
  s(l, h, g, "\xAC", "\\lnot");
  s(l, h, g, "\u22A4", "\\top");
  s(l, h, g, "\u22A5", "\\bot");
  s(l, h, g, "\u2205", "\\emptyset");
  s(l, f, g, "\u2205", "\\varnothing");
  s(l, h, R, "\u03B1", "\\alpha", true);
  s(l, h, R, "\u03B2", "\\beta", true);
  s(l, h, R, "\u03B3", "\\gamma", true);
  s(l, h, R, "\u03B4", "\\delta", true);
  s(l, h, R, "\u03F5", "\\epsilon", true);
  s(l, h, R, "\u03B6", "\\zeta", true);
  s(l, h, R, "\u03B7", "\\eta", true);
  s(l, h, R, "\u03B8", "\\theta", true);
  s(l, h, R, "\u03B9", "\\iota", true);
  s(l, h, R, "\u03BA", "\\kappa", true);
  s(l, h, R, "\u03BB", "\\lambda", true);
  s(l, h, R, "\u03BC", "\\mu", true);
  s(l, h, R, "\u03BD", "\\nu", true);
  s(l, h, R, "\u03BE", "\\xi", true);
  s(l, h, R, "\u03BF", "\\omicron", true);
  s(l, h, R, "\u03C0", "\\pi", true);
  s(l, h, R, "\u03C1", "\\rho", true);
  s(l, h, R, "\u03C3", "\\sigma", true);
  s(l, h, R, "\u03C4", "\\tau", true);
  s(l, h, R, "\u03C5", "\\upsilon", true);
  s(l, h, R, "\u03D5", "\\phi", true);
  s(l, h, R, "\u03C7", "\\chi", true);
  s(l, h, R, "\u03C8", "\\psi", true);
  s(l, h, R, "\u03C9", "\\omega", true);
  s(l, h, R, "\u03B5", "\\varepsilon", true);
  s(l, h, R, "\u03D1", "\\vartheta", true);
  s(l, h, R, "\u03D6", "\\varpi", true);
  s(l, h, R, "\u03F1", "\\varrho", true);
  s(l, h, R, "\u03C2", "\\varsigma", true);
  s(l, h, R, "\u03C6", "\\varphi", true);
  s(l, h, q, "\u2217", "*", true);
  s(l, h, q, "+", "+");
  s(l, h, q, "\u2212", "-", true);
  s(l, h, q, "\u22C5", "\\cdot", true);
  s(l, h, q, "\u2218", "\\circ", true);
  s(l, h, q, "\xF7", "\\div", true);
  s(l, h, q, "\xB1", "\\pm", true);
  s(l, h, q, "\xD7", "\\times", true);
  s(l, h, q, "\u2229", "\\cap", true);
  s(l, h, q, "\u222A", "\\cup", true);
  s(l, h, q, "\u2216", "\\setminus", true);
  s(l, h, q, "\u2227", "\\land");
  s(l, h, q, "\u2228", "\\lor");
  s(l, h, q, "\u2227", "\\wedge", true);
  s(l, h, q, "\u2228", "\\vee", true);
  s(l, h, g, "\u221A", "\\surd");
  s(l, h, f0, "\u27E8", "\\langle", true);
  s(l, h, f0, "\u2223", "\\lvert");
  s(l, h, f0, "\u2225", "\\lVert");
  s(l, h, l0, "?", "?");
  s(l, h, l0, "!", "!");
  s(l, h, l0, "\u27E9", "\\rangle", true);
  s(l, h, l0, "\u2223", "\\rvert");
  s(l, h, l0, "\u2225", "\\rVert");
  s(l, h, v, "=", "=");
  s(l, h, v, ":", ":");
  s(l, h, v, "\u2248", "\\approx", true);
  s(l, h, v, "\u2245", "\\cong", true);
  s(l, h, v, "\u2265", "\\ge");
  s(l, h, v, "\u2265", "\\geq", true);
  s(l, h, v, "\u2190", "\\gets");
  s(l, h, v, ">", "\\gt", true);
  s(l, h, v, "\u2208", "\\in", true);
  s(l, h, v, "\uE020", "\\@not");
  s(l, h, v, "\u2282", "\\subset", true);
  s(l, h, v, "\u2283", "\\supset", true);
  s(l, h, v, "\u2286", "\\subseteq", true);
  s(l, h, v, "\u2287", "\\supseteq", true);
  s(l, f, v, "\u2288", "\\nsubseteq", true);
  s(l, f, v, "\u2289", "\\nsupseteq", true);
  s(l, h, v, "\u22A8", "\\models");
  s(l, h, v, "\u2190", "\\leftarrow", true);
  s(l, h, v, "\u2264", "\\le");
  s(l, h, v, "\u2264", "\\leq", true);
  s(l, h, v, "<", "\\lt", true);
  s(l, h, v, "\u2192", "\\rightarrow", true);
  s(l, h, v, "\u2192", "\\to");
  s(l, f, v, "\u2271", "\\ngeq", true);
  s(l, f, v, "\u2270", "\\nleq", true);
  s(l, h, V0, "\xA0", "\\ ");
  s(l, h, V0, "\xA0", "\\space");
  s(l, h, V0, "\xA0", "\\nobreakspace");
  s(M, h, V0, "\xA0", "\\ ");
  s(M, h, V0, "\xA0", " ");
  s(M, h, V0, "\xA0", "\\space");
  s(M, h, V0, "\xA0", "\\nobreakspace");
  s(l, h, V0, null, "\\nobreak");
  s(l, h, V0, null, "\\allowbreak");
  s(l, h, tt, ",", ",");
  s(l, h, tt, ";", ";");
  s(l, f, q, "\u22BC", "\\barwedge", true);
  s(l, f, q, "\u22BB", "\\veebar", true);
  s(l, h, q, "\u2299", "\\odot", true);
  s(l, h, q, "\u2295", "\\oplus", true);
  s(l, h, q, "\u2297", "\\otimes", true);
  s(l, h, g, "\u2202", "\\partial", true);
  s(l, h, q, "\u2298", "\\oslash", true);
  s(l, f, q, "\u229A", "\\circledcirc", true);
  s(l, f, q, "\u22A1", "\\boxdot", true);
  s(l, h, q, "\u25B3", "\\bigtriangleup");
  s(l, h, q, "\u25BD", "\\bigtriangledown");
  s(l, h, q, "\u2020", "\\dagger");
  s(l, h, q, "\u22C4", "\\diamond");
  s(l, h, q, "\u22C6", "\\star");
  s(l, h, q, "\u25C3", "\\triangleleft");
  s(l, h, q, "\u25B9", "\\triangleright");
  s(l, h, f0, "{", "\\{");
  s(M, h, g, "{", "\\{");
  s(M, h, g, "{", "\\textbraceleft");
  s(l, h, l0, "}", "\\}");
  s(M, h, g, "}", "\\}");
  s(M, h, g, "}", "\\textbraceright");
  s(l, h, f0, "{", "\\lbrace");
  s(l, h, l0, "}", "\\rbrace");
  s(l, h, f0, "[", "\\lbrack", true);
  s(M, h, g, "[", "\\lbrack", true);
  s(l, h, l0, "]", "\\rbrack", true);
  s(M, h, g, "]", "\\rbrack", true);
  s(l, h, f0, "(", "\\lparen", true);
  s(l, h, l0, ")", "\\rparen", true);
  s(M, h, g, "<", "\\textless", true);
  s(M, h, g, ">", "\\textgreater", true);
  s(l, h, f0, "\u230A", "\\lfloor", true);
  s(l, h, l0, "\u230B", "\\rfloor", true);
  s(l, h, f0, "\u2308", "\\lceil", true);
  s(l, h, l0, "\u2309", "\\rceil", true);
  s(l, h, g, "\\", "\\backslash");
  s(l, h, g, "\u2223", "|");
  s(l, h, g, "\u2223", "\\vert");
  s(M, h, g, "|", "\\textbar", true);
  s(l, h, g, "\u2225", "\\|");
  s(l, h, g, "\u2225", "\\Vert");
  s(M, h, g, "\u2225", "\\textbardbl");
  s(M, h, g, "~", "\\textasciitilde");
  s(M, h, g, "\\", "\\textbackslash");
  s(M, h, g, "^", "\\textasciicircum");
  s(l, h, v, "\u2191", "\\uparrow", true);
  s(l, h, v, "\u21D1", "\\Uparrow", true);
  s(l, h, v, "\u2193", "\\downarrow", true);
  s(l, h, v, "\u21D3", "\\Downarrow", true);
  s(l, h, v, "\u2195", "\\updownarrow", true);
  s(l, h, v, "\u21D5", "\\Updownarrow", true);
  s(l, h, Q, "\u2210", "\\coprod");
  s(l, h, Q, "\u22C1", "\\bigvee");
  s(l, h, Q, "\u22C0", "\\bigwedge");
  s(l, h, Q, "\u2A04", "\\biguplus");
  s(l, h, Q, "\u22C2", "\\bigcap");
  s(l, h, Q, "\u22C3", "\\bigcup");
  s(l, h, Q, "\u222B", "\\int");
  s(l, h, Q, "\u222B", "\\intop");
  s(l, h, Q, "\u222C", "\\iint");
  s(l, h, Q, "\u222D", "\\iiint");
  s(l, h, Q, "\u220F", "\\prod");
  s(l, h, Q, "\u2211", "\\sum");
  s(l, h, Q, "\u2A02", "\\bigotimes");
  s(l, h, Q, "\u2A01", "\\bigoplus");
  s(l, h, Q, "\u2A00", "\\bigodot");
  s(l, h, Q, "\u222E", "\\oint");
  s(l, h, Q, "\u222F", "\\oiint");
  s(l, h, Q, "\u2230", "\\oiiint");
  s(l, h, Q, "\u2A06", "\\bigsqcup");
  s(l, h, Q, "\u222B", "\\smallint");
  s(M, h, pe, "\u2026", "\\textellipsis");
  s(l, h, pe, "\u2026", "\\mathellipsis");
  s(M, h, pe, "\u2026", "\\ldots", true);
  s(l, h, pe, "\u2026", "\\ldots", true);
  s(l, h, pe, "\u22EF", "\\@cdots", true);
  s(l, h, pe, "\u22F1", "\\ddots", true);
  s(l, h, g, "\u22EE", "\\varvdots");
  s(M, h, g, "\u22EE", "\\varvdots");
  s(l, h, Z, "\u02CA", "\\acute");
  s(l, h, Z, "\u02CB", "\\grave");
  s(l, h, Z, "\xA8", "\\ddot");
  s(l, h, Z, "~", "\\tilde");
  s(l, h, Z, "\u02C9", "\\bar");
  s(l, h, Z, "\u02D8", "\\breve");
  s(l, h, Z, "\u02C7", "\\check");
  s(l, h, Z, "^", "\\hat");
  s(l, h, Z, "\u20D7", "\\vec");
  s(l, h, Z, "\u02D9", "\\dot");
  s(l, h, Z, "\u02DA", "\\mathring");
  s(l, h, R, "\uE131", "\\@imath");
  s(l, h, R, "\uE237", "\\@jmath");
  s(l, h, g, "\u0131", "\u0131");
  s(l, h, g, "\u0237", "\u0237");
  s(M, h, g, "\u0131", "\\i", true);
  s(M, h, g, "\u0237", "\\j", true);
  s(M, h, g, "\xDF", "\\ss", true);
  s(M, h, g, "\xE6", "\\ae", true);
  s(M, h, g, "\u0153", "\\oe", true);
  s(M, h, g, "\xF8", "\\o", true);
  s(M, h, g, "\xC6", "\\AE", true);
  s(M, h, g, "\u0152", "\\OE", true);
  s(M, h, g, "\xD8", "\\O", true);
  s(M, h, Z, "\u02CA", "\\'");
  s(M, h, Z, "\u02CB", "\\`");
  s(M, h, Z, "\u02C6", "\\^");
  s(M, h, Z, "\u02DC", "\\~");
  s(M, h, Z, "\u02C9", "\\=");
  s(M, h, Z, "\u02D8", "\\u");
  s(M, h, Z, "\u02D9", "\\.");
  s(M, h, Z, "\xB8", "\\c");
  s(M, h, Z, "\u02DA", "\\r");
  s(M, h, Z, "\u02C7", "\\v");
  s(M, h, Z, "\xA8", '\\"');
  s(M, h, Z, "\u02DD", "\\H");
  s(M, h, Z, "\u25EF", "\\textcircled");
  var Ta = {
    "--": true,
    "---": true,
    "``": true,
    "''": true
  };
  s(M, h, g, "\u2013", "--", true);
  s(M, h, g, "\u2013", "\\textendash");
  s(M, h, g, "\u2014", "---", true);
  s(M, h, g, "\u2014", "\\textemdash");
  s(M, h, g, "\u2018", "`", true);
  s(M, h, g, "\u2018", "\\textquoteleft");
  s(M, h, g, "\u2019", "'", true);
  s(M, h, g, "\u2019", "\\textquoteright");
  s(M, h, g, "\u201C", "``", true);
  s(M, h, g, "\u201C", "\\textquotedblleft");
  s(M, h, g, "\u201D", "''", true);
  s(M, h, g, "\u201D", "\\textquotedblright");
  s(l, h, g, "\xB0", "\\degree", true);
  s(M, h, g, "\xB0", "\\degree");
  s(M, h, g, "\xB0", "\\textdegree", true);
  s(l, h, g, "\xA3", "\\pounds");
  s(l, h, g, "\xA3", "\\mathsterling", true);
  s(M, h, g, "\xA3", "\\pounds");
  s(M, h, g, "\xA3", "\\textsterling", true);
  s(l, f, g, "\u2720", "\\maltese");
  s(M, f, g, "\u2720", "\\maltese");
  var Nr = '0123456789/@."';
  for (var bt = 0; bt < Nr.length; bt++) {
    var qr = Nr.charAt(bt);
    s(l, h, g, qr, qr);
  }
  var Er = '0123456789!@*()-=+";:?/.,';
  for (var yt = 0; yt < Er.length; yt++) {
    var Rr = Er.charAt(yt);
    s(M, h, g, Rr, Rr);
  }
  var je = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
  for (var xt = 0; xt < je.length; xt++) {
    var Ie = je.charAt(xt);
    s(l, h, R, Ie, Ie), s(M, h, g, Ie, Ie);
  }
  s(l, f, g, "C", "\u2102");
  s(M, f, g, "C", "\u2102");
  s(l, f, g, "H", "\u210D");
  s(M, f, g, "H", "\u210D");
  s(l, f, g, "N", "\u2115");
  s(M, f, g, "N", "\u2115");
  s(l, f, g, "P", "\u2119");
  s(M, f, g, "P", "\u2119");
  s(l, f, g, "Q", "\u211A");
  s(M, f, g, "Q", "\u211A");
  s(l, f, g, "R", "\u211D");
  s(M, f, g, "R", "\u211D");
  s(l, f, g, "Z", "\u2124");
  s(M, f, g, "Z", "\u2124");
  s(l, h, R, "h", "\u210E");
  s(M, h, R, "h", "\u210E");
  var F = "";
  for (var n0 = 0; n0 < je.length; n0++) {
    var _ = je.charAt(n0);
    F = String.fromCharCode(55349, 56320 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56372 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56424 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56580 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56684 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56736 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56788 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56840 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56944 + n0), s(l, h, R, _, F), s(M, h, g, _, F), n0 < 26 && (F = String.fromCharCode(55349, 56632 + n0), s(l, h, R, _, F), s(M, h, g, _, F), F = String.fromCharCode(55349, 56476 + n0), s(l, h, R, _, F), s(M, h, g, _, F));
  }
  F = "\u{1D55C}";
  s(l, h, R, "k", F);
  s(M, h, g, "k", F);
  for (var te = 0; te < 10; te++) {
    var W0 = te.toString();
    F = String.fromCharCode(55349, 57294 + te), s(l, h, R, W0, F), s(M, h, g, W0, F), F = String.fromCharCode(55349, 57314 + te), s(l, h, R, W0, F), s(M, h, g, W0, F), F = String.fromCharCode(55349, 57324 + te), s(l, h, R, W0, F), s(M, h, g, W0, F), F = String.fromCharCode(55349, 57334 + te), s(l, h, R, W0, F), s(M, h, g, W0, F);
  }
  var Vt = "\xD0\xDE\xFE";
  for (var wt = 0; wt < Vt.length; wt++) {
    var Oe = Vt.charAt(wt);
    s(l, h, R, Oe, Oe), s(M, h, g, Oe, Oe);
  }
  var Fe = [
    [
      "mathbf",
      "textbf",
      "Main-Bold"
    ],
    [
      "mathbf",
      "textbf",
      "Main-Bold"
    ],
    [
      "mathnormal",
      "textit",
      "Math-Italic"
    ],
    [
      "mathnormal",
      "textit",
      "Math-Italic"
    ],
    [
      "boldsymbol",
      "boldsymbol",
      "Main-BoldItalic"
    ],
    [
      "boldsymbol",
      "boldsymbol",
      "Main-BoldItalic"
    ],
    [
      "mathscr",
      "textscr",
      "Script-Regular"
    ],
    [
      "",
      "",
      ""
    ],
    [
      "",
      "",
      ""
    ],
    [
      "",
      "",
      ""
    ],
    [
      "mathfrak",
      "textfrak",
      "Fraktur-Regular"
    ],
    [
      "mathfrak",
      "textfrak",
      "Fraktur-Regular"
    ],
    [
      "mathbb",
      "textbb",
      "AMS-Regular"
    ],
    [
      "mathbb",
      "textbb",
      "AMS-Regular"
    ],
    [
      "mathboldfrak",
      "textboldfrak",
      "Fraktur-Regular"
    ],
    [
      "mathboldfrak",
      "textboldfrak",
      "Fraktur-Regular"
    ],
    [
      "mathsf",
      "textsf",
      "SansSerif-Regular"
    ],
    [
      "mathsf",
      "textsf",
      "SansSerif-Regular"
    ],
    [
      "mathboldsf",
      "textboldsf",
      "SansSerif-Bold"
    ],
    [
      "mathboldsf",
      "textboldsf",
      "SansSerif-Bold"
    ],
    [
      "mathitsf",
      "textitsf",
      "SansSerif-Italic"
    ],
    [
      "mathitsf",
      "textitsf",
      "SansSerif-Italic"
    ],
    [
      "",
      "",
      ""
    ],
    [
      "",
      "",
      ""
    ],
    [
      "mathtt",
      "texttt",
      "Typewriter-Regular"
    ],
    [
      "mathtt",
      "texttt",
      "Typewriter-Regular"
    ]
  ], Ir = [
    [
      "mathbf",
      "textbf",
      "Main-Bold"
    ],
    [
      "",
      "",
      ""
    ],
    [
      "mathsf",
      "textsf",
      "SansSerif-Regular"
    ],
    [
      "mathboldsf",
      "textboldsf",
      "SansSerif-Bold"
    ],
    [
      "mathtt",
      "texttt",
      "Typewriter-Regular"
    ]
  ], ci = function(e, t) {
    var a = e.charCodeAt(0), n = e.charCodeAt(1), i = (a - 55296) * 1024 + (n - 56320) + 65536, o = t === "math" ? 0 : 1;
    if (119808 <= i && i < 120484) {
      var u = Math.floor((i - 119808) / 26);
      return [
        Fe[u][2],
        Fe[u][o]
      ];
    } else if (120782 <= i && i <= 120831) {
      var m = Math.floor((i - 120782) / 10);
      return [
        Ir[m][2],
        Ir[m][o]
      ];
    } else {
      if (i === 120485 || i === 120486) return [
        Fe[0][2],
        Fe[0][o]
      ];
      if (120486 < i && i < 120782) return [
        "",
        ""
      ];
      throw new T("Unsupported character: " + e);
    }
  }, rt = function(e, t, a) {
    return W[a][e] && W[a][e].replace && (e = W[a][e].replace), {
      value: e,
      metrics: _t(e, t, a)
    };
  }, k0 = function(e, t, a, n, i) {
    var o = rt(e, t, a), u = o.metrics;
    e = o.value;
    var m;
    if (u) {
      var d = u.italic;
      (a === "text" || n && n.font === "mathit") && (d = 0), m = new b0(e, u.height, u.depth, d, u.skew, u.width, i);
    } else typeof console < "u" && console.warn("No character metrics " + ("for '" + e + "' in style '" + t + "' and mode '" + a + "'")), m = new b0(e, 0, 0, 0, 0, 0, i);
    if (n) {
      m.maxFontSize = n.sizeMultiplier, n.style.isTight() && m.classes.push("mtight");
      var p = n.getColor();
      p && (m.style.color = p);
    }
    return m;
  }, di = function(e, t, a, n) {
    return n === void 0 && (n = []), a.font === "boldsymbol" && rt(e, "Main-Bold", t).metrics ? k0(e, "Main-Bold", t, a, n.concat([
      "mathbf"
    ])) : e === "\\" || W[t][e].font === "main" ? k0(e, "Main-Regular", t, a, n) : k0(e, "AMS-Regular", t, a, n.concat([
      "amsrm"
    ]));
  }, fi = function(e, t, a, n, i) {
    return i !== "textord" && rt(e, "Math-BoldItalic", t).metrics ? {
      fontName: "Math-BoldItalic",
      fontClass: "boldsymbol"
    } : {
      fontName: "Main-Bold",
      fontClass: "mathbf"
    };
  }, vi = function(e, t, a) {
    var n = e.mode, i = e.text, o = [
      "mord"
    ], u = n === "math" || n === "text" && t.font, m = u ? t.font : t.fontFamily, d = "", p = "";
    if (i.charCodeAt(0) === 55349 && ([d, p] = ci(i, n)), d.length > 0) return k0(i, d, n, t, o.concat(p));
    if (m) {
      var b, y;
      if (m === "boldsymbol") {
        var x = fi(i, n, t, o, a);
        b = x.fontName, y = [
          x.fontClass
        ];
      } else u ? (b = Da[m].fontName, y = [
        m
      ]) : (b = He(m, t.fontWeight, t.fontShape), y = [
        m,
        t.fontWeight,
        t.fontShape
      ]);
      if (rt(i, b, n).metrics) return k0(i, b, n, t, o.concat(y));
      if (Ta.hasOwnProperty(i) && b.slice(0, 10) === "Typewriter") {
        for (var k = [], S = 0; S < i.length; S++) k.push(k0(i[S], b, n, t, o.concat(y)));
        return Ca(k);
      }
    }
    if (a === "mathord") return k0(i, "Math-Italic", n, t, o.concat([
      "mathnormal"
    ]));
    if (a === "textord") {
      var z = W[n][i] && W[n][i].font;
      if (z === "ams") {
        var B = He("amsrm", t.fontWeight, t.fontShape);
        return k0(i, B, n, t, o.concat("amsrm", t.fontWeight, t.fontShape));
      } else if (z === "main" || !z) {
        var N = He("textrm", t.fontWeight, t.fontShape);
        return k0(i, N, n, t, o.concat(t.fontWeight, t.fontShape));
      } else {
        var E = He(z, t.fontWeight, t.fontShape);
        return k0(i, E, n, t, o.concat(E, t.fontWeight, t.fontShape));
      }
    } else throw new Error("unexpected type: " + a + " in makeOrd");
  }, pi = (r, e) => {
    if (K0(r.classes) !== K0(e.classes) || r.skew !== e.skew || r.maxFontSize !== e.maxFontSize) return false;
    if (r.classes.length === 1) {
      var t = r.classes[0];
      if (t === "mbin" || t === "mord") return false;
    }
    for (var a in r.style) if (r.style.hasOwnProperty(a) && r.style[a] !== e.style[a]) return false;
    for (var n in e.style) if (e.style.hasOwnProperty(n) && r.style[n] !== e.style[n]) return false;
    return true;
  }, gi = (r) => {
    for (var e = 0; e < r.length - 1; e++) {
      var t = r[e], a = r[e + 1];
      t instanceof b0 && a instanceof b0 && pi(t, a) && (t.text += a.text, t.height = Math.max(t.height, a.height), t.depth = Math.max(t.depth, a.depth), t.italic = a.italic, r.splice(e + 1, 1), e--);
    }
    return r;
  }, Qt = function(e) {
    for (var t = 0, a = 0, n = 0, i = 0; i < e.children.length; i++) {
      var o = e.children[i];
      o.height > t && (t = o.height), o.depth > a && (a = o.depth), o.maxFontSize > n && (n = o.maxFontSize);
    }
    e.height = t, e.depth = a, e.maxFontSize = n;
  }, u0 = function(e, t, a, n) {
    var i = new Be(e, t, a, n);
    return Qt(i), i;
  }, Ba = (r, e, t, a) => new Be(r, e, t, a), bi = function(e, t, a) {
    var n = u0([
      e
    ], [], t);
    return n.height = Math.max(a || t.fontMetrics().defaultRuleThickness, t.minRuleThickness), n.style.borderBottomWidth = C(n.height), n.maxFontSize = 1, n;
  }, yi = function(e, t, a, n) {
    var i = new Jt(e, t, a, n);
    return Qt(i), i;
  }, Ca = function(e) {
    var t = new Te(e);
    return Qt(t), t;
  }, xi = function(e, t) {
    return e instanceof Te ? u0([], [
      e
    ], t) : e;
  }, wi = function(e) {
    if (e.positionType === "individualShift") {
      for (var t = e.children, a = [
        t[0]
      ], n = -t[0].shift - t[0].elem.depth, i = n, o = 1; o < t.length; o++) {
        var u = -t[o].shift - i - t[o].elem.depth, m = u - (t[o - 1].elem.height + t[o - 1].elem.depth);
        i = i + u, a.push({
          type: "kern",
          size: m
        }), a.push(t[o]);
      }
      return {
        children: a,
        depth: n
      };
    }
    var d;
    if (e.positionType === "top") {
      for (var p = e.positionData, b = 0; b < e.children.length; b++) {
        var y = e.children[b];
        p -= y.type === "kern" ? y.size : y.elem.height + y.elem.depth;
      }
      d = p;
    } else if (e.positionType === "bottom") d = -e.positionData;
    else {
      var x = e.children[0];
      if (x.type !== "elem") throw new Error('First child must have type "elem".');
      if (e.positionType === "shift") d = -x.elem.depth - e.positionData;
      else if (e.positionType === "firstBaseline") d = -x.elem.depth;
      else throw new Error("Invalid positionType " + e.positionType + ".");
    }
    return {
      children: e.children,
      depth: d
    };
  }, ki = function(e, t) {
    for (var { children: a, depth: n } = wi(e), i = 0, o = 0; o < a.length; o++) {
      var u = a[o];
      if (u.type === "elem") {
        var m = u.elem;
        i = Math.max(i, m.maxFontSize, m.height);
      }
    }
    i += 2;
    var d = u0([
      "pstrut"
    ], []);
    d.style.height = C(i);
    for (var p = [], b = n, y = n, x = n, k = 0; k < a.length; k++) {
      var S = a[k];
      if (S.type === "kern") x += S.size;
      else {
        var z = S.elem, B = S.wrapperClasses || [], N = S.wrapperStyle || {}, E = u0(B, [
          d,
          z
        ], void 0, N);
        E.style.top = C(-i - x - z.depth), S.marginLeft && (E.style.marginLeft = S.marginLeft), S.marginRight && (E.style.marginRight = S.marginRight), p.push(E), x += z.height + z.depth;
      }
      b = Math.min(b, x), y = Math.max(y, x);
    }
    var H = u0([
      "vlist"
    ], p);
    H.style.height = C(y);
    var O;
    if (b < 0) {
      var L = u0([], []), V = u0([
        "vlist"
      ], [
        L
      ]);
      V.style.height = C(-b);
      var U = u0([
        "vlist-s"
      ], [
        new b0("\u200B")
      ]);
      O = [
        u0([
          "vlist-r"
        ], [
          H,
          U
        ]),
        u0([
          "vlist-r"
        ], [
          V
        ])
      ];
    } else O = [
      u0([
        "vlist-r"
      ], [
        H
      ])
    ];
    var P = u0([
      "vlist-t"
    ], O);
    return O.length === 2 && P.classes.push("vlist-t2"), P.height = y, P.depth = -b, P;
  }, Si = (r, e) => {
    var t = u0([
      "mspace"
    ], [], e), a = K(r, e);
    return t.style.marginRight = C(a), t;
  }, He = function(e, t, a) {
    var n = "";
    switch (e) {
      case "amsrm":
        n = "AMS";
        break;
      case "textrm":
        n = "Main";
        break;
      case "textsf":
        n = "SansSerif";
        break;
      case "texttt":
        n = "Typewriter";
        break;
      default:
        n = e;
    }
    var i;
    return t === "textbf" && a === "textit" ? i = "BoldItalic" : t === "textbf" ? i = "Bold" : t === "textit" ? i = "Italic" : i = "Regular", n + "-" + i;
  }, Da = {
    mathbf: {
      variant: "bold",
      fontName: "Main-Bold"
    },
    mathrm: {
      variant: "normal",
      fontName: "Main-Regular"
    },
    textit: {
      variant: "italic",
      fontName: "Main-Italic"
    },
    mathit: {
      variant: "italic",
      fontName: "Main-Italic"
    },
    mathnormal: {
      variant: "italic",
      fontName: "Math-Italic"
    },
    mathsfit: {
      variant: "sans-serif-italic",
      fontName: "SansSerif-Italic"
    },
    mathbb: {
      variant: "double-struck",
      fontName: "AMS-Regular"
    },
    mathcal: {
      variant: "script",
      fontName: "Caligraphic-Regular"
    },
    mathfrak: {
      variant: "fraktur",
      fontName: "Fraktur-Regular"
    },
    mathscr: {
      variant: "script",
      fontName: "Script-Regular"
    },
    mathsf: {
      variant: "sans-serif",
      fontName: "SansSerif-Regular"
    },
    mathtt: {
      variant: "monospace",
      fontName: "Typewriter-Regular"
    }
  }, Na = {
    vec: [
      "vec",
      0.471,
      0.714
    ],
    oiintSize1: [
      "oiintSize1",
      0.957,
      0.499
    ],
    oiintSize2: [
      "oiintSize2",
      1.472,
      0.659
    ],
    oiiintSize1: [
      "oiiintSize1",
      1.304,
      0.499
    ],
    oiiintSize2: [
      "oiiintSize2",
      1.98,
      0.659
    ]
  }, Mi = function(e, t) {
    var [a, n, i] = Na[e], o = new _0(a), u = new H0([
      o
    ], {
      width: C(n),
      height: C(i),
      style: "width:" + C(n),
      viewBox: "0 0 " + 1e3 * n + " " + 1e3 * i,
      preserveAspectRatio: "xMinYMin"
    }), m = Ba([
      "overlay"
    ], [
      u
    ], t);
    return m.height = i, m.style.height = C(i), m.style.width = C(n), m;
  }, w = {
    fontMap: Da,
    makeSymbol: k0,
    mathsym: di,
    makeSpan: u0,
    makeSvgSpan: Ba,
    makeLineSpan: bi,
    makeAnchor: yi,
    makeFragment: Ca,
    wrapFragment: xi,
    makeVList: ki,
    makeOrd: vi,
    makeGlue: Si,
    staticSvg: Mi,
    svgData: Na,
    tryCombineChars: gi
  }, j = {
    number: 3,
    unit: "mu"
  }, re = {
    number: 4,
    unit: "mu"
  }, E0 = {
    number: 5,
    unit: "mu"
  }, zi = {
    mord: {
      mop: j,
      mbin: re,
      mrel: E0,
      minner: j
    },
    mop: {
      mord: j,
      mop: j,
      mrel: E0,
      minner: j
    },
    mbin: {
      mord: re,
      mop: re,
      mopen: re,
      minner: re
    },
    mrel: {
      mord: E0,
      mop: E0,
      mopen: E0,
      minner: E0
    },
    mopen: {},
    mclose: {
      mop: j,
      mbin: re,
      mrel: E0,
      minner: j
    },
    mpunct: {
      mord: j,
      mop: j,
      mrel: E0,
      mopen: j,
      mclose: j,
      mpunct: j,
      minner: j
    },
    minner: {
      mord: j,
      mop: j,
      mbin: re,
      mrel: E0,
      mopen: j,
      mpunct: j,
      minner: j
    }
  }, Ai = {
    mord: {
      mop: j
    },
    mop: {
      mord: j,
      mop: j
    },
    mbin: {},
    mrel: {},
    mopen: {},
    mclose: {
      mop: j
    },
    mpunct: {},
    minner: {
      mop: j
    }
  }, qa = {}, Ke = {}, _e = {};
  function D(r) {
    for (var { type: e, names: t, props: a, handler: n, htmlBuilder: i, mathmlBuilder: o } = r, u = {
      type: e,
      numArgs: a.numArgs,
      argTypes: a.argTypes,
      allowedInArgument: !!a.allowedInArgument,
      allowedInText: !!a.allowedInText,
      allowedInMath: a.allowedInMath === void 0 ? true : a.allowedInMath,
      numOptionalArgs: a.numOptionalArgs || 0,
      infix: !!a.infix,
      primitive: !!a.primitive,
      handler: n
    }, m = 0; m < t.length; ++m) qa[t[m]] = u;
    e && (i && (Ke[e] = i), o && (_e[e] = o));
  }
  function ae(r) {
    var { type: e, htmlBuilder: t, mathmlBuilder: a } = r;
    D({
      type: e,
      names: [],
      props: {
        numArgs: 0
      },
      handler() {
        throw new Error("Should never be called.");
      },
      htmlBuilder: t,
      mathmlBuilder: a
    });
  }
  var Je = function(e) {
    return e.type === "ordgroup" && e.body.length === 1 ? e.body[0] : e;
  }, J = function(e) {
    return e.type === "ordgroup" ? e.body : [
      e
    ];
  }, L0 = w.makeSpan, Ti = [
    "leftmost",
    "mbin",
    "mopen",
    "mrel",
    "mop",
    "mpunct"
  ], Bi = [
    "rightmost",
    "mrel",
    "mclose",
    "mpunct"
  ], Ci = {
    display: I.DISPLAY,
    text: I.TEXT,
    script: I.SCRIPT,
    scriptscript: I.SCRIPTSCRIPT
  }, Di = {
    mord: "mord",
    mop: "mop",
    mbin: "mbin",
    mrel: "mrel",
    mopen: "mopen",
    mclose: "mclose",
    mpunct: "mpunct",
    minner: "minner"
  }, t0 = function(e, t, a, n) {
    n === void 0 && (n = [
      null,
      null
    ]);
    for (var i = [], o = 0; o < e.length; o++) {
      var u = $(e[o], t);
      if (u instanceof Te) {
        var m = u.children;
        i.push(...m);
      } else i.push(u);
    }
    if (w.tryCombineChars(i), !a) return i;
    var d = t;
    if (e.length === 1) {
      var p = e[0];
      p.type === "sizing" ? d = t.havingSize(p.size) : p.type === "styling" && (d = t.havingStyle(Ci[p.style]));
    }
    var b = L0([
      n[0] || "leftmost"
    ], [], t), y = L0([
      n[1] || "rightmost"
    ], [], t), x = a === "root";
    return Or(i, (k, S) => {
      var z = S.classes[0], B = k.classes[0];
      z === "mbin" && Bi.includes(B) ? S.classes[0] = "mord" : B === "mbin" && Ti.includes(z) && (k.classes[0] = "mord");
    }, {
      node: b
    }, y, x), Or(i, (k, S) => {
      var z = Gt(S), B = Gt(k), N = z && B ? k.hasClass("mtight") ? Ai[z][B] : zi[z][B] : null;
      if (N) return w.makeGlue(N, d);
    }, {
      node: b
    }, y, x), i;
  }, Or = function r(e, t, a, n, i) {
    n && e.push(n);
    for (var o = 0; o < e.length; o++) {
      var u = e[o], m = Ea(u);
      if (m) {
        r(m.children, t, a, null, i);
        continue;
      }
      var d = !u.hasClass("mspace");
      if (d) {
        var p = t(u, a.node);
        p && (a.insertAfter ? a.insertAfter(p) : (e.unshift(p), o++));
      }
      d ? a.node = u : i && u.hasClass("newline") && (a.node = L0([
        "leftmost"
      ])), a.insertAfter = /* @__PURE__ */ ((b) => (y) => {
        e.splice(b + 1, 0, y), o++;
      })(o);
    }
    n && e.pop();
  }, Ea = function(e) {
    return e instanceof Te || e instanceof Jt || e instanceof Be && e.hasClass("enclosing") ? e : null;
  }, Ni = function r(e, t) {
    var a = Ea(e);
    if (a) {
      var n = a.children;
      if (n.length) {
        if (t === "right") return r(n[n.length - 1], "right");
        if (t === "left") return r(n[0], "left");
      }
    }
    return e;
  }, Gt = function(e, t) {
    return e ? (t && (e = Ni(e, t)), Di[e.classes[0]] || null) : null;
  }, Ae = function(e, t) {
    var a = [
      "nulldelimiter"
    ].concat(e.baseSizingClasses());
    return L0(t.concat(a));
  }, $ = function(e, t, a) {
    if (!e) return L0();
    if (Ke[e.type]) {
      var n = Ke[e.type](e, t);
      if (a && t.size !== a.size) {
        n = L0(t.sizingClasses(a), [
          n
        ], t);
        var i = t.sizeMultiplier / a.sizeMultiplier;
        n.height *= i, n.depth *= i;
      }
      return n;
    } else throw new T("Got group of unknown type: '" + e.type + "'");
  };
  function Le(r, e) {
    var t = L0([
      "base"
    ], r, e), a = L0([
      "strut"
    ]);
    return a.style.height = C(t.height + t.depth), t.depth && (a.style.verticalAlign = C(-t.depth)), t.children.unshift(a), t;
  }
  function Ut(r, e) {
    var t = null;
    r.length === 1 && r[0].type === "tag" && (t = r[0].tag, r = r[0].body);
    var a = t0(r, e, "root"), n;
    a.length === 2 && a[1].hasClass("tag") && (n = a.pop());
    for (var i = [], o = [], u = 0; u < a.length; u++) if (o.push(a[u]), a[u].hasClass("mbin") || a[u].hasClass("mrel") || a[u].hasClass("allowbreak")) {
      for (var m = false; u < a.length - 1 && a[u + 1].hasClass("mspace") && !a[u + 1].hasClass("newline"); ) u++, o.push(a[u]), a[u].hasClass("nobreak") && (m = true);
      m || (i.push(Le(o, e)), o = []);
    } else a[u].hasClass("newline") && (o.pop(), o.length > 0 && (i.push(Le(o, e)), o = []), i.push(a[u]));
    o.length > 0 && i.push(Le(o, e));
    var d;
    t ? (d = Le(t0(t, e, true)), d.classes = [
      "tag"
    ], i.push(d)) : n && i.push(n);
    var p = L0([
      "katex-html"
    ], i);
    if (p.setAttribute("aria-hidden", "true"), d) {
      var b = d.children[0];
      b.style.height = C(p.height + p.depth), p.depth && (b.style.verticalAlign = C(-p.depth));
    }
    return p;
  }
  function Ra(r) {
    return new Te(r);
  }
  class c0 {
    constructor(e, t, a) {
      this.type = void 0, this.attributes = void 0, this.children = void 0, this.classes = void 0, this.type = e, this.attributes = {}, this.children = t || [], this.classes = a || [];
    }
    setAttribute(e, t) {
      this.attributes[e] = t;
    }
    getAttribute(e) {
      return this.attributes[e];
    }
    toNode() {
      var e = document.createElementNS("http://www.w3.org/1998/Math/MathML", this.type);
      for (var t in this.attributes) Object.prototype.hasOwnProperty.call(this.attributes, t) && e.setAttribute(t, this.attributes[t]);
      this.classes.length > 0 && (e.className = K0(this.classes));
      for (var a = 0; a < this.children.length; a++) if (this.children[a] instanceof B0 && this.children[a + 1] instanceof B0) {
        for (var n = this.children[a].toText() + this.children[++a].toText(); this.children[a + 1] instanceof B0; ) n += this.children[++a].toText();
        e.appendChild(new B0(n).toNode());
      } else e.appendChild(this.children[a].toNode());
      return e;
    }
    toMarkup() {
      var e = "<" + this.type;
      for (var t in this.attributes) Object.prototype.hasOwnProperty.call(this.attributes, t) && (e += " " + t + '="', e += Y.escape(this.attributes[t]), e += '"');
      this.classes.length > 0 && (e += ' class ="' + Y.escape(K0(this.classes)) + '"'), e += ">";
      for (var a = 0; a < this.children.length; a++) e += this.children[a].toMarkup();
      return e += "</" + this.type + ">", e;
    }
    toText() {
      return this.children.map((e) => e.toText()).join("");
    }
  }
  class B0 {
    constructor(e) {
      this.text = void 0, this.text = e;
    }
    toNode() {
      return document.createTextNode(this.text);
    }
    toMarkup() {
      return Y.escape(this.toText());
    }
    toText() {
      return this.text;
    }
  }
  class qi {
    constructor(e) {
      this.width = void 0, this.character = void 0, this.width = e, e >= 0.05555 && e <= 0.05556 ? this.character = "\u200A" : e >= 0.1666 && e <= 0.1667 ? this.character = "\u2009" : e >= 0.2222 && e <= 0.2223 ? this.character = "\u2005" : e >= 0.2777 && e <= 0.2778 ? this.character = "\u2005\u200A" : e >= -0.05556 && e <= -0.05555 ? this.character = "\u200A\u2063" : e >= -0.1667 && e <= -0.1666 ? this.character = "\u2009\u2063" : e >= -0.2223 && e <= -0.2222 ? this.character = "\u205F\u2063" : e >= -0.2778 && e <= -0.2777 ? this.character = "\u2005\u2063" : this.character = null;
    }
    toNode() {
      if (this.character) return document.createTextNode(this.character);
      var e = document.createElementNS("http://www.w3.org/1998/Math/MathML", "mspace");
      return e.setAttribute("width", C(this.width)), e;
    }
    toMarkup() {
      return this.character ? "<mtext>" + this.character + "</mtext>" : '<mspace width="' + C(this.width) + '"/>';
    }
    toText() {
      return this.character ? this.character : " ";
    }
  }
  var A = {
    MathNode: c0,
    TextNode: B0,
    SpaceNode: qi,
    newDocumentFragment: Ra
  }, y0 = function(e, t, a) {
    return W[t][e] && W[t][e].replace && e.charCodeAt(0) !== 55349 && !(Ta.hasOwnProperty(e) && a && (a.fontFamily && a.fontFamily.slice(4, 6) === "tt" || a.font && a.font.slice(4, 6) === "tt")) && (e = W[t][e].replace), new A.TextNode(e);
  }, er = function(e) {
    return e.length === 1 ? e[0] : new A.MathNode("mrow", e);
  }, tr = function(e, t) {
    if (t.fontFamily === "texttt") return "monospace";
    if (t.fontFamily === "textsf") return t.fontShape === "textit" && t.fontWeight === "textbf" ? "sans-serif-bold-italic" : t.fontShape === "textit" ? "sans-serif-italic" : t.fontWeight === "textbf" ? "bold-sans-serif" : "sans-serif";
    if (t.fontShape === "textit" && t.fontWeight === "textbf") return "bold-italic";
    if (t.fontShape === "textit") return "italic";
    if (t.fontWeight === "textbf") return "bold";
    var a = t.font;
    if (!a || a === "mathnormal") return null;
    var n = e.mode;
    if (a === "mathit") return "italic";
    if (a === "boldsymbol") return e.type === "textord" ? "bold" : "bold-italic";
    if (a === "mathbf") return "bold";
    if (a === "mathbb") return "double-struck";
    if (a === "mathsfit") return "sans-serif-italic";
    if (a === "mathfrak") return "fraktur";
    if (a === "mathscr" || a === "mathcal") return "script";
    if (a === "mathsf") return "sans-serif";
    if (a === "mathtt") return "monospace";
    var i = e.text;
    if ([
      "\\imath",
      "\\jmath"
    ].includes(i)) return null;
    W[n][i] && W[n][i].replace && (i = W[n][i].replace);
    var o = w.fontMap[a].fontName;
    return _t(i, o, n) ? w.fontMap[a].variant : null;
  };
  function kt(r) {
    if (!r) return false;
    if (r.type === "mi" && r.children.length === 1) {
      var e = r.children[0];
      return e instanceof B0 && e.text === ".";
    } else if (r.type === "mo" && r.children.length === 1 && r.getAttribute("separator") === "true" && r.getAttribute("lspace") === "0em" && r.getAttribute("rspace") === "0em") {
      var t = r.children[0];
      return t instanceof B0 && t.text === ",";
    } else return false;
  }
  var m0 = function(e, t, a) {
    if (e.length === 1) {
      var n = X(e[0], t);
      return a && n instanceof c0 && n.type === "mo" && (n.setAttribute("lspace", "0em"), n.setAttribute("rspace", "0em")), [
        n
      ];
    }
    for (var i = [], o, u = 0; u < e.length; u++) {
      var m = X(e[u], t);
      if (m instanceof c0 && o instanceof c0) {
        if (m.type === "mtext" && o.type === "mtext" && m.getAttribute("mathvariant") === o.getAttribute("mathvariant")) {
          o.children.push(...m.children);
          continue;
        } else if (m.type === "mn" && o.type === "mn") {
          o.children.push(...m.children);
          continue;
        } else if (kt(m) && o.type === "mn") {
          o.children.push(...m.children);
          continue;
        } else if (m.type === "mn" && kt(o)) m.children = [
          ...o.children,
          ...m.children
        ], i.pop();
        else if ((m.type === "msup" || m.type === "msub") && m.children.length >= 1 && (o.type === "mn" || kt(o))) {
          var d = m.children[0];
          d instanceof c0 && d.type === "mn" && (d.children = [
            ...o.children,
            ...d.children
          ], i.pop());
        } else if (o.type === "mi" && o.children.length === 1) {
          var p = o.children[0];
          if (p instanceof B0 && p.text === "\u0338" && (m.type === "mo" || m.type === "mi" || m.type === "mn")) {
            var b = m.children[0];
            b instanceof B0 && b.text.length > 0 && (b.text = b.text.slice(0, 1) + "\u0338" + b.text.slice(1), i.pop());
          }
        }
      }
      i.push(m), o = m;
    }
    return i;
  }, J0 = function(e, t, a) {
    return er(m0(e, t, a));
  }, X = function(e, t) {
    if (!e) return new A.MathNode("mrow");
    if (_e[e.type]) {
      var a = _e[e.type](e, t);
      return a;
    } else throw new T("Got group of unknown type: '" + e.type + "'");
  };
  function Fr(r, e, t, a, n) {
    var i = m0(r, t), o;
    i.length === 1 && i[0] instanceof c0 && [
      "mrow",
      "mtable"
    ].includes(i[0].type) ? o = i[0] : o = new A.MathNode("mrow", i);
    var u = new A.MathNode("annotation", [
      new A.TextNode(e)
    ]);
    u.setAttribute("encoding", "application/x-tex");
    var m = new A.MathNode("semantics", [
      o,
      u
    ]), d = new A.MathNode("math", [
      m
    ]);
    d.setAttribute("xmlns", "http://www.w3.org/1998/Math/MathML"), a && d.setAttribute("display", "block");
    var p = n ? "katex" : "katex-mathml";
    return w.makeSpan([
      p
    ], [
      d
    ]);
  }
  var Ia = function(e) {
    return new R0({
      style: e.displayMode ? I.DISPLAY : I.TEXT,
      maxSize: e.maxSize,
      minRuleThickness: e.minRuleThickness
    });
  }, Oa = function(e, t) {
    if (t.displayMode) {
      var a = [
        "katex-display"
      ];
      t.leqno && a.push("leqno"), t.fleqn && a.push("fleqn"), e = w.makeSpan(a, [
        e
      ]);
    }
    return e;
  }, Ei = function(e, t, a) {
    var n = Ia(a), i;
    if (a.output === "mathml") return Fr(e, t, n, a.displayMode, true);
    if (a.output === "html") {
      var o = Ut(e, n);
      i = w.makeSpan([
        "katex"
      ], [
        o
      ]);
    } else {
      var u = Fr(e, t, n, a.displayMode, false), m = Ut(e, n);
      i = w.makeSpan([
        "katex"
      ], [
        u,
        m
      ]);
    }
    return Oa(i, a);
  }, Ri = function(e, t, a) {
    var n = Ia(a), i = Ut(e, n), o = w.makeSpan([
      "katex"
    ], [
      i
    ]);
    return Oa(o, a);
  }, Ii = {
    widehat: "^",
    widecheck: "\u02C7",
    widetilde: "~",
    utilde: "~",
    overleftarrow: "\u2190",
    underleftarrow: "\u2190",
    xleftarrow: "\u2190",
    overrightarrow: "\u2192",
    underrightarrow: "\u2192",
    xrightarrow: "\u2192",
    underbrace: "\u23DF",
    overbrace: "\u23DE",
    overgroup: "\u23E0",
    undergroup: "\u23E1",
    overleftrightarrow: "\u2194",
    underleftrightarrow: "\u2194",
    xleftrightarrow: "\u2194",
    Overrightarrow: "\u21D2",
    xRightarrow: "\u21D2",
    overleftharpoon: "\u21BC",
    xleftharpoonup: "\u21BC",
    overrightharpoon: "\u21C0",
    xrightharpoonup: "\u21C0",
    xLeftarrow: "\u21D0",
    xLeftrightarrow: "\u21D4",
    xhookleftarrow: "\u21A9",
    xhookrightarrow: "\u21AA",
    xmapsto: "\u21A6",
    xrightharpoondown: "\u21C1",
    xleftharpoondown: "\u21BD",
    xrightleftharpoons: "\u21CC",
    xleftrightharpoons: "\u21CB",
    xtwoheadleftarrow: "\u219E",
    xtwoheadrightarrow: "\u21A0",
    xlongequal: "=",
    xtofrom: "\u21C4",
    xrightleftarrows: "\u21C4",
    xrightequilibrium: "\u21CC",
    xleftequilibrium: "\u21CB",
    "\\cdrightarrow": "\u2192",
    "\\cdleftarrow": "\u2190",
    "\\cdlongequal": "="
  }, Oi = function(e) {
    var t = new A.MathNode("mo", [
      new A.TextNode(Ii[e.replace(/^\\/, "")])
    ]);
    return t.setAttribute("stretchy", "true"), t;
  }, Fi = {
    overrightarrow: [
      [
        "rightarrow"
      ],
      0.888,
      522,
      "xMaxYMin"
    ],
    overleftarrow: [
      [
        "leftarrow"
      ],
      0.888,
      522,
      "xMinYMin"
    ],
    underrightarrow: [
      [
        "rightarrow"
      ],
      0.888,
      522,
      "xMaxYMin"
    ],
    underleftarrow: [
      [
        "leftarrow"
      ],
      0.888,
      522,
      "xMinYMin"
    ],
    xrightarrow: [
      [
        "rightarrow"
      ],
      1.469,
      522,
      "xMaxYMin"
    ],
    "\\cdrightarrow": [
      [
        "rightarrow"
      ],
      3,
      522,
      "xMaxYMin"
    ],
    xleftarrow: [
      [
        "leftarrow"
      ],
      1.469,
      522,
      "xMinYMin"
    ],
    "\\cdleftarrow": [
      [
        "leftarrow"
      ],
      3,
      522,
      "xMinYMin"
    ],
    Overrightarrow: [
      [
        "doublerightarrow"
      ],
      0.888,
      560,
      "xMaxYMin"
    ],
    xRightarrow: [
      [
        "doublerightarrow"
      ],
      1.526,
      560,
      "xMaxYMin"
    ],
    xLeftarrow: [
      [
        "doubleleftarrow"
      ],
      1.526,
      560,
      "xMinYMin"
    ],
    overleftharpoon: [
      [
        "leftharpoon"
      ],
      0.888,
      522,
      "xMinYMin"
    ],
    xleftharpoonup: [
      [
        "leftharpoon"
      ],
      0.888,
      522,
      "xMinYMin"
    ],
    xleftharpoondown: [
      [
        "leftharpoondown"
      ],
      0.888,
      522,
      "xMinYMin"
    ],
    overrightharpoon: [
      [
        "rightharpoon"
      ],
      0.888,
      522,
      "xMaxYMin"
    ],
    xrightharpoonup: [
      [
        "rightharpoon"
      ],
      0.888,
      522,
      "xMaxYMin"
    ],
    xrightharpoondown: [
      [
        "rightharpoondown"
      ],
      0.888,
      522,
      "xMaxYMin"
    ],
    xlongequal: [
      [
        "longequal"
      ],
      0.888,
      334,
      "xMinYMin"
    ],
    "\\cdlongequal": [
      [
        "longequal"
      ],
      3,
      334,
      "xMinYMin"
    ],
    xtwoheadleftarrow: [
      [
        "twoheadleftarrow"
      ],
      0.888,
      334,
      "xMinYMin"
    ],
    xtwoheadrightarrow: [
      [
        "twoheadrightarrow"
      ],
      0.888,
      334,
      "xMaxYMin"
    ],
    overleftrightarrow: [
      [
        "leftarrow",
        "rightarrow"
      ],
      0.888,
      522
    ],
    overbrace: [
      [
        "leftbrace",
        "midbrace",
        "rightbrace"
      ],
      1.6,
      548
    ],
    underbrace: [
      [
        "leftbraceunder",
        "midbraceunder",
        "rightbraceunder"
      ],
      1.6,
      548
    ],
    underleftrightarrow: [
      [
        "leftarrow",
        "rightarrow"
      ],
      0.888,
      522
    ],
    xleftrightarrow: [
      [
        "leftarrow",
        "rightarrow"
      ],
      1.75,
      522
    ],
    xLeftrightarrow: [
      [
        "doubleleftarrow",
        "doublerightarrow"
      ],
      1.75,
      560
    ],
    xrightleftharpoons: [
      [
        "leftharpoondownplus",
        "rightharpoonplus"
      ],
      1.75,
      716
    ],
    xleftrightharpoons: [
      [
        "leftharpoonplus",
        "rightharpoondownplus"
      ],
      1.75,
      716
    ],
    xhookleftarrow: [
      [
        "leftarrow",
        "righthook"
      ],
      1.08,
      522
    ],
    xhookrightarrow: [
      [
        "lefthook",
        "rightarrow"
      ],
      1.08,
      522
    ],
    overlinesegment: [
      [
        "leftlinesegment",
        "rightlinesegment"
      ],
      0.888,
      522
    ],
    underlinesegment: [
      [
        "leftlinesegment",
        "rightlinesegment"
      ],
      0.888,
      522
    ],
    overgroup: [
      [
        "leftgroup",
        "rightgroup"
      ],
      0.888,
      342
    ],
    undergroup: [
      [
        "leftgroupunder",
        "rightgroupunder"
      ],
      0.888,
      342
    ],
    xmapsto: [
      [
        "leftmapsto",
        "rightarrow"
      ],
      1.5,
      522
    ],
    xtofrom: [
      [
        "leftToFrom",
        "rightToFrom"
      ],
      1.75,
      528
    ],
    xrightleftarrows: [
      [
        "baraboveleftarrow",
        "rightarrowabovebar"
      ],
      1.75,
      901
    ],
    xrightequilibrium: [
      [
        "baraboveshortleftharpoon",
        "rightharpoonaboveshortbar"
      ],
      1.75,
      716
    ],
    xleftequilibrium: [
      [
        "shortbaraboveleftharpoon",
        "shortrightharpoonabovebar"
      ],
      1.75,
      716
    ]
  }, Hi = function(e) {
    return e.type === "ordgroup" ? e.body.length : 1;
  }, Li = function(e, t) {
    function a() {
      var u = 4e5, m = e.label.slice(1);
      if ([
        "widehat",
        "widecheck",
        "widetilde",
        "utilde"
      ].includes(m)) {
        var d = e, p = Hi(d.base), b, y, x;
        if (p > 5) m === "widehat" || m === "widecheck" ? (b = 420, u = 2364, x = 0.42, y = m + "4") : (b = 312, u = 2340, x = 0.34, y = "tilde4");
        else {
          var k = [
            1,
            1,
            2,
            2,
            3,
            3
          ][p];
          m === "widehat" || m === "widecheck" ? (u = [
            0,
            1062,
            2364,
            2364,
            2364
          ][k], b = [
            0,
            239,
            300,
            360,
            420
          ][k], x = [
            0,
            0.24,
            0.3,
            0.3,
            0.36,
            0.42
          ][k], y = m + k) : (u = [
            0,
            600,
            1033,
            2339,
            2340
          ][k], b = [
            0,
            260,
            286,
            306,
            312
          ][k], x = [
            0,
            0.26,
            0.286,
            0.3,
            0.306,
            0.34
          ][k], y = "tilde" + k);
        }
        var S = new _0(y), z = new H0([
          S
        ], {
          width: "100%",
          height: C(x),
          viewBox: "0 0 " + u + " " + b,
          preserveAspectRatio: "none"
        });
        return {
          span: w.makeSvgSpan([], [
            z
          ], t),
          minWidth: 0,
          height: x
        };
      } else {
        var B = [], N = Fi[m], [E, H, O] = N, L = O / 1e3, V = E.length, U, P;
        if (V === 1) {
          var x0 = N[3];
          U = [
            "hide-tail"
          ], P = [
            x0
          ];
        } else if (V === 2) U = [
          "halfarrow-left",
          "halfarrow-right"
        ], P = [
          "xMinYMin",
          "xMaxYMin"
        ];
        else if (V === 3) U = [
          "brace-left",
          "brace-center",
          "brace-right"
        ], P = [
          "xMinYMin",
          "xMidYMin",
          "xMaxYMin"
        ];
        else throw new Error(`Correct katexImagesData or update code here to support
                    ` + V + " children.");
        for (var r0 = 0; r0 < V; r0++) {
          var e0 = new _0(E[r0]), ee = new H0([
            e0
          ], {
            width: "400em",
            height: C(L),
            viewBox: "0 0 " + u + " " + O,
            preserveAspectRatio: P[r0] + " slice"
          }), o0 = w.makeSvgSpan([
            U[r0]
          ], [
            ee
          ], t);
          if (V === 1) return {
            span: o0,
            minWidth: H,
            height: L
          };
          o0.style.height = C(L), B.push(o0);
        }
        return {
          span: w.makeSpan([
            "stretchy"
          ], B, t),
          minWidth: H,
          height: L
        };
      }
    }
    var { span: n, minWidth: i, height: o } = a();
    return n.height = o, n.style.height = C(o), i > 0 && (n.style.minWidth = C(i)), n;
  }, Pi = function(e, t, a, n, i) {
    var o, u = e.height + e.depth + a + n;
    if (/fbox|color|angl/.test(t)) {
      if (o = w.makeSpan([
        "stretchy",
        t
      ], [], i), t === "fbox") {
        var m = i.color && i.getColor();
        m && (o.style.borderColor = m);
      }
    } else {
      var d = [];
      /^[bx]cancel$/.test(t) && d.push(new Pt({
        x1: "0",
        y1: "0",
        x2: "100%",
        y2: "100%",
        "stroke-width": "0.046em"
      })), /^x?cancel$/.test(t) && d.push(new Pt({
        x1: "0",
        y1: "100%",
        x2: "100%",
        y2: "0",
        "stroke-width": "0.046em"
      }));
      var p = new H0(d, {
        width: "100%",
        height: C(u)
      });
      o = w.makeSvgSpan([], [
        p
      ], i);
    }
    return o.height = u, o.style.height = C(u), o;
  }, P0 = {
    encloseSpan: Pi,
    mathMLnode: Oi,
    svgSpan: Li
  };
  function G(r, e) {
    if (!r || r.type !== e) throw new Error("Expected node of type " + e + ", but got " + (r ? "node of type " + r.type : String(r)));
    return r;
  }
  function rr(r) {
    var e = at(r);
    if (!e) throw new Error("Expected node of symbol group type, but got " + (r ? "node of type " + r.type : String(r)));
    return e;
  }
  function at(r) {
    return r && (r.type === "atom" || mi.hasOwnProperty(r.type)) ? r : null;
  }
  var ar = (r, e) => {
    var t, a, n;
    r && r.type === "supsub" ? (a = G(r.base, "accent"), t = a.base, r.base = t, n = ui($(r, e)), r.base = a) : (a = G(r, "accent"), t = a.base);
    var i = $(t, e.havingCrampedStyle()), o = a.isShifty && Y.isCharacterBox(t), u = 0;
    if (o) {
      var m = Y.getBaseElem(t), d = $(m, e.havingCrampedStyle());
      u = Dr(d).skew;
    }
    var p = a.label === "\\c", b = p ? i.height + i.depth : Math.min(i.height, e.fontMetrics().xHeight), y;
    if (a.isStretchy) y = P0.svgSpan(a, e), y = w.makeVList({
      positionType: "firstBaseline",
      children: [
        {
          type: "elem",
          elem: i
        },
        {
          type: "elem",
          elem: y,
          wrapperClasses: [
            "svg-align"
          ],
          wrapperStyle: u > 0 ? {
            width: "calc(100% - " + C(2 * u) + ")",
            marginLeft: C(2 * u)
          } : void 0
        }
      ]
    }, e);
    else {
      var x, k;
      a.label === "\\vec" ? (x = w.staticSvg("vec", e), k = w.svgData.vec[1]) : (x = w.makeOrd({
        mode: a.mode,
        text: a.label
      }, e, "textord"), x = Dr(x), x.italic = 0, k = x.width, p && (b += x.depth)), y = w.makeSpan([
        "accent-body"
      ], [
        x
      ]);
      var S = a.label === "\\textcircled";
      S && (y.classes.push("accent-full"), b = i.height);
      var z = u;
      S || (z -= k / 2), y.style.left = C(z), a.label === "\\textcircled" && (y.style.top = ".2em"), y = w.makeVList({
        positionType: "firstBaseline",
        children: [
          {
            type: "elem",
            elem: i
          },
          {
            type: "kern",
            size: -b
          },
          {
            type: "elem",
            elem: y
          }
        ]
      }, e);
    }
    var B = w.makeSpan([
      "mord",
      "accent"
    ], [
      y
    ], e);
    return n ? (n.children[0] = B, n.height = Math.max(B.height, n.height), n.classes[0] = "mord", n) : B;
  }, Fa = (r, e) => {
    var t = r.isStretchy ? P0.mathMLnode(r.label) : new A.MathNode("mo", [
      y0(r.label, r.mode)
    ]), a = new A.MathNode("mover", [
      X(r.base, e),
      t
    ]);
    return a.setAttribute("accent", "true"), a;
  }, Vi = new RegExp([
    "\\acute",
    "\\grave",
    "\\ddot",
    "\\tilde",
    "\\bar",
    "\\breve",
    "\\check",
    "\\hat",
    "\\vec",
    "\\dot",
    "\\mathring"
  ].map((r) => "\\" + r).join("|"));
  D({
    type: "accent",
    names: [
      "\\acute",
      "\\grave",
      "\\ddot",
      "\\tilde",
      "\\bar",
      "\\breve",
      "\\check",
      "\\hat",
      "\\vec",
      "\\dot",
      "\\mathring",
      "\\widecheck",
      "\\widehat",
      "\\widetilde",
      "\\overrightarrow",
      "\\overleftarrow",
      "\\Overrightarrow",
      "\\overleftrightarrow",
      "\\overgroup",
      "\\overlinesegment",
      "\\overleftharpoon",
      "\\overrightharpoon"
    ],
    props: {
      numArgs: 1
    },
    handler: (r, e) => {
      var t = Je(e[0]), a = !Vi.test(r.funcName), n = !a || r.funcName === "\\widehat" || r.funcName === "\\widetilde" || r.funcName === "\\widecheck";
      return {
        type: "accent",
        mode: r.parser.mode,
        label: r.funcName,
        isStretchy: a,
        isShifty: n,
        base: t
      };
    },
    htmlBuilder: ar,
    mathmlBuilder: Fa
  });
  D({
    type: "accent",
    names: [
      "\\'",
      "\\`",
      "\\^",
      "\\~",
      "\\=",
      "\\u",
      "\\.",
      '\\"',
      "\\c",
      "\\r",
      "\\H",
      "\\v",
      "\\textcircled"
    ],
    props: {
      numArgs: 1,
      allowedInText: true,
      allowedInMath: true,
      argTypes: [
        "primitive"
      ]
    },
    handler: (r, e) => {
      var t = e[0], a = r.parser.mode;
      return a === "math" && (r.parser.settings.reportNonstrict("mathVsTextAccents", "LaTeX's accent " + r.funcName + " works only in text mode"), a = "text"), {
        type: "accent",
        mode: a,
        label: r.funcName,
        isStretchy: false,
        isShifty: true,
        base: t
      };
    },
    htmlBuilder: ar,
    mathmlBuilder: Fa
  });
  D({
    type: "accentUnder",
    names: [
      "\\underleftarrow",
      "\\underrightarrow",
      "\\underleftrightarrow",
      "\\undergroup",
      "\\underlinesegment",
      "\\utilde"
    ],
    props: {
      numArgs: 1
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = e[0];
      return {
        type: "accentUnder",
        mode: t.mode,
        label: a,
        base: n
      };
    },
    htmlBuilder: (r, e) => {
      var t = $(r.base, e), a = P0.svgSpan(r, e), n = r.label === "\\utilde" ? 0.12 : 0, i = w.makeVList({
        positionType: "top",
        positionData: t.height,
        children: [
          {
            type: "elem",
            elem: a,
            wrapperClasses: [
              "svg-align"
            ]
          },
          {
            type: "kern",
            size: n
          },
          {
            type: "elem",
            elem: t
          }
        ]
      }, e);
      return w.makeSpan([
        "mord",
        "accentunder"
      ], [
        i
      ], e);
    },
    mathmlBuilder: (r, e) => {
      var t = P0.mathMLnode(r.label), a = new A.MathNode("munder", [
        X(r.base, e),
        t
      ]);
      return a.setAttribute("accentunder", "true"), a;
    }
  });
  var Pe = (r) => {
    var e = new A.MathNode("mpadded", r ? [
      r
    ] : []);
    return e.setAttribute("width", "+0.6em"), e.setAttribute("lspace", "0.3em"), e;
  };
  D({
    type: "xArrow",
    names: [
      "\\xleftarrow",
      "\\xrightarrow",
      "\\xLeftarrow",
      "\\xRightarrow",
      "\\xleftrightarrow",
      "\\xLeftrightarrow",
      "\\xhookleftarrow",
      "\\xhookrightarrow",
      "\\xmapsto",
      "\\xrightharpoondown",
      "\\xrightharpoonup",
      "\\xleftharpoondown",
      "\\xleftharpoonup",
      "\\xrightleftharpoons",
      "\\xleftrightharpoons",
      "\\xlongequal",
      "\\xtwoheadrightarrow",
      "\\xtwoheadleftarrow",
      "\\xtofrom",
      "\\xrightleftarrows",
      "\\xrightequilibrium",
      "\\xleftequilibrium",
      "\\\\cdrightarrow",
      "\\\\cdleftarrow",
      "\\\\cdlongequal"
    ],
    props: {
      numArgs: 1,
      numOptionalArgs: 1
    },
    handler(r, e, t) {
      var { parser: a, funcName: n } = r;
      return {
        type: "xArrow",
        mode: a.mode,
        label: n,
        body: e[0],
        below: t[0]
      };
    },
    htmlBuilder(r, e) {
      var t = e.style, a = e.havingStyle(t.sup()), n = w.wrapFragment($(r.body, a, e), e), i = r.label.slice(0, 2) === "\\x" ? "x" : "cd";
      n.classes.push(i + "-arrow-pad");
      var o;
      r.below && (a = e.havingStyle(t.sub()), o = w.wrapFragment($(r.below, a, e), e), o.classes.push(i + "-arrow-pad"));
      var u = P0.svgSpan(r, e), m = -e.fontMetrics().axisHeight + 0.5 * u.height, d = -e.fontMetrics().axisHeight - 0.5 * u.height - 0.111;
      (n.depth > 0.25 || r.label === "\\xleftequilibrium") && (d -= n.depth);
      var p;
      if (o) {
        var b = -e.fontMetrics().axisHeight + o.height + 0.5 * u.height + 0.111;
        p = w.makeVList({
          positionType: "individualShift",
          children: [
            {
              type: "elem",
              elem: n,
              shift: d
            },
            {
              type: "elem",
              elem: u,
              shift: m
            },
            {
              type: "elem",
              elem: o,
              shift: b
            }
          ]
        }, e);
      } else p = w.makeVList({
        positionType: "individualShift",
        children: [
          {
            type: "elem",
            elem: n,
            shift: d
          },
          {
            type: "elem",
            elem: u,
            shift: m
          }
        ]
      }, e);
      return p.children[0].children[0].children[1].classes.push("svg-align"), w.makeSpan([
        "mrel",
        "x-arrow"
      ], [
        p
      ], e);
    },
    mathmlBuilder(r, e) {
      var t = P0.mathMLnode(r.label);
      t.setAttribute("minsize", r.label.charAt(0) === "x" ? "1.75em" : "3.0em");
      var a;
      if (r.body) {
        var n = Pe(X(r.body, e));
        if (r.below) {
          var i = Pe(X(r.below, e));
          a = new A.MathNode("munderover", [
            t,
            i,
            n
          ]);
        } else a = new A.MathNode("mover", [
          t,
          n
        ]);
      } else if (r.below) {
        var o = Pe(X(r.below, e));
        a = new A.MathNode("munder", [
          t,
          o
        ]);
      } else a = Pe(), a = new A.MathNode("mover", [
        t,
        a
      ]);
      return a;
    }
  });
  var Gi = w.makeSpan;
  function Ha(r, e) {
    var t = t0(r.body, e, true);
    return Gi([
      r.mclass
    ], t, e);
  }
  function La(r, e) {
    var t, a = m0(r.body, e);
    return r.mclass === "minner" ? t = new A.MathNode("mpadded", a) : r.mclass === "mord" ? r.isCharacterBox ? (t = a[0], t.type = "mi") : t = new A.MathNode("mi", a) : (r.isCharacterBox ? (t = a[0], t.type = "mo") : t = new A.MathNode("mo", a), r.mclass === "mbin" ? (t.attributes.lspace = "0.22em", t.attributes.rspace = "0.22em") : r.mclass === "mpunct" ? (t.attributes.lspace = "0em", t.attributes.rspace = "0.17em") : r.mclass === "mopen" || r.mclass === "mclose" ? (t.attributes.lspace = "0em", t.attributes.rspace = "0em") : r.mclass === "minner" && (t.attributes.lspace = "0.0556em", t.attributes.width = "+0.1111em")), t;
  }
  D({
    type: "mclass",
    names: [
      "\\mathord",
      "\\mathbin",
      "\\mathrel",
      "\\mathopen",
      "\\mathclose",
      "\\mathpunct",
      "\\mathinner"
    ],
    props: {
      numArgs: 1,
      primitive: true
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r, n = e[0];
      return {
        type: "mclass",
        mode: t.mode,
        mclass: "m" + a.slice(5),
        body: J(n),
        isCharacterBox: Y.isCharacterBox(n)
      };
    },
    htmlBuilder: Ha,
    mathmlBuilder: La
  });
  var nt = (r) => {
    var e = r.type === "ordgroup" && r.body.length ? r.body[0] : r;
    return e.type === "atom" && (e.family === "bin" || e.family === "rel") ? "m" + e.family : "mord";
  };
  D({
    type: "mclass",
    names: [
      "\\@binrel"
    ],
    props: {
      numArgs: 2
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "mclass",
        mode: t.mode,
        mclass: nt(e[0]),
        body: J(e[1]),
        isCharacterBox: Y.isCharacterBox(e[1])
      };
    }
  });
  D({
    type: "mclass",
    names: [
      "\\stackrel",
      "\\overset",
      "\\underset"
    ],
    props: {
      numArgs: 2
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r, n = e[1], i = e[0], o;
      a !== "\\stackrel" ? o = nt(n) : o = "mrel";
      var u = {
        type: "op",
        mode: n.mode,
        limits: true,
        alwaysHandleSupSub: true,
        parentIsSupSub: false,
        symbol: false,
        suppressBaseShift: a !== "\\stackrel",
        body: J(n)
      }, m = {
        type: "supsub",
        mode: i.mode,
        base: u,
        sup: a === "\\underset" ? null : i,
        sub: a === "\\underset" ? i : null
      };
      return {
        type: "mclass",
        mode: t.mode,
        mclass: o,
        body: [
          m
        ],
        isCharacterBox: Y.isCharacterBox(m)
      };
    },
    htmlBuilder: Ha,
    mathmlBuilder: La
  });
  D({
    type: "pmb",
    names: [
      "\\pmb"
    ],
    props: {
      numArgs: 1,
      allowedInText: true
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "pmb",
        mode: t.mode,
        mclass: nt(e[0]),
        body: J(e[0])
      };
    },
    htmlBuilder(r, e) {
      var t = t0(r.body, e, true), a = w.makeSpan([
        r.mclass
      ], t, e);
      return a.style.textShadow = "0.02em 0.01em 0.04px", a;
    },
    mathmlBuilder(r, e) {
      var t = m0(r.body, e), a = new A.MathNode("mstyle", t);
      return a.setAttribute("style", "text-shadow: 0.02em 0.01em 0.04px"), a;
    }
  });
  var Ui = {
    ">": "\\\\cdrightarrow",
    "<": "\\\\cdleftarrow",
    "=": "\\\\cdlongequal",
    A: "\\uparrow",
    V: "\\downarrow",
    "|": "\\Vert",
    ".": "no arrow"
  }, Hr = () => ({
    type: "styling",
    body: [],
    mode: "math",
    style: "display"
  }), Lr = (r) => r.type === "textord" && r.text === "@", $i = (r, e) => (r.type === "mathord" || r.type === "atom") && r.text === e;
  function Yi(r, e, t) {
    var a = Ui[r];
    switch (a) {
      case "\\\\cdrightarrow":
      case "\\\\cdleftarrow":
        return t.callFunction(a, [
          e[0]
        ], [
          e[1]
        ]);
      case "\\uparrow":
      case "\\downarrow": {
        var n = t.callFunction("\\\\cdleft", [
          e[0]
        ], []), i = {
          type: "atom",
          text: a,
          mode: "math",
          family: "rel"
        }, o = t.callFunction("\\Big", [
          i
        ], []), u = t.callFunction("\\\\cdright", [
          e[1]
        ], []), m = {
          type: "ordgroup",
          mode: "math",
          body: [
            n,
            o,
            u
          ]
        };
        return t.callFunction("\\\\cdparent", [
          m
        ], []);
      }
      case "\\\\cdlongequal":
        return t.callFunction("\\\\cdlongequal", [], []);
      case "\\Vert": {
        var d = {
          type: "textord",
          text: "\\Vert",
          mode: "math"
        };
        return t.callFunction("\\Big", [
          d
        ], []);
      }
      default:
        return {
          type: "textord",
          text: " ",
          mode: "math"
        };
    }
  }
  function Xi(r) {
    var e = [];
    for (r.gullet.beginGroup(), r.gullet.macros.set("\\cr", "\\\\\\relax"), r.gullet.beginGroup(); ; ) {
      e.push(r.parseExpression(false, "\\\\")), r.gullet.endGroup(), r.gullet.beginGroup();
      var t = r.fetch().text;
      if (t === "&" || t === "\\\\") r.consume();
      else if (t === "\\end") {
        e[e.length - 1].length === 0 && e.pop();
        break;
      } else throw new T("Expected \\\\ or \\cr or \\end", r.nextToken);
    }
    for (var a = [], n = [
      a
    ], i = 0; i < e.length; i++) {
      for (var o = e[i], u = Hr(), m = 0; m < o.length; m++) if (!Lr(o[m])) u.body.push(o[m]);
      else {
        a.push(u), m += 1;
        var d = rr(o[m]).text, p = new Array(2);
        if (p[0] = {
          type: "ordgroup",
          mode: "math",
          body: []
        }, p[1] = {
          type: "ordgroup",
          mode: "math",
          body: []
        }, !("=|.".indexOf(d) > -1)) if ("<>AV".indexOf(d) > -1) for (var b = 0; b < 2; b++) {
          for (var y = true, x = m + 1; x < o.length; x++) {
            if ($i(o[x], d)) {
              y = false, m = x;
              break;
            }
            if (Lr(o[x])) throw new T("Missing a " + d + " character to complete a CD arrow.", o[x]);
            p[b].body.push(o[x]);
          }
          if (y) throw new T("Missing a " + d + " character to complete a CD arrow.", o[m]);
        }
        else throw new T('Expected one of "<>AV=|." after @', o[m]);
        var k = Yi(d, p, r), S = {
          type: "styling",
          body: [
            k
          ],
          mode: "math",
          style: "display"
        };
        a.push(S), u = Hr();
      }
      i % 2 === 0 ? a.push(u) : a.shift(), a = [], n.push(a);
    }
    r.gullet.endGroup(), r.gullet.endGroup();
    var z = new Array(n[0].length).fill({
      type: "align",
      align: "c",
      pregap: 0.25,
      postgap: 0.25
    });
    return {
      type: "array",
      mode: "math",
      body: n,
      arraystretch: 1,
      addJot: true,
      rowGaps: [
        null
      ],
      cols: z,
      colSeparationType: "CD",
      hLinesBeforeRow: new Array(n.length + 1).fill([])
    };
  }
  D({
    type: "cdlabel",
    names: [
      "\\\\cdleft",
      "\\\\cdright"
    ],
    props: {
      numArgs: 1
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r;
      return {
        type: "cdlabel",
        mode: t.mode,
        side: a.slice(4),
        label: e[0]
      };
    },
    htmlBuilder(r, e) {
      var t = e.havingStyle(e.style.sup()), a = w.wrapFragment($(r.label, t, e), e);
      return a.classes.push("cd-label-" + r.side), a.style.bottom = C(0.8 - a.depth), a.height = 0, a.depth = 0, a;
    },
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mrow", [
        X(r.label, e)
      ]);
      return t = new A.MathNode("mpadded", [
        t
      ]), t.setAttribute("width", "0"), r.side === "left" && t.setAttribute("lspace", "-1width"), t.setAttribute("voffset", "0.7em"), t = new A.MathNode("mstyle", [
        t
      ]), t.setAttribute("displaystyle", "false"), t.setAttribute("scriptlevel", "1"), t;
    }
  });
  D({
    type: "cdlabelparent",
    names: [
      "\\\\cdparent"
    ],
    props: {
      numArgs: 1
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "cdlabelparent",
        mode: t.mode,
        fragment: e[0]
      };
    },
    htmlBuilder(r, e) {
      var t = w.wrapFragment($(r.fragment, e), e);
      return t.classes.push("cd-vert-arrow"), t;
    },
    mathmlBuilder(r, e) {
      return new A.MathNode("mrow", [
        X(r.fragment, e)
      ]);
    }
  });
  D({
    type: "textord",
    names: [
      "\\@char"
    ],
    props: {
      numArgs: 1,
      allowedInText: true
    },
    handler(r, e) {
      for (var { parser: t } = r, a = G(e[0], "ordgroup"), n = a.body, i = "", o = 0; o < n.length; o++) {
        var u = G(n[o], "textord");
        i += u.text;
      }
      var m = parseInt(i), d;
      if (isNaN(m)) throw new T("\\@char has non-numeric argument " + i);
      if (m < 0 || m >= 1114111) throw new T("\\@char with invalid code point " + i);
      return m <= 65535 ? d = String.fromCharCode(m) : (m -= 65536, d = String.fromCharCode((m >> 10) + 55296, (m & 1023) + 56320)), {
        type: "textord",
        mode: t.mode,
        text: d
      };
    }
  });
  var Pa = (r, e) => {
    var t = t0(r.body, e.withColor(r.color), false);
    return w.makeFragment(t);
  }, Va = (r, e) => {
    var t = m0(r.body, e.withColor(r.color)), a = new A.MathNode("mstyle", t);
    return a.setAttribute("mathcolor", r.color), a;
  };
  D({
    type: "color",
    names: [
      "\\textcolor"
    ],
    props: {
      numArgs: 2,
      allowedInText: true,
      argTypes: [
        "color",
        "original"
      ]
    },
    handler(r, e) {
      var { parser: t } = r, a = G(e[0], "color-token").color, n = e[1];
      return {
        type: "color",
        mode: t.mode,
        color: a,
        body: J(n)
      };
    },
    htmlBuilder: Pa,
    mathmlBuilder: Va
  });
  D({
    type: "color",
    names: [
      "\\color"
    ],
    props: {
      numArgs: 1,
      allowedInText: true,
      argTypes: [
        "color"
      ]
    },
    handler(r, e) {
      var { parser: t, breakOnTokenText: a } = r, n = G(e[0], "color-token").color;
      t.gullet.macros.set("\\current@color", n);
      var i = t.parseExpression(true, a);
      return {
        type: "color",
        mode: t.mode,
        color: n,
        body: i
      };
    },
    htmlBuilder: Pa,
    mathmlBuilder: Va
  });
  D({
    type: "cr",
    names: [
      "\\\\"
    ],
    props: {
      numArgs: 0,
      numOptionalArgs: 0,
      allowedInText: true
    },
    handler(r, e, t) {
      var { parser: a } = r, n = a.gullet.future().text === "[" ? a.parseSizeGroup(true) : null, i = !a.settings.displayMode || !a.settings.useStrictBehavior("newLineInDisplayMode", "In LaTeX, \\\\ or \\newline does nothing in display mode");
      return {
        type: "cr",
        mode: a.mode,
        newLine: i,
        size: n && G(n, "size").value
      };
    },
    htmlBuilder(r, e) {
      var t = w.makeSpan([
        "mspace"
      ], [], e);
      return r.newLine && (t.classes.push("newline"), r.size && (t.style.marginTop = C(K(r.size, e)))), t;
    },
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mspace");
      return r.newLine && (t.setAttribute("linebreak", "newline"), r.size && t.setAttribute("height", C(K(r.size, e)))), t;
    }
  });
  var $t = {
    "\\global": "\\global",
    "\\long": "\\\\globallong",
    "\\\\globallong": "\\\\globallong",
    "\\def": "\\gdef",
    "\\gdef": "\\gdef",
    "\\edef": "\\xdef",
    "\\xdef": "\\xdef",
    "\\let": "\\\\globallet",
    "\\futurelet": "\\\\globalfuture"
  }, Ga = (r) => {
    var e = r.text;
    if (/^(?:[\\{}$&#^_]|EOF)$/.test(e)) throw new T("Expected a control sequence", r);
    return e;
  }, Wi = (r) => {
    var e = r.gullet.popToken();
    return e.text === "=" && (e = r.gullet.popToken(), e.text === " " && (e = r.gullet.popToken())), e;
  }, Ua = (r, e, t, a) => {
    var n = r.gullet.macros.get(t.text);
    n == null && (t.noexpand = true, n = {
      tokens: [
        t
      ],
      numArgs: 0,
      unexpandable: !r.gullet.isExpandable(t.text)
    }), r.gullet.macros.set(e, n, a);
  };
  D({
    type: "internal",
    names: [
      "\\global",
      "\\long",
      "\\\\globallong"
    ],
    props: {
      numArgs: 0,
      allowedInText: true
    },
    handler(r) {
      var { parser: e, funcName: t } = r;
      e.consumeSpaces();
      var a = e.fetch();
      if ($t[a.text]) return (t === "\\global" || t === "\\\\globallong") && (a.text = $t[a.text]), G(e.parseFunction(), "internal");
      throw new T("Invalid token after macro prefix", a);
    }
  });
  D({
    type: "internal",
    names: [
      "\\def",
      "\\gdef",
      "\\edef",
      "\\xdef"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      primitive: true
    },
    handler(r) {
      var { parser: e, funcName: t } = r, a = e.gullet.popToken(), n = a.text;
      if (/^(?:[\\{}$&#^_]|EOF)$/.test(n)) throw new T("Expected a control sequence", a);
      for (var i = 0, o, u = [
        []
      ]; e.gullet.future().text !== "{"; ) if (a = e.gullet.popToken(), a.text === "#") {
        if (e.gullet.future().text === "{") {
          o = e.gullet.future(), u[i].push("{");
          break;
        }
        if (a = e.gullet.popToken(), !/^[1-9]$/.test(a.text)) throw new T('Invalid argument number "' + a.text + '"');
        if (parseInt(a.text) !== i + 1) throw new T('Argument number "' + a.text + '" out of order');
        i++, u.push([]);
      } else {
        if (a.text === "EOF") throw new T("Expected a macro definition");
        u[i].push(a.text);
      }
      var { tokens: m } = e.gullet.consumeArg();
      return o && m.unshift(o), (t === "\\edef" || t === "\\xdef") && (m = e.gullet.expandTokens(m), m.reverse()), e.gullet.macros.set(n, {
        tokens: m,
        numArgs: i,
        delimiters: u
      }, t === $t[t]), {
        type: "internal",
        mode: e.mode
      };
    }
  });
  D({
    type: "internal",
    names: [
      "\\let",
      "\\\\globallet"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      primitive: true
    },
    handler(r) {
      var { parser: e, funcName: t } = r, a = Ga(e.gullet.popToken());
      e.gullet.consumeSpaces();
      var n = Wi(e);
      return Ua(e, a, n, t === "\\\\globallet"), {
        type: "internal",
        mode: e.mode
      };
    }
  });
  D({
    type: "internal",
    names: [
      "\\futurelet",
      "\\\\globalfuture"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      primitive: true
    },
    handler(r) {
      var { parser: e, funcName: t } = r, a = Ga(e.gullet.popToken()), n = e.gullet.popToken(), i = e.gullet.popToken();
      return Ua(e, a, i, t === "\\\\globalfuture"), e.gullet.pushToken(i), e.gullet.pushToken(n), {
        type: "internal",
        mode: e.mode
      };
    }
  });
  var ke = function(e, t, a) {
    var n = W.math[e] && W.math[e].replace, i = _t(n || e, t, a);
    if (!i) throw new Error("Unsupported symbol " + e + " and font size " + t + ".");
    return i;
  }, nr = function(e, t, a, n) {
    var i = a.havingBaseStyle(t), o = w.makeSpan(n.concat(i.sizingClasses(a)), [
      e
    ], a), u = i.sizeMultiplier / a.sizeMultiplier;
    return o.height *= u, o.depth *= u, o.maxFontSize = i.sizeMultiplier, o;
  }, $a = function(e, t, a) {
    var n = t.havingBaseStyle(a), i = (1 - t.sizeMultiplier / n.sizeMultiplier) * t.fontMetrics().axisHeight;
    e.classes.push("delimcenter"), e.style.top = C(i), e.height -= i, e.depth += i;
  }, Zi = function(e, t, a, n, i, o) {
    var u = w.makeSymbol(e, "Main-Regular", i, n), m = nr(u, t, n, o);
    return a && $a(m, n, t), m;
  }, ji = function(e, t, a, n) {
    return w.makeSymbol(e, "Size" + t + "-Regular", a, n);
  }, Ya = function(e, t, a, n, i, o) {
    var u = ji(e, t, i, n), m = nr(w.makeSpan([
      "delimsizing",
      "size" + t
    ], [
      u
    ], n), I.TEXT, n, o);
    return a && $a(m, n, I.TEXT), m;
  }, St = function(e, t, a) {
    var n;
    t === "Size1-Regular" ? n = "delim-size1" : n = "delim-size4";
    var i = w.makeSpan([
      "delimsizinginner",
      n
    ], [
      w.makeSpan([], [
        w.makeSymbol(e, t, a)
      ])
    ]);
    return {
      type: "elem",
      elem: i
    };
  }, Mt = function(e, t, a) {
    var n = T0["Size4-Regular"][e.charCodeAt(0)] ? T0["Size4-Regular"][e.charCodeAt(0)][4] : T0["Size1-Regular"][e.charCodeAt(0)][4], i = new _0("inner", ei(e, Math.round(1e3 * t))), o = new H0([
      i
    ], {
      width: C(n),
      height: C(t),
      style: "width:" + C(n),
      viewBox: "0 0 " + 1e3 * n + " " + Math.round(1e3 * t),
      preserveAspectRatio: "xMinYMin"
    }), u = w.makeSvgSpan([], [
      o
    ], a);
    return u.height = t, u.style.height = C(t), u.style.width = C(n), {
      type: "elem",
      elem: u
    };
  }, Yt = 8e-3, Ve = {
    type: "kern",
    size: -1 * Yt
  }, Ki = [
    "|",
    "\\lvert",
    "\\rvert",
    "\\vert"
  ], _i = [
    "\\|",
    "\\lVert",
    "\\rVert",
    "\\Vert"
  ], Xa = function(e, t, a, n, i, o) {
    var u, m, d, p, b = "", y = 0;
    u = d = p = e, m = null;
    var x = "Size1-Regular";
    e === "\\uparrow" ? d = p = "\u23D0" : e === "\\Uparrow" ? d = p = "\u2016" : e === "\\downarrow" ? u = d = "\u23D0" : e === "\\Downarrow" ? u = d = "\u2016" : e === "\\updownarrow" ? (u = "\\uparrow", d = "\u23D0", p = "\\downarrow") : e === "\\Updownarrow" ? (u = "\\Uparrow", d = "\u2016", p = "\\Downarrow") : Ki.includes(e) ? (d = "\u2223", b = "vert", y = 333) : _i.includes(e) ? (d = "\u2225", b = "doublevert", y = 556) : e === "[" || e === "\\lbrack" ? (u = "\u23A1", d = "\u23A2", p = "\u23A3", x = "Size4-Regular", b = "lbrack", y = 667) : e === "]" || e === "\\rbrack" ? (u = "\u23A4", d = "\u23A5", p = "\u23A6", x = "Size4-Regular", b = "rbrack", y = 667) : e === "\\lfloor" || e === "\u230A" ? (d = u = "\u23A2", p = "\u23A3", x = "Size4-Regular", b = "lfloor", y = 667) : e === "\\lceil" || e === "\u2308" ? (u = "\u23A1", d = p = "\u23A2", x = "Size4-Regular", b = "lceil", y = 667) : e === "\\rfloor" || e === "\u230B" ? (d = u = "\u23A5", p = "\u23A6", x = "Size4-Regular", b = "rfloor", y = 667) : e === "\\rceil" || e === "\u2309" ? (u = "\u23A4", d = p = "\u23A5", x = "Size4-Regular", b = "rceil", y = 667) : e === "(" || e === "\\lparen" ? (u = "\u239B", d = "\u239C", p = "\u239D", x = "Size4-Regular", b = "lparen", y = 875) : e === ")" || e === "\\rparen" ? (u = "\u239E", d = "\u239F", p = "\u23A0", x = "Size4-Regular", b = "rparen", y = 875) : e === "\\{" || e === "\\lbrace" ? (u = "\u23A7", m = "\u23A8", p = "\u23A9", d = "\u23AA", x = "Size4-Regular") : e === "\\}" || e === "\\rbrace" ? (u = "\u23AB", m = "\u23AC", p = "\u23AD", d = "\u23AA", x = "Size4-Regular") : e === "\\lgroup" || e === "\u27EE" ? (u = "\u23A7", p = "\u23A9", d = "\u23AA", x = "Size4-Regular") : e === "\\rgroup" || e === "\u27EF" ? (u = "\u23AB", p = "\u23AD", d = "\u23AA", x = "Size4-Regular") : e === "\\lmoustache" || e === "\u23B0" ? (u = "\u23A7", p = "\u23AD", d = "\u23AA", x = "Size4-Regular") : (e === "\\rmoustache" || e === "\u23B1") && (u = "\u23AB", p = "\u23A9", d = "\u23AA", x = "Size4-Regular");
    var k = ke(u, x, i), S = k.height + k.depth, z = ke(d, x, i), B = z.height + z.depth, N = ke(p, x, i), E = N.height + N.depth, H = 0, O = 1;
    if (m !== null) {
      var L = ke(m, x, i);
      H = L.height + L.depth, O = 2;
    }
    var V = S + E + H, U = Math.max(0, Math.ceil((t - V) / (O * B))), P = V + U * O * B, x0 = n.fontMetrics().axisHeight;
    a && (x0 *= n.sizeMultiplier);
    var r0 = P / 2 - x0, e0 = [];
    if (b.length > 0) {
      var ee = P - S - E, o0 = Math.round(P * 1e3), w0 = ti(b, Math.round(ee * 1e3)), G0 = new _0(b, w0), ne = (y / 1e3).toFixed(3) + "em", ie = (o0 / 1e3).toFixed(3) + "em", ot = new H0([
        G0
      ], {
        width: ne,
        height: ie,
        viewBox: "0 0 " + y + " " + o0
      }), U0 = w.makeSvgSpan([], [
        ot
      ], n);
      U0.height = o0 / 1e3, U0.style.width = ne, U0.style.height = ie, e0.push({
        type: "elem",
        elem: U0
      });
    } else {
      if (e0.push(St(p, x, i)), e0.push(Ve), m === null) {
        var $0 = P - S - E + 2 * Yt;
        e0.push(Mt(d, $0, n));
      } else {
        var v0 = (P - S - E - H) / 2 + 2 * Yt;
        e0.push(Mt(d, v0, n)), e0.push(Ve), e0.push(St(m, x, i)), e0.push(Ve), e0.push(Mt(d, v0, n));
      }
      e0.push(Ve), e0.push(St(u, x, i));
    }
    var be = n.havingBaseStyle(I.TEXT), ut = w.makeVList({
      positionType: "bottom",
      positionData: r0,
      children: e0
    }, be);
    return nr(w.makeSpan([
      "delimsizing",
      "mult"
    ], [
      ut
    ], be), I.TEXT, n, o);
  }, zt = 80, At = 0.08, Tt = function(e, t, a, n, i) {
    var o = Qn(e, n, a), u = new _0(e, o), m = new H0([
      u
    ], {
      width: "400em",
      height: C(t),
      viewBox: "0 0 400000 " + a,
      preserveAspectRatio: "xMinYMin slice"
    });
    return w.makeSvgSpan([
      "hide-tail"
    ], [
      m
    ], i);
  }, Ji = function(e, t) {
    var a = t.havingBaseSizing(), n = Ka("\\surd", e * a.sizeMultiplier, ja, a), i = a.sizeMultiplier, o = Math.max(0, t.minRuleThickness - t.fontMetrics().sqrtRuleThickness), u, m = 0, d = 0, p = 0, b;
    return n.type === "small" ? (p = 1e3 + 1e3 * o + zt, e < 1 ? i = 1 : e < 1.4 && (i = 0.7), m = (1 + o + At) / i, d = (1 + o) / i, u = Tt("sqrtMain", m, p, o, t), u.style.minWidth = "0.853em", b = 0.833 / i) : n.type === "large" ? (p = (1e3 + zt) * Me[n.size], d = (Me[n.size] + o) / i, m = (Me[n.size] + o + At) / i, u = Tt("sqrtSize" + n.size, m, p, o, t), u.style.minWidth = "1.02em", b = 1 / i) : (m = e + o + At, d = e + o, p = Math.floor(1e3 * e + o) + zt, u = Tt("sqrtTall", m, p, o, t), u.style.minWidth = "0.742em", b = 1.056), u.height = d, u.style.height = C(m), {
      span: u,
      advanceWidth: b,
      ruleWidth: (t.fontMetrics().sqrtRuleThickness + o) * i
    };
  }, Wa = [
    "(",
    "\\lparen",
    ")",
    "\\rparen",
    "[",
    "\\lbrack",
    "]",
    "\\rbrack",
    "\\{",
    "\\lbrace",
    "\\}",
    "\\rbrace",
    "\\lfloor",
    "\\rfloor",
    "\u230A",
    "\u230B",
    "\\lceil",
    "\\rceil",
    "\u2308",
    "\u2309",
    "\\surd"
  ], Qi = [
    "\\uparrow",
    "\\downarrow",
    "\\updownarrow",
    "\\Uparrow",
    "\\Downarrow",
    "\\Updownarrow",
    "|",
    "\\|",
    "\\vert",
    "\\Vert",
    "\\lvert",
    "\\rvert",
    "\\lVert",
    "\\rVert",
    "\\lgroup",
    "\\rgroup",
    "\u27EE",
    "\u27EF",
    "\\lmoustache",
    "\\rmoustache",
    "\u23B0",
    "\u23B1"
  ], Za = [
    "<",
    ">",
    "\\langle",
    "\\rangle",
    "/",
    "\\backslash",
    "\\lt",
    "\\gt"
  ], Me = [
    0,
    1.2,
    1.8,
    2.4,
    3
  ], e4 = function(e, t, a, n, i) {
    if (e === "<" || e === "\\lt" || e === "\u27E8" ? e = "\\langle" : (e === ">" || e === "\\gt" || e === "\u27E9") && (e = "\\rangle"), Wa.includes(e) || Za.includes(e)) return Ya(e, t, false, a, n, i);
    if (Qi.includes(e)) return Xa(e, Me[t], false, a, n, i);
    throw new T("Illegal delimiter: '" + e + "'");
  }, t4 = [
    {
      type: "small",
      style: I.SCRIPTSCRIPT
    },
    {
      type: "small",
      style: I.SCRIPT
    },
    {
      type: "small",
      style: I.TEXT
    },
    {
      type: "large",
      size: 1
    },
    {
      type: "large",
      size: 2
    },
    {
      type: "large",
      size: 3
    },
    {
      type: "large",
      size: 4
    }
  ], r4 = [
    {
      type: "small",
      style: I.SCRIPTSCRIPT
    },
    {
      type: "small",
      style: I.SCRIPT
    },
    {
      type: "small",
      style: I.TEXT
    },
    {
      type: "stack"
    }
  ], ja = [
    {
      type: "small",
      style: I.SCRIPTSCRIPT
    },
    {
      type: "small",
      style: I.SCRIPT
    },
    {
      type: "small",
      style: I.TEXT
    },
    {
      type: "large",
      size: 1
    },
    {
      type: "large",
      size: 2
    },
    {
      type: "large",
      size: 3
    },
    {
      type: "large",
      size: 4
    },
    {
      type: "stack"
    }
  ], a4 = function(e) {
    if (e.type === "small") return "Main-Regular";
    if (e.type === "large") return "Size" + e.size + "-Regular";
    if (e.type === "stack") return "Size4-Regular";
    throw new Error("Add support for delim type '" + e.type + "' here.");
  }, Ka = function(e, t, a, n) {
    for (var i = Math.min(2, 3 - n.style.size), o = i; o < a.length && a[o].type !== "stack"; o++) {
      var u = ke(e, a4(a[o]), "math"), m = u.height + u.depth;
      if (a[o].type === "small") {
        var d = n.havingBaseStyle(a[o].style);
        m *= d.sizeMultiplier;
      }
      if (m > t) return a[o];
    }
    return a[a.length - 1];
  }, _a = function(e, t, a, n, i, o) {
    e === "<" || e === "\\lt" || e === "\u27E8" ? e = "\\langle" : (e === ">" || e === "\\gt" || e === "\u27E9") && (e = "\\rangle");
    var u;
    Za.includes(e) ? u = t4 : Wa.includes(e) ? u = ja : u = r4;
    var m = Ka(e, t, u, n);
    return m.type === "small" ? Zi(e, m.style, a, n, i, o) : m.type === "large" ? Ya(e, m.size, a, n, i, o) : Xa(e, t, a, n, i, o);
  }, n4 = function(e, t, a, n, i, o) {
    var u = n.fontMetrics().axisHeight * n.sizeMultiplier, m = 901, d = 5 / n.fontMetrics().ptPerEm, p = Math.max(t - u, a + u), b = Math.max(p / 500 * m, 2 * p - d);
    return _a(e, b, true, n, i, o);
  }, F0 = {
    sqrtImage: Ji,
    sizedDelim: e4,
    sizeToMaxHeight: Me,
    customSizedDelim: _a,
    leftRightDelim: n4
  }, Pr = {
    "\\bigl": {
      mclass: "mopen",
      size: 1
    },
    "\\Bigl": {
      mclass: "mopen",
      size: 2
    },
    "\\biggl": {
      mclass: "mopen",
      size: 3
    },
    "\\Biggl": {
      mclass: "mopen",
      size: 4
    },
    "\\bigr": {
      mclass: "mclose",
      size: 1
    },
    "\\Bigr": {
      mclass: "mclose",
      size: 2
    },
    "\\biggr": {
      mclass: "mclose",
      size: 3
    },
    "\\Biggr": {
      mclass: "mclose",
      size: 4
    },
    "\\bigm": {
      mclass: "mrel",
      size: 1
    },
    "\\Bigm": {
      mclass: "mrel",
      size: 2
    },
    "\\biggm": {
      mclass: "mrel",
      size: 3
    },
    "\\Biggm": {
      mclass: "mrel",
      size: 4
    },
    "\\big": {
      mclass: "mord",
      size: 1
    },
    "\\Big": {
      mclass: "mord",
      size: 2
    },
    "\\bigg": {
      mclass: "mord",
      size: 3
    },
    "\\Bigg": {
      mclass: "mord",
      size: 4
    }
  }, i4 = [
    "(",
    "\\lparen",
    ")",
    "\\rparen",
    "[",
    "\\lbrack",
    "]",
    "\\rbrack",
    "\\{",
    "\\lbrace",
    "\\}",
    "\\rbrace",
    "\\lfloor",
    "\\rfloor",
    "\u230A",
    "\u230B",
    "\\lceil",
    "\\rceil",
    "\u2308",
    "\u2309",
    "<",
    ">",
    "\\langle",
    "\u27E8",
    "\\rangle",
    "\u27E9",
    "\\lt",
    "\\gt",
    "\\lvert",
    "\\rvert",
    "\\lVert",
    "\\rVert",
    "\\lgroup",
    "\\rgroup",
    "\u27EE",
    "\u27EF",
    "\\lmoustache",
    "\\rmoustache",
    "\u23B0",
    "\u23B1",
    "/",
    "\\backslash",
    "|",
    "\\vert",
    "\\|",
    "\\Vert",
    "\\uparrow",
    "\\Uparrow",
    "\\downarrow",
    "\\Downarrow",
    "\\updownarrow",
    "\\Updownarrow",
    "."
  ];
  function it(r, e) {
    var t = at(r);
    if (t && i4.includes(t.text)) return t;
    throw t ? new T("Invalid delimiter '" + t.text + "' after '" + e.funcName + "'", r) : new T("Invalid delimiter type '" + r.type + "'", r);
  }
  D({
    type: "delimsizing",
    names: [
      "\\bigl",
      "\\Bigl",
      "\\biggl",
      "\\Biggl",
      "\\bigr",
      "\\Bigr",
      "\\biggr",
      "\\Biggr",
      "\\bigm",
      "\\Bigm",
      "\\biggm",
      "\\Biggm",
      "\\big",
      "\\Big",
      "\\bigg",
      "\\Bigg"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "primitive"
      ]
    },
    handler: (r, e) => {
      var t = it(e[0], r);
      return {
        type: "delimsizing",
        mode: r.parser.mode,
        size: Pr[r.funcName].size,
        mclass: Pr[r.funcName].mclass,
        delim: t.text
      };
    },
    htmlBuilder: (r, e) => r.delim === "." ? w.makeSpan([
      r.mclass
    ]) : F0.sizedDelim(r.delim, r.size, e, r.mode, [
      r.mclass
    ]),
    mathmlBuilder: (r) => {
      var e = [];
      r.delim !== "." && e.push(y0(r.delim, r.mode));
      var t = new A.MathNode("mo", e);
      r.mclass === "mopen" || r.mclass === "mclose" ? t.setAttribute("fence", "true") : t.setAttribute("fence", "false"), t.setAttribute("stretchy", "true");
      var a = C(F0.sizeToMaxHeight[r.size]);
      return t.setAttribute("minsize", a), t.setAttribute("maxsize", a), t;
    }
  });
  function Vr(r) {
    if (!r.body) throw new Error("Bug: The leftright ParseNode wasn't fully parsed.");
  }
  D({
    type: "leftright-right",
    names: [
      "\\right"
    ],
    props: {
      numArgs: 1,
      primitive: true
    },
    handler: (r, e) => {
      var t = r.parser.gullet.macros.get("\\current@color");
      if (t && typeof t != "string") throw new T("\\current@color set to non-string in \\right");
      return {
        type: "leftright-right",
        mode: r.parser.mode,
        delim: it(e[0], r).text,
        color: t
      };
    }
  });
  D({
    type: "leftright",
    names: [
      "\\left"
    ],
    props: {
      numArgs: 1,
      primitive: true
    },
    handler: (r, e) => {
      var t = it(e[0], r), a = r.parser;
      ++a.leftrightDepth;
      var n = a.parseExpression(false);
      --a.leftrightDepth, a.expect("\\right", false);
      var i = G(a.parseFunction(), "leftright-right");
      return {
        type: "leftright",
        mode: a.mode,
        body: n,
        left: t.text,
        right: i.delim,
        rightColor: i.color
      };
    },
    htmlBuilder: (r, e) => {
      Vr(r);
      for (var t = t0(r.body, e, true, [
        "mopen",
        "mclose"
      ]), a = 0, n = 0, i = false, o = 0; o < t.length; o++) t[o].isMiddle ? i = true : (a = Math.max(t[o].height, a), n = Math.max(t[o].depth, n));
      a *= e.sizeMultiplier, n *= e.sizeMultiplier;
      var u;
      if (r.left === "." ? u = Ae(e, [
        "mopen"
      ]) : u = F0.leftRightDelim(r.left, a, n, e, r.mode, [
        "mopen"
      ]), t.unshift(u), i) for (var m = 1; m < t.length; m++) {
        var d = t[m], p = d.isMiddle;
        p && (t[m] = F0.leftRightDelim(p.delim, a, n, p.options, r.mode, []));
      }
      var b;
      if (r.right === ".") b = Ae(e, [
        "mclose"
      ]);
      else {
        var y = r.rightColor ? e.withColor(r.rightColor) : e;
        b = F0.leftRightDelim(r.right, a, n, y, r.mode, [
          "mclose"
        ]);
      }
      return t.push(b), w.makeSpan([
        "minner"
      ], t, e);
    },
    mathmlBuilder: (r, e) => {
      Vr(r);
      var t = m0(r.body, e);
      if (r.left !== ".") {
        var a = new A.MathNode("mo", [
          y0(r.left, r.mode)
        ]);
        a.setAttribute("fence", "true"), t.unshift(a);
      }
      if (r.right !== ".") {
        var n = new A.MathNode("mo", [
          y0(r.right, r.mode)
        ]);
        n.setAttribute("fence", "true"), r.rightColor && n.setAttribute("mathcolor", r.rightColor), t.push(n);
      }
      return er(t);
    }
  });
  D({
    type: "middle",
    names: [
      "\\middle"
    ],
    props: {
      numArgs: 1,
      primitive: true
    },
    handler: (r, e) => {
      var t = it(e[0], r);
      if (!r.parser.leftrightDepth) throw new T("\\middle without preceding \\left", t);
      return {
        type: "middle",
        mode: r.parser.mode,
        delim: t.text
      };
    },
    htmlBuilder: (r, e) => {
      var t;
      if (r.delim === ".") t = Ae(e, []);
      else {
        t = F0.sizedDelim(r.delim, 1, e, r.mode, []);
        var a = {
          delim: r.delim,
          options: e
        };
        t.isMiddle = a;
      }
      return t;
    },
    mathmlBuilder: (r, e) => {
      var t = r.delim === "\\vert" || r.delim === "|" ? y0("|", "text") : y0(r.delim, r.mode), a = new A.MathNode("mo", [
        t
      ]);
      return a.setAttribute("fence", "true"), a.setAttribute("lspace", "0.05em"), a.setAttribute("rspace", "0.05em"), a;
    }
  });
  var ir = (r, e) => {
    var t = w.wrapFragment($(r.body, e), e), a = r.label.slice(1), n = e.sizeMultiplier, i, o = 0, u = Y.isCharacterBox(r.body);
    if (a === "sout") i = w.makeSpan([
      "stretchy",
      "sout"
    ]), i.height = e.fontMetrics().defaultRuleThickness / n, o = -0.5 * e.fontMetrics().xHeight;
    else if (a === "phase") {
      var m = K({
        number: 0.6,
        unit: "pt"
      }, e), d = K({
        number: 0.35,
        unit: "ex"
      }, e), p = e.havingBaseSizing();
      n = n / p.sizeMultiplier;
      var b = t.height + t.depth + m + d;
      t.style.paddingLeft = C(b / 2 + m);
      var y = Math.floor(1e3 * b * n), x = _n(y), k = new H0([
        new _0("phase", x)
      ], {
        width: "400em",
        height: C(y / 1e3),
        viewBox: "0 0 400000 " + y,
        preserveAspectRatio: "xMinYMin slice"
      });
      i = w.makeSvgSpan([
        "hide-tail"
      ], [
        k
      ], e), i.style.height = C(b), o = t.depth + m + d;
    } else {
      /cancel/.test(a) ? u || t.classes.push("cancel-pad") : a === "angl" ? t.classes.push("anglpad") : t.classes.push("boxpad");
      var S = 0, z = 0, B = 0;
      /box/.test(a) ? (B = Math.max(e.fontMetrics().fboxrule, e.minRuleThickness), S = e.fontMetrics().fboxsep + (a === "colorbox" ? 0 : B), z = S) : a === "angl" ? (B = Math.max(e.fontMetrics().defaultRuleThickness, e.minRuleThickness), S = 4 * B, z = Math.max(0, 0.25 - t.depth)) : (S = u ? 0.2 : 0, z = S), i = P0.encloseSpan(t, a, S, z, e), /fbox|boxed|fcolorbox/.test(a) ? (i.style.borderStyle = "solid", i.style.borderWidth = C(B)) : a === "angl" && B !== 0.049 && (i.style.borderTopWidth = C(B), i.style.borderRightWidth = C(B)), o = t.depth + z, r.backgroundColor && (i.style.backgroundColor = r.backgroundColor, r.borderColor && (i.style.borderColor = r.borderColor));
    }
    var N;
    if (r.backgroundColor) N = w.makeVList({
      positionType: "individualShift",
      children: [
        {
          type: "elem",
          elem: i,
          shift: o
        },
        {
          type: "elem",
          elem: t,
          shift: 0
        }
      ]
    }, e);
    else {
      var E = /cancel|phase/.test(a) ? [
        "svg-align"
      ] : [];
      N = w.makeVList({
        positionType: "individualShift",
        children: [
          {
            type: "elem",
            elem: t,
            shift: 0
          },
          {
            type: "elem",
            elem: i,
            shift: o,
            wrapperClasses: E
          }
        ]
      }, e);
    }
    return /cancel/.test(a) && (N.height = t.height, N.depth = t.depth), /cancel/.test(a) && !u ? w.makeSpan([
      "mord",
      "cancel-lap"
    ], [
      N
    ], e) : w.makeSpan([
      "mord"
    ], [
      N
    ], e);
  }, sr = (r, e) => {
    var t = 0, a = new A.MathNode(r.label.indexOf("colorbox") > -1 ? "mpadded" : "menclose", [
      X(r.body, e)
    ]);
    switch (r.label) {
      case "\\cancel":
        a.setAttribute("notation", "updiagonalstrike");
        break;
      case "\\bcancel":
        a.setAttribute("notation", "downdiagonalstrike");
        break;
      case "\\phase":
        a.setAttribute("notation", "phasorangle");
        break;
      case "\\sout":
        a.setAttribute("notation", "horizontalstrike");
        break;
      case "\\fbox":
        a.setAttribute("notation", "box");
        break;
      case "\\angl":
        a.setAttribute("notation", "actuarial");
        break;
      case "\\fcolorbox":
      case "\\colorbox":
        if (t = e.fontMetrics().fboxsep * e.fontMetrics().ptPerEm, a.setAttribute("width", "+" + 2 * t + "pt"), a.setAttribute("height", "+" + 2 * t + "pt"), a.setAttribute("lspace", t + "pt"), a.setAttribute("voffset", t + "pt"), r.label === "\\fcolorbox") {
          var n = Math.max(e.fontMetrics().fboxrule, e.minRuleThickness);
          a.setAttribute("style", "border: " + n + "em solid " + String(r.borderColor));
        }
        break;
      case "\\xcancel":
        a.setAttribute("notation", "updiagonalstrike downdiagonalstrike");
        break;
    }
    return r.backgroundColor && a.setAttribute("mathbackground", r.backgroundColor), a;
  };
  D({
    type: "enclose",
    names: [
      "\\colorbox"
    ],
    props: {
      numArgs: 2,
      allowedInText: true,
      argTypes: [
        "color",
        "text"
      ]
    },
    handler(r, e, t) {
      var { parser: a, funcName: n } = r, i = G(e[0], "color-token").color, o = e[1];
      return {
        type: "enclose",
        mode: a.mode,
        label: n,
        backgroundColor: i,
        body: o
      };
    },
    htmlBuilder: ir,
    mathmlBuilder: sr
  });
  D({
    type: "enclose",
    names: [
      "\\fcolorbox"
    ],
    props: {
      numArgs: 3,
      allowedInText: true,
      argTypes: [
        "color",
        "color",
        "text"
      ]
    },
    handler(r, e, t) {
      var { parser: a, funcName: n } = r, i = G(e[0], "color-token").color, o = G(e[1], "color-token").color, u = e[2];
      return {
        type: "enclose",
        mode: a.mode,
        label: n,
        backgroundColor: o,
        borderColor: i,
        body: u
      };
    },
    htmlBuilder: ir,
    mathmlBuilder: sr
  });
  D({
    type: "enclose",
    names: [
      "\\fbox"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "hbox"
      ],
      allowedInText: true
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "enclose",
        mode: t.mode,
        label: "\\fbox",
        body: e[0]
      };
    }
  });
  D({
    type: "enclose",
    names: [
      "\\cancel",
      "\\bcancel",
      "\\xcancel",
      "\\sout",
      "\\phase"
    ],
    props: {
      numArgs: 1
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r, n = e[0];
      return {
        type: "enclose",
        mode: t.mode,
        label: a,
        body: n
      };
    },
    htmlBuilder: ir,
    mathmlBuilder: sr
  });
  D({
    type: "enclose",
    names: [
      "\\angl"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "hbox"
      ],
      allowedInText: false
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "enclose",
        mode: t.mode,
        label: "\\angl",
        body: e[0]
      };
    }
  });
  var Ja = {};
  function D0(r) {
    for (var { type: e, names: t, props: a, handler: n, htmlBuilder: i, mathmlBuilder: o } = r, u = {
      type: e,
      numArgs: a.numArgs || 0,
      allowedInText: false,
      numOptionalArgs: 0,
      handler: n
    }, m = 0; m < t.length; ++m) Ja[t[m]] = u;
    i && (Ke[e] = i), o && (_e[e] = o);
  }
  var Qa = {};
  function c(r, e) {
    Qa[r] = e;
  }
  function Gr(r) {
    var e = [];
    r.consumeSpaces();
    var t = r.fetch().text;
    for (t === "\\relax" && (r.consume(), r.consumeSpaces(), t = r.fetch().text); t === "\\hline" || t === "\\hdashline"; ) r.consume(), e.push(t === "\\hdashline"), r.consumeSpaces(), t = r.fetch().text;
    return e;
  }
  var st = (r) => {
    var e = r.parser.settings;
    if (!e.displayMode) throw new T("{" + r.envName + "} can be used only in display mode.");
  };
  function lr(r) {
    if (r.indexOf("ed") === -1) return r.indexOf("*") === -1;
  }
  function Q0(r, e, t) {
    var { hskipBeforeAndAfter: a, addJot: n, cols: i, arraystretch: o, colSeparationType: u, autoTag: m, singleRow: d, emptySingleRow: p, maxNumCols: b, leqno: y } = e;
    if (r.gullet.beginGroup(), d || r.gullet.macros.set("\\cr", "\\\\\\relax"), !o) {
      var x = r.gullet.expandMacroAsText("\\arraystretch");
      if (x == null) o = 1;
      else if (o = parseFloat(x), !o || o < 0) throw new T("Invalid \\arraystretch: " + x);
    }
    r.gullet.beginGroup();
    var k = [], S = [
      k
    ], z = [], B = [], N = m != null ? [] : void 0;
    function E() {
      m && r.gullet.macros.set("\\@eqnsw", "1", true);
    }
    function H() {
      N && (r.gullet.macros.get("\\df@tag") ? (N.push(r.subparse([
        new d0("\\df@tag")
      ])), r.gullet.macros.set("\\df@tag", void 0, true)) : N.push(!!m && r.gullet.macros.get("\\@eqnsw") === "1"));
    }
    for (E(), B.push(Gr(r)); ; ) {
      var O = r.parseExpression(false, d ? "\\end" : "\\\\");
      r.gullet.endGroup(), r.gullet.beginGroup(), O = {
        type: "ordgroup",
        mode: r.mode,
        body: O
      }, t && (O = {
        type: "styling",
        mode: r.mode,
        style: t,
        body: [
          O
        ]
      }), k.push(O);
      var L = r.fetch().text;
      if (L === "&") {
        if (b && k.length === b) {
          if (d || u) throw new T("Too many tab characters: &", r.nextToken);
          r.settings.reportNonstrict("textEnv", "Too few columns specified in the {array} column argument.");
        }
        r.consume();
      } else if (L === "\\end") {
        H(), k.length === 1 && O.type === "styling" && O.body[0].body.length === 0 && (S.length > 1 || !p) && S.pop(), B.length < S.length + 1 && B.push([]);
        break;
      } else if (L === "\\\\") {
        r.consume();
        var V = void 0;
        r.gullet.future().text !== " " && (V = r.parseSizeGroup(true)), z.push(V ? V.value : null), H(), B.push(Gr(r)), k = [], S.push(k), E();
      } else throw new T("Expected & or \\\\ or \\cr or \\end", r.nextToken);
    }
    return r.gullet.endGroup(), r.gullet.endGroup(), {
      type: "array",
      mode: r.mode,
      addJot: n,
      arraystretch: o,
      body: S,
      cols: i,
      rowGaps: z,
      hskipBeforeAndAfter: a,
      hLinesBeforeRow: B,
      colSeparationType: u,
      tags: N,
      leqno: y
    };
  }
  function or(r) {
    return r.slice(0, 1) === "d" ? "display" : "text";
  }
  var N0 = function(e, t) {
    var a, n, i = e.body.length, o = e.hLinesBeforeRow, u = 0, m = new Array(i), d = [], p = Math.max(t.fontMetrics().arrayRuleWidth, t.minRuleThickness), b = 1 / t.fontMetrics().ptPerEm, y = 5 * b;
    if (e.colSeparationType && e.colSeparationType === "small") {
      var x = t.havingStyle(I.SCRIPT).sizeMultiplier;
      y = 0.2778 * (x / t.sizeMultiplier);
    }
    var k = e.colSeparationType === "CD" ? K({
      number: 3,
      unit: "ex"
    }, t) : 12 * b, S = 3 * b, z = e.arraystretch * k, B = 0.7 * z, N = 0.3 * z, E = 0;
    function H(qe) {
      for (var Ee = 0; Ee < qe.length; ++Ee) Ee > 0 && (E += 0.25), d.push({
        pos: E,
        isDashed: qe[Ee]
      });
    }
    for (H(o[0]), a = 0; a < e.body.length; ++a) {
      var O = e.body[a], L = B, V = N;
      u < O.length && (u = O.length);
      var U = new Array(O.length);
      for (n = 0; n < O.length; ++n) {
        var P = $(O[n], t);
        V < P.depth && (V = P.depth), L < P.height && (L = P.height), U[n] = P;
      }
      var x0 = e.rowGaps[a], r0 = 0;
      x0 && (r0 = K(x0, t), r0 > 0 && (r0 += N, V < r0 && (V = r0), r0 = 0)), e.addJot && (V += S), U.height = L, U.depth = V, E += L, U.pos = E, E += V + r0, m[a] = U, H(o[a + 1]);
    }
    var e0 = E / 2 + t.fontMetrics().axisHeight, ee = e.cols || [], o0 = [], w0, G0, ne = [];
    if (e.tags && e.tags.some((qe) => qe)) for (a = 0; a < i; ++a) {
      var ie = m[a], ot = ie.pos - e0, U0 = e.tags[a], $0 = void 0;
      U0 === true ? $0 = w.makeSpan([
        "eqn-num"
      ], [], t) : U0 === false ? $0 = w.makeSpan([], [], t) : $0 = w.makeSpan([], t0(U0, t, true), t), $0.depth = ie.depth, $0.height = ie.height, ne.push({
        type: "elem",
        elem: $0,
        shift: ot
      });
    }
    for (n = 0, G0 = 0; n < u || G0 < ee.length; ++n, ++G0) {
      for (var v0 = ee[G0] || {}, be = true; v0.type === "separator"; ) {
        if (be || (w0 = w.makeSpan([
          "arraycolsep"
        ], []), w0.style.width = C(t.fontMetrics().doubleRuleSep), o0.push(w0)), v0.separator === "|" || v0.separator === ":") {
          var ut = v0.separator === "|" ? "solid" : "dashed", se = w.makeSpan([
            "vertical-separator"
          ], [], t);
          se.style.height = C(E), se.style.borderRightWidth = C(p), se.style.borderRightStyle = ut, se.style.margin = "0 " + C(-p / 2);
          var vr = E - e0;
          vr && (se.style.verticalAlign = C(-vr)), o0.push(se);
        } else throw new T("Invalid separator type: " + v0.separator);
        G0++, v0 = ee[G0] || {}, be = false;
      }
      if (!(n >= u)) {
        var le = void 0;
        (n > 0 || e.hskipBeforeAndAfter) && (le = Y.deflt(v0.pregap, y), le !== 0 && (w0 = w.makeSpan([
          "arraycolsep"
        ], []), w0.style.width = C(le), o0.push(w0)));
        var oe = [];
        for (a = 0; a < i; ++a) {
          var De = m[a], Ne = De[n];
          if (Ne) {
            var p1 = De.pos - e0;
            Ne.depth = De.depth, Ne.height = De.height, oe.push({
              type: "elem",
              elem: Ne,
              shift: p1
            });
          }
        }
        oe = w.makeVList({
          positionType: "individualShift",
          children: oe
        }, t), oe = w.makeSpan([
          "col-align-" + (v0.align || "c")
        ], [
          oe
        ]), o0.push(oe), (n < u - 1 || e.hskipBeforeAndAfter) && (le = Y.deflt(v0.postgap, y), le !== 0 && (w0 = w.makeSpan([
          "arraycolsep"
        ], []), w0.style.width = C(le), o0.push(w0)));
      }
    }
    if (m = w.makeSpan([
      "mtable"
    ], o0), d.length > 0) {
      for (var g1 = w.makeLineSpan("hline", t, p), b1 = w.makeLineSpan("hdashline", t, p), ht = [
        {
          type: "elem",
          elem: m,
          shift: 0
        }
      ]; d.length > 0; ) {
        var pr = d.pop(), gr = pr.pos - e0;
        pr.isDashed ? ht.push({
          type: "elem",
          elem: b1,
          shift: gr
        }) : ht.push({
          type: "elem",
          elem: g1,
          shift: gr
        });
      }
      m = w.makeVList({
        positionType: "individualShift",
        children: ht
      }, t);
    }
    if (ne.length === 0) return w.makeSpan([
      "mord"
    ], [
      m
    ], t);
    var mt = w.makeVList({
      positionType: "individualShift",
      children: ne
    }, t);
    return mt = w.makeSpan([
      "tag"
    ], [
      mt
    ], t), w.makeFragment([
      m,
      mt
    ]);
  }, s4 = {
    c: "center ",
    l: "left ",
    r: "right "
  }, q0 = function(e, t) {
    for (var a = [], n = new A.MathNode("mtd", [], [
      "mtr-glue"
    ]), i = new A.MathNode("mtd", [], [
      "mml-eqn-num"
    ]), o = 0; o < e.body.length; o++) {
      for (var u = e.body[o], m = [], d = 0; d < u.length; d++) m.push(new A.MathNode("mtd", [
        X(u[d], t)
      ]));
      e.tags && e.tags[o] && (m.unshift(n), m.push(n), e.leqno ? m.unshift(i) : m.push(i)), a.push(new A.MathNode("mtr", m));
    }
    var p = new A.MathNode("mtable", a), b = e.arraystretch === 0.5 ? 0.1 : 0.16 + e.arraystretch - 1 + (e.addJot ? 0.09 : 0);
    p.setAttribute("rowspacing", C(b));
    var y = "", x = "";
    if (e.cols && e.cols.length > 0) {
      var k = e.cols, S = "", z = false, B = 0, N = k.length;
      k[0].type === "separator" && (y += "top ", B = 1), k[k.length - 1].type === "separator" && (y += "bottom ", N -= 1);
      for (var E = B; E < N; E++) k[E].type === "align" ? (x += s4[k[E].align], z && (S += "none "), z = true) : k[E].type === "separator" && z && (S += k[E].separator === "|" ? "solid " : "dashed ", z = false);
      p.setAttribute("columnalign", x.trim()), /[sd]/.test(S) && p.setAttribute("columnlines", S.trim());
    }
    if (e.colSeparationType === "align") {
      for (var H = e.cols || [], O = "", L = 1; L < H.length; L++) O += L % 2 ? "0em " : "1em ";
      p.setAttribute("columnspacing", O.trim());
    } else e.colSeparationType === "alignat" || e.colSeparationType === "gather" ? p.setAttribute("columnspacing", "0em") : e.colSeparationType === "small" ? p.setAttribute("columnspacing", "0.2778em") : e.colSeparationType === "CD" ? p.setAttribute("columnspacing", "0.5em") : p.setAttribute("columnspacing", "1em");
    var V = "", U = e.hLinesBeforeRow;
    y += U[0].length > 0 ? "left " : "", y += U[U.length - 1].length > 0 ? "right " : "";
    for (var P = 1; P < U.length - 1; P++) V += U[P].length === 0 ? "none " : U[P][0] ? "dashed " : "solid ";
    return /[sd]/.test(V) && p.setAttribute("rowlines", V.trim()), y !== "" && (p = new A.MathNode("menclose", [
      p
    ]), p.setAttribute("notation", y.trim())), e.arraystretch && e.arraystretch < 1 && (p = new A.MathNode("mstyle", [
      p
    ]), p.setAttribute("scriptlevel", "1")), p;
  }, e1 = function(e, t) {
    e.envName.indexOf("ed") === -1 && st(e);
    var a = [], n = e.envName.indexOf("at") > -1 ? "alignat" : "align", i = e.envName === "split", o = Q0(e.parser, {
      cols: a,
      addJot: true,
      autoTag: i ? void 0 : lr(e.envName),
      emptySingleRow: true,
      colSeparationType: n,
      maxNumCols: i ? 2 : void 0,
      leqno: e.parser.settings.leqno
    }, "display"), u, m = 0, d = {
      type: "ordgroup",
      mode: e.mode,
      body: []
    };
    if (t[0] && t[0].type === "ordgroup") {
      for (var p = "", b = 0; b < t[0].body.length; b++) {
        var y = G(t[0].body[b], "textord");
        p += y.text;
      }
      u = Number(p), m = u * 2;
    }
    var x = !m;
    o.body.forEach(function(B) {
      for (var N = 1; N < B.length; N += 2) {
        var E = G(B[N], "styling"), H = G(E.body[0], "ordgroup");
        H.body.unshift(d);
      }
      if (x) m < B.length && (m = B.length);
      else {
        var O = B.length / 2;
        if (u < O) throw new T("Too many math in a row: " + ("expected " + u + ", but got " + O), B[0]);
      }
    });
    for (var k = 0; k < m; ++k) {
      var S = "r", z = 0;
      k % 2 === 1 ? S = "l" : k > 0 && x && (z = 1), a[k] = {
        type: "align",
        align: S,
        pregap: z,
        postgap: 0
      };
    }
    return o.colSeparationType = x ? "align" : "alignat", o;
  };
  D0({
    type: "array",
    names: [
      "array",
      "darray"
    ],
    props: {
      numArgs: 1
    },
    handler(r, e) {
      var t = at(e[0]), a = t ? [
        e[0]
      ] : G(e[0], "ordgroup").body, n = a.map(function(o) {
        var u = rr(o), m = u.text;
        if ("lcr".indexOf(m) !== -1) return {
          type: "align",
          align: m
        };
        if (m === "|") return {
          type: "separator",
          separator: "|"
        };
        if (m === ":") return {
          type: "separator",
          separator: ":"
        };
        throw new T("Unknown column alignment: " + m, o);
      }), i = {
        cols: n,
        hskipBeforeAndAfter: true,
        maxNumCols: n.length
      };
      return Q0(r.parser, i, or(r.envName));
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "matrix",
      "pmatrix",
      "bmatrix",
      "Bmatrix",
      "vmatrix",
      "Vmatrix",
      "matrix*",
      "pmatrix*",
      "bmatrix*",
      "Bmatrix*",
      "vmatrix*",
      "Vmatrix*"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      var e = {
        matrix: null,
        pmatrix: [
          "(",
          ")"
        ],
        bmatrix: [
          "[",
          "]"
        ],
        Bmatrix: [
          "\\{",
          "\\}"
        ],
        vmatrix: [
          "|",
          "|"
        ],
        Vmatrix: [
          "\\Vert",
          "\\Vert"
        ]
      }[r.envName.replace("*", "")], t = "c", a = {
        hskipBeforeAndAfter: false,
        cols: [
          {
            type: "align",
            align: t
          }
        ]
      };
      if (r.envName.charAt(r.envName.length - 1) === "*") {
        var n = r.parser;
        if (n.consumeSpaces(), n.fetch().text === "[") {
          if (n.consume(), n.consumeSpaces(), t = n.fetch().text, "lcr".indexOf(t) === -1) throw new T("Expected l or c or r", n.nextToken);
          n.consume(), n.consumeSpaces(), n.expect("]"), n.consume(), a.cols = [
            {
              type: "align",
              align: t
            }
          ];
        }
      }
      var i = Q0(r.parser, a, or(r.envName)), o = Math.max(0, ...i.body.map((u) => u.length));
      return i.cols = new Array(o).fill({
        type: "align",
        align: t
      }), e ? {
        type: "leftright",
        mode: r.mode,
        body: [
          i
        ],
        left: e[0],
        right: e[1],
        rightColor: void 0
      } : i;
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "smallmatrix"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      var e = {
        arraystretch: 0.5
      }, t = Q0(r.parser, e, "script");
      return t.colSeparationType = "small", t;
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "subarray"
    ],
    props: {
      numArgs: 1
    },
    handler(r, e) {
      var t = at(e[0]), a = t ? [
        e[0]
      ] : G(e[0], "ordgroup").body, n = a.map(function(o) {
        var u = rr(o), m = u.text;
        if ("lc".indexOf(m) !== -1) return {
          type: "align",
          align: m
        };
        throw new T("Unknown column alignment: " + m, o);
      });
      if (n.length > 1) throw new T("{subarray} can contain only one column");
      var i = {
        cols: n,
        hskipBeforeAndAfter: false,
        arraystretch: 0.5
      };
      if (i = Q0(r.parser, i, "script"), i.body.length > 0 && i.body[0].length > 1) throw new T("{subarray} can contain only one column");
      return i;
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "cases",
      "dcases",
      "rcases",
      "drcases"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      var e = {
        arraystretch: 1.2,
        cols: [
          {
            type: "align",
            align: "l",
            pregap: 0,
            postgap: 1
          },
          {
            type: "align",
            align: "l",
            pregap: 0,
            postgap: 0
          }
        ]
      }, t = Q0(r.parser, e, or(r.envName));
      return {
        type: "leftright",
        mode: r.mode,
        body: [
          t
        ],
        left: r.envName.indexOf("r") > -1 ? "." : "\\{",
        right: r.envName.indexOf("r") > -1 ? "\\}" : ".",
        rightColor: void 0
      };
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "align",
      "align*",
      "aligned",
      "split"
    ],
    props: {
      numArgs: 0
    },
    handler: e1,
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "gathered",
      "gather",
      "gather*"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      [
        "gather",
        "gather*"
      ].includes(r.envName) && st(r);
      var e = {
        cols: [
          {
            type: "align",
            align: "c"
          }
        ],
        addJot: true,
        colSeparationType: "gather",
        autoTag: lr(r.envName),
        emptySingleRow: true,
        leqno: r.parser.settings.leqno
      };
      return Q0(r.parser, e, "display");
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "alignat",
      "alignat*",
      "alignedat"
    ],
    props: {
      numArgs: 1
    },
    handler: e1,
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "equation",
      "equation*"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      st(r);
      var e = {
        autoTag: lr(r.envName),
        emptySingleRow: true,
        singleRow: true,
        maxNumCols: 1,
        leqno: r.parser.settings.leqno
      };
      return Q0(r.parser, e, "display");
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  D0({
    type: "array",
    names: [
      "CD"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      return st(r), Xi(r.parser);
    },
    htmlBuilder: N0,
    mathmlBuilder: q0
  });
  c("\\nonumber", "\\gdef\\@eqnsw{0}");
  c("\\notag", "\\nonumber");
  D({
    type: "text",
    names: [
      "\\hline",
      "\\hdashline"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      allowedInMath: true
    },
    handler(r, e) {
      throw new T(r.funcName + " valid only within array environment");
    }
  });
  var Ur = Ja;
  D({
    type: "environment",
    names: [
      "\\begin",
      "\\end"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "text"
      ]
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r, n = e[0];
      if (n.type !== "ordgroup") throw new T("Invalid environment name", n);
      for (var i = "", o = 0; o < n.body.length; ++o) i += G(n.body[o], "textord").text;
      if (a === "\\begin") {
        if (!Ur.hasOwnProperty(i)) throw new T("No such environment: " + i, n);
        var u = Ur[i], { args: m, optArgs: d } = t.parseArguments("\\begin{" + i + "}", u), p = {
          mode: t.mode,
          envName: i,
          parser: t
        }, b = u.handler(p, m, d);
        t.expect("\\end", false);
        var y = t.nextToken, x = G(t.parseFunction(), "environment");
        if (x.name !== i) throw new T("Mismatch: \\begin{" + i + "} matched by \\end{" + x.name + "}", y);
        return b;
      }
      return {
        type: "environment",
        mode: t.mode,
        name: i,
        nameGroup: n
      };
    }
  });
  var t1 = (r, e) => {
    var t = r.font, a = e.withFont(t);
    return $(r.body, a);
  }, r1 = (r, e) => {
    var t = r.font, a = e.withFont(t);
    return X(r.body, a);
  }, $r = {
    "\\Bbb": "\\mathbb",
    "\\bold": "\\mathbf",
    "\\frak": "\\mathfrak",
    "\\bm": "\\boldsymbol"
  };
  D({
    type: "font",
    names: [
      "\\mathrm",
      "\\mathit",
      "\\mathbf",
      "\\mathnormal",
      "\\mathsfit",
      "\\mathbb",
      "\\mathcal",
      "\\mathfrak",
      "\\mathscr",
      "\\mathsf",
      "\\mathtt",
      "\\Bbb",
      "\\bold",
      "\\frak"
    ],
    props: {
      numArgs: 1,
      allowedInArgument: true
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = Je(e[0]), i = a;
      return i in $r && (i = $r[i]), {
        type: "font",
        mode: t.mode,
        font: i.slice(1),
        body: n
      };
    },
    htmlBuilder: t1,
    mathmlBuilder: r1
  });
  D({
    type: "mclass",
    names: [
      "\\boldsymbol",
      "\\bm"
    ],
    props: {
      numArgs: 1
    },
    handler: (r, e) => {
      var { parser: t } = r, a = e[0], n = Y.isCharacterBox(a);
      return {
        type: "mclass",
        mode: t.mode,
        mclass: nt(a),
        body: [
          {
            type: "font",
            mode: t.mode,
            font: "boldsymbol",
            body: a
          }
        ],
        isCharacterBox: n
      };
    }
  });
  D({
    type: "font",
    names: [
      "\\rm",
      "\\sf",
      "\\tt",
      "\\bf",
      "\\it",
      "\\cal"
    ],
    props: {
      numArgs: 0,
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t, funcName: a, breakOnTokenText: n } = r, { mode: i } = t, o = t.parseExpression(true, n), u = "math" + a.slice(1);
      return {
        type: "font",
        mode: i,
        font: u,
        body: {
          type: "ordgroup",
          mode: t.mode,
          body: o
        }
      };
    },
    htmlBuilder: t1,
    mathmlBuilder: r1
  });
  var a1 = (r, e) => {
    var t = e;
    return r === "display" ? t = t.id >= I.SCRIPT.id ? t.text() : I.DISPLAY : r === "text" && t.size === I.DISPLAY.size ? t = I.TEXT : r === "script" ? t = I.SCRIPT : r === "scriptscript" && (t = I.SCRIPTSCRIPT), t;
  }, ur = (r, e) => {
    var t = a1(r.size, e.style), a = t.fracNum(), n = t.fracDen(), i;
    i = e.havingStyle(a);
    var o = $(r.numer, i, e);
    if (r.continued) {
      var u = 8.5 / e.fontMetrics().ptPerEm, m = 3.5 / e.fontMetrics().ptPerEm;
      o.height = o.height < u ? u : o.height, o.depth = o.depth < m ? m : o.depth;
    }
    i = e.havingStyle(n);
    var d = $(r.denom, i, e), p, b, y;
    r.hasBarLine ? (r.barSize ? (b = K(r.barSize, e), p = w.makeLineSpan("frac-line", e, b)) : p = w.makeLineSpan("frac-line", e), b = p.height, y = p.height) : (p = null, b = 0, y = e.fontMetrics().defaultRuleThickness);
    var x, k, S;
    t.size === I.DISPLAY.size || r.size === "display" ? (x = e.fontMetrics().num1, b > 0 ? k = 3 * y : k = 7 * y, S = e.fontMetrics().denom1) : (b > 0 ? (x = e.fontMetrics().num2, k = y) : (x = e.fontMetrics().num3, k = 3 * y), S = e.fontMetrics().denom2);
    var z;
    if (p) {
      var N = e.fontMetrics().axisHeight;
      x - o.depth - (N + 0.5 * b) < k && (x += k - (x - o.depth - (N + 0.5 * b))), N - 0.5 * b - (d.height - S) < k && (S += k - (N - 0.5 * b - (d.height - S)));
      var E = -(N - 0.5 * b);
      z = w.makeVList({
        positionType: "individualShift",
        children: [
          {
            type: "elem",
            elem: d,
            shift: S
          },
          {
            type: "elem",
            elem: p,
            shift: E
          },
          {
            type: "elem",
            elem: o,
            shift: -x
          }
        ]
      }, e);
    } else {
      var B = x - o.depth - (d.height - S);
      B < k && (x += 0.5 * (k - B), S += 0.5 * (k - B)), z = w.makeVList({
        positionType: "individualShift",
        children: [
          {
            type: "elem",
            elem: d,
            shift: S
          },
          {
            type: "elem",
            elem: o,
            shift: -x
          }
        ]
      }, e);
    }
    i = e.havingStyle(t), z.height *= i.sizeMultiplier / e.sizeMultiplier, z.depth *= i.sizeMultiplier / e.sizeMultiplier;
    var H;
    t.size === I.DISPLAY.size ? H = e.fontMetrics().delim1 : t.size === I.SCRIPTSCRIPT.size ? H = e.havingStyle(I.SCRIPT).fontMetrics().delim2 : H = e.fontMetrics().delim2;
    var O, L;
    return r.leftDelim == null ? O = Ae(e, [
      "mopen"
    ]) : O = F0.customSizedDelim(r.leftDelim, H, true, e.havingStyle(t), r.mode, [
      "mopen"
    ]), r.continued ? L = w.makeSpan([]) : r.rightDelim == null ? L = Ae(e, [
      "mclose"
    ]) : L = F0.customSizedDelim(r.rightDelim, H, true, e.havingStyle(t), r.mode, [
      "mclose"
    ]), w.makeSpan([
      "mord"
    ].concat(i.sizingClasses(e)), [
      O,
      w.makeSpan([
        "mfrac"
      ], [
        z
      ]),
      L
    ], e);
  }, hr = (r, e) => {
    var t = new A.MathNode("mfrac", [
      X(r.numer, e),
      X(r.denom, e)
    ]);
    if (!r.hasBarLine) t.setAttribute("linethickness", "0px");
    else if (r.barSize) {
      var a = K(r.barSize, e);
      t.setAttribute("linethickness", C(a));
    }
    var n = a1(r.size, e.style);
    if (n.size !== e.style.size) {
      t = new A.MathNode("mstyle", [
        t
      ]);
      var i = n.size === I.DISPLAY.size ? "true" : "false";
      t.setAttribute("displaystyle", i), t.setAttribute("scriptlevel", "0");
    }
    if (r.leftDelim != null || r.rightDelim != null) {
      var o = [];
      if (r.leftDelim != null) {
        var u = new A.MathNode("mo", [
          new A.TextNode(r.leftDelim.replace("\\", ""))
        ]);
        u.setAttribute("fence", "true"), o.push(u);
      }
      if (o.push(t), r.rightDelim != null) {
        var m = new A.MathNode("mo", [
          new A.TextNode(r.rightDelim.replace("\\", ""))
        ]);
        m.setAttribute("fence", "true"), o.push(m);
      }
      return er(o);
    }
    return t;
  };
  D({
    type: "genfrac",
    names: [
      "\\dfrac",
      "\\frac",
      "\\tfrac",
      "\\dbinom",
      "\\binom",
      "\\tbinom",
      "\\\\atopfrac",
      "\\\\bracefrac",
      "\\\\brackfrac"
    ],
    props: {
      numArgs: 2,
      allowedInArgument: true
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = e[0], i = e[1], o, u = null, m = null, d = "auto";
      switch (a) {
        case "\\dfrac":
        case "\\frac":
        case "\\tfrac":
          o = true;
          break;
        case "\\\\atopfrac":
          o = false;
          break;
        case "\\dbinom":
        case "\\binom":
        case "\\tbinom":
          o = false, u = "(", m = ")";
          break;
        case "\\\\bracefrac":
          o = false, u = "\\{", m = "\\}";
          break;
        case "\\\\brackfrac":
          o = false, u = "[", m = "]";
          break;
        default:
          throw new Error("Unrecognized genfrac command");
      }
      switch (a) {
        case "\\dfrac":
        case "\\dbinom":
          d = "display";
          break;
        case "\\tfrac":
        case "\\tbinom":
          d = "text";
          break;
      }
      return {
        type: "genfrac",
        mode: t.mode,
        continued: false,
        numer: n,
        denom: i,
        hasBarLine: o,
        leftDelim: u,
        rightDelim: m,
        size: d,
        barSize: null
      };
    },
    htmlBuilder: ur,
    mathmlBuilder: hr
  });
  D({
    type: "genfrac",
    names: [
      "\\cfrac"
    ],
    props: {
      numArgs: 2
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = e[0], i = e[1];
      return {
        type: "genfrac",
        mode: t.mode,
        continued: true,
        numer: n,
        denom: i,
        hasBarLine: true,
        leftDelim: null,
        rightDelim: null,
        size: "display",
        barSize: null
      };
    }
  });
  D({
    type: "infix",
    names: [
      "\\over",
      "\\choose",
      "\\atop",
      "\\brace",
      "\\brack"
    ],
    props: {
      numArgs: 0,
      infix: true
    },
    handler(r) {
      var { parser: e, funcName: t, token: a } = r, n;
      switch (t) {
        case "\\over":
          n = "\\frac";
          break;
        case "\\choose":
          n = "\\binom";
          break;
        case "\\atop":
          n = "\\\\atopfrac";
          break;
        case "\\brace":
          n = "\\\\bracefrac";
          break;
        case "\\brack":
          n = "\\\\brackfrac";
          break;
        default:
          throw new Error("Unrecognized infix genfrac command");
      }
      return {
        type: "infix",
        mode: e.mode,
        replaceWith: n,
        token: a
      };
    }
  });
  var Yr = [
    "display",
    "text",
    "script",
    "scriptscript"
  ], Xr = function(e) {
    var t = null;
    return e.length > 0 && (t = e, t = t === "." ? null : t), t;
  };
  D({
    type: "genfrac",
    names: [
      "\\genfrac"
    ],
    props: {
      numArgs: 6,
      allowedInArgument: true,
      argTypes: [
        "math",
        "math",
        "size",
        "text",
        "math",
        "math"
      ]
    },
    handler(r, e) {
      var { parser: t } = r, a = e[4], n = e[5], i = Je(e[0]), o = i.type === "atom" && i.family === "open" ? Xr(i.text) : null, u = Je(e[1]), m = u.type === "atom" && u.family === "close" ? Xr(u.text) : null, d = G(e[2], "size"), p, b = null;
      d.isBlank ? p = true : (b = d.value, p = b.number > 0);
      var y = "auto", x = e[3];
      if (x.type === "ordgroup") {
        if (x.body.length > 0) {
          var k = G(x.body[0], "textord");
          y = Yr[Number(k.text)];
        }
      } else x = G(x, "textord"), y = Yr[Number(x.text)];
      return {
        type: "genfrac",
        mode: t.mode,
        numer: a,
        denom: n,
        continued: false,
        hasBarLine: p,
        barSize: b,
        leftDelim: o,
        rightDelim: m,
        size: y
      };
    },
    htmlBuilder: ur,
    mathmlBuilder: hr
  });
  D({
    type: "infix",
    names: [
      "\\above"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "size"
      ],
      infix: true
    },
    handler(r, e) {
      var { parser: t, funcName: a, token: n } = r;
      return {
        type: "infix",
        mode: t.mode,
        replaceWith: "\\\\abovefrac",
        size: G(e[0], "size").value,
        token: n
      };
    }
  });
  D({
    type: "genfrac",
    names: [
      "\\\\abovefrac"
    ],
    props: {
      numArgs: 3,
      argTypes: [
        "math",
        "size",
        "math"
      ]
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = e[0], i = On(G(e[1], "infix").size), o = e[2], u = i.number > 0;
      return {
        type: "genfrac",
        mode: t.mode,
        numer: n,
        denom: o,
        continued: false,
        hasBarLine: u,
        barSize: i,
        leftDelim: null,
        rightDelim: null,
        size: "auto"
      };
    },
    htmlBuilder: ur,
    mathmlBuilder: hr
  });
  var n1 = (r, e) => {
    var t = e.style, a, n;
    r.type === "supsub" ? (a = r.sup ? $(r.sup, e.havingStyle(t.sup()), e) : $(r.sub, e.havingStyle(t.sub()), e), n = G(r.base, "horizBrace")) : n = G(r, "horizBrace");
    var i = $(n.base, e.havingBaseStyle(I.DISPLAY)), o = P0.svgSpan(n, e), u;
    if (n.isOver ? (u = w.makeVList({
      positionType: "firstBaseline",
      children: [
        {
          type: "elem",
          elem: i
        },
        {
          type: "kern",
          size: 0.1
        },
        {
          type: "elem",
          elem: o
        }
      ]
    }, e), u.children[0].children[0].children[1].classes.push("svg-align")) : (u = w.makeVList({
      positionType: "bottom",
      positionData: i.depth + 0.1 + o.height,
      children: [
        {
          type: "elem",
          elem: o
        },
        {
          type: "kern",
          size: 0.1
        },
        {
          type: "elem",
          elem: i
        }
      ]
    }, e), u.children[0].children[0].children[0].classes.push("svg-align")), a) {
      var m = w.makeSpan([
        "mord",
        n.isOver ? "mover" : "munder"
      ], [
        u
      ], e);
      n.isOver ? u = w.makeVList({
        positionType: "firstBaseline",
        children: [
          {
            type: "elem",
            elem: m
          },
          {
            type: "kern",
            size: 0.2
          },
          {
            type: "elem",
            elem: a
          }
        ]
      }, e) : u = w.makeVList({
        positionType: "bottom",
        positionData: m.depth + 0.2 + a.height + a.depth,
        children: [
          {
            type: "elem",
            elem: a
          },
          {
            type: "kern",
            size: 0.2
          },
          {
            type: "elem",
            elem: m
          }
        ]
      }, e);
    }
    return w.makeSpan([
      "mord",
      n.isOver ? "mover" : "munder"
    ], [
      u
    ], e);
  }, l4 = (r, e) => {
    var t = P0.mathMLnode(r.label);
    return new A.MathNode(r.isOver ? "mover" : "munder", [
      X(r.base, e),
      t
    ]);
  };
  D({
    type: "horizBrace",
    names: [
      "\\overbrace",
      "\\underbrace"
    ],
    props: {
      numArgs: 1
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r;
      return {
        type: "horizBrace",
        mode: t.mode,
        label: a,
        isOver: /^\\over/.test(a),
        base: e[0]
      };
    },
    htmlBuilder: n1,
    mathmlBuilder: l4
  });
  D({
    type: "href",
    names: [
      "\\href"
    ],
    props: {
      numArgs: 2,
      argTypes: [
        "url",
        "original"
      ],
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t } = r, a = e[1], n = G(e[0], "url").url;
      return t.settings.isTrusted({
        command: "\\href",
        url: n
      }) ? {
        type: "href",
        mode: t.mode,
        href: n,
        body: J(a)
      } : t.formatUnsupportedCmd("\\href");
    },
    htmlBuilder: (r, e) => {
      var t = t0(r.body, e, false);
      return w.makeAnchor(r.href, [], t, e);
    },
    mathmlBuilder: (r, e) => {
      var t = J0(r.body, e);
      return t instanceof c0 || (t = new c0("mrow", [
        t
      ])), t.setAttribute("href", r.href), t;
    }
  });
  D({
    type: "href",
    names: [
      "\\url"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "url"
      ],
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t } = r, a = G(e[0], "url").url;
      if (!t.settings.isTrusted({
        command: "\\url",
        url: a
      })) return t.formatUnsupportedCmd("\\url");
      for (var n = [], i = 0; i < a.length; i++) {
        var o = a[i];
        o === "~" && (o = "\\textasciitilde"), n.push({
          type: "textord",
          mode: "text",
          text: o
        });
      }
      var u = {
        type: "text",
        mode: t.mode,
        font: "\\texttt",
        body: n
      };
      return {
        type: "href",
        mode: t.mode,
        href: a,
        body: J(u)
      };
    }
  });
  D({
    type: "hbox",
    names: [
      "\\hbox"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "text"
      ],
      allowedInText: true,
      primitive: true
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "hbox",
        mode: t.mode,
        body: J(e[0])
      };
    },
    htmlBuilder(r, e) {
      var t = t0(r.body, e, false);
      return w.makeFragment(t);
    },
    mathmlBuilder(r, e) {
      return new A.MathNode("mrow", m0(r.body, e));
    }
  });
  D({
    type: "html",
    names: [
      "\\htmlClass",
      "\\htmlId",
      "\\htmlStyle",
      "\\htmlData"
    ],
    props: {
      numArgs: 2,
      argTypes: [
        "raw",
        "original"
      ],
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t, funcName: a, token: n } = r, i = G(e[0], "raw").string, o = e[1];
      t.settings.strict && t.settings.reportNonstrict("htmlExtension", "HTML extension is disabled on strict mode");
      var u, m = {};
      switch (a) {
        case "\\htmlClass":
          m.class = i, u = {
            command: "\\htmlClass",
            class: i
          };
          break;
        case "\\htmlId":
          m.id = i, u = {
            command: "\\htmlId",
            id: i
          };
          break;
        case "\\htmlStyle":
          m.style = i, u = {
            command: "\\htmlStyle",
            style: i
          };
          break;
        case "\\htmlData": {
          for (var d = i.split(","), p = 0; p < d.length; p++) {
            var b = d[p], y = b.indexOf("=");
            if (y < 0) throw new T("\\htmlData key/value '" + b + "' missing equals sign");
            var x = b.slice(0, y), k = b.slice(y + 1);
            m["data-" + x.trim()] = k;
          }
          u = {
            command: "\\htmlData",
            attributes: m
          };
          break;
        }
        default:
          throw new Error("Unrecognized html command");
      }
      return t.settings.isTrusted(u) ? {
        type: "html",
        mode: t.mode,
        attributes: m,
        body: J(o)
      } : t.formatUnsupportedCmd(a);
    },
    htmlBuilder: (r, e) => {
      var t = t0(r.body, e, false), a = [
        "enclosing"
      ];
      r.attributes.class && a.push(...r.attributes.class.trim().split(/\s+/));
      var n = w.makeSpan(a, t, e);
      for (var i in r.attributes) i !== "class" && r.attributes.hasOwnProperty(i) && n.setAttribute(i, r.attributes[i]);
      return n;
    },
    mathmlBuilder: (r, e) => J0(r.body, e)
  });
  D({
    type: "htmlmathml",
    names: [
      "\\html@mathml"
    ],
    props: {
      numArgs: 2,
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t } = r;
      return {
        type: "htmlmathml",
        mode: t.mode,
        html: J(e[0]),
        mathml: J(e[1])
      };
    },
    htmlBuilder: (r, e) => {
      var t = t0(r.html, e, false);
      return w.makeFragment(t);
    },
    mathmlBuilder: (r, e) => J0(r.mathml, e)
  });
  var Bt = function(e) {
    if (/^[-+]? *(\d+(\.\d*)?|\.\d+)$/.test(e)) return {
      number: +e,
      unit: "bp"
    };
    var t = /([-+]?) *(\d+(?:\.\d*)?|\.\d+) *([a-z]{2})/.exec(e);
    if (!t) throw new T("Invalid size: '" + e + "' in \\includegraphics");
    var a = {
      number: +(t[1] + t[2]),
      unit: t[3]
    };
    if (!Sa(a)) throw new T("Invalid unit: '" + a.unit + "' in \\includegraphics.");
    return a;
  };
  D({
    type: "includegraphics",
    names: [
      "\\includegraphics"
    ],
    props: {
      numArgs: 1,
      numOptionalArgs: 1,
      argTypes: [
        "raw",
        "url"
      ],
      allowedInText: false
    },
    handler: (r, e, t) => {
      var { parser: a } = r, n = {
        number: 0,
        unit: "em"
      }, i = {
        number: 0.9,
        unit: "em"
      }, o = {
        number: 0,
        unit: "em"
      }, u = "";
      if (t[0]) for (var m = G(t[0], "raw").string, d = m.split(","), p = 0; p < d.length; p++) {
        var b = d[p].split("=");
        if (b.length === 2) {
          var y = b[1].trim();
          switch (b[0].trim()) {
            case "alt":
              u = y;
              break;
            case "width":
              n = Bt(y);
              break;
            case "height":
              i = Bt(y);
              break;
            case "totalheight":
              o = Bt(y);
              break;
            default:
              throw new T("Invalid key: '" + b[0] + "' in \\includegraphics.");
          }
        }
      }
      var x = G(e[0], "url").url;
      return u === "" && (u = x, u = u.replace(/^.*[\\/]/, ""), u = u.substring(0, u.lastIndexOf("."))), a.settings.isTrusted({
        command: "\\includegraphics",
        url: x
      }) ? {
        type: "includegraphics",
        mode: a.mode,
        alt: u,
        width: n,
        height: i,
        totalheight: o,
        src: x
      } : a.formatUnsupportedCmd("\\includegraphics");
    },
    htmlBuilder: (r, e) => {
      var t = K(r.height, e), a = 0;
      r.totalheight.number > 0 && (a = K(r.totalheight, e) - t);
      var n = 0;
      r.width.number > 0 && (n = K(r.width, e));
      var i = {
        height: C(t + a)
      };
      n > 0 && (i.width = C(n)), a > 0 && (i.verticalAlign = C(-a));
      var o = new li(r.src, r.alt, i);
      return o.height = t, o.depth = a, o;
    },
    mathmlBuilder: (r, e) => {
      var t = new A.MathNode("mglyph", []);
      t.setAttribute("alt", r.alt);
      var a = K(r.height, e), n = 0;
      if (r.totalheight.number > 0 && (n = K(r.totalheight, e) - a, t.setAttribute("valign", C(-n))), t.setAttribute("height", C(a + n)), r.width.number > 0) {
        var i = K(r.width, e);
        t.setAttribute("width", C(i));
      }
      return t.setAttribute("src", r.src), t;
    }
  });
  D({
    type: "kern",
    names: [
      "\\kern",
      "\\mkern",
      "\\hskip",
      "\\mskip"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "size"
      ],
      primitive: true,
      allowedInText: true
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r, n = G(e[0], "size");
      if (t.settings.strict) {
        var i = a[1] === "m", o = n.value.unit === "mu";
        i ? (o || t.settings.reportNonstrict("mathVsTextUnits", "LaTeX's " + a + " supports only mu units, " + ("not " + n.value.unit + " units")), t.mode !== "math" && t.settings.reportNonstrict("mathVsTextUnits", "LaTeX's " + a + " works only in math mode")) : o && t.settings.reportNonstrict("mathVsTextUnits", "LaTeX's " + a + " doesn't support mu units");
      }
      return {
        type: "kern",
        mode: t.mode,
        dimension: n.value
      };
    },
    htmlBuilder(r, e) {
      return w.makeGlue(r.dimension, e);
    },
    mathmlBuilder(r, e) {
      var t = K(r.dimension, e);
      return new A.SpaceNode(t);
    }
  });
  D({
    type: "lap",
    names: [
      "\\mathllap",
      "\\mathrlap",
      "\\mathclap"
    ],
    props: {
      numArgs: 1,
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = e[0];
      return {
        type: "lap",
        mode: t.mode,
        alignment: a.slice(5),
        body: n
      };
    },
    htmlBuilder: (r, e) => {
      var t;
      r.alignment === "clap" ? (t = w.makeSpan([], [
        $(r.body, e)
      ]), t = w.makeSpan([
        "inner"
      ], [
        t
      ], e)) : t = w.makeSpan([
        "inner"
      ], [
        $(r.body, e)
      ]);
      var a = w.makeSpan([
        "fix"
      ], []), n = w.makeSpan([
        r.alignment
      ], [
        t,
        a
      ], e), i = w.makeSpan([
        "strut"
      ]);
      return i.style.height = C(n.height + n.depth), n.depth && (i.style.verticalAlign = C(-n.depth)), n.children.unshift(i), n = w.makeSpan([
        "thinbox"
      ], [
        n
      ], e), w.makeSpan([
        "mord",
        "vbox"
      ], [
        n
      ], e);
    },
    mathmlBuilder: (r, e) => {
      var t = new A.MathNode("mpadded", [
        X(r.body, e)
      ]);
      if (r.alignment !== "rlap") {
        var a = r.alignment === "llap" ? "-1" : "-0.5";
        t.setAttribute("lspace", a + "width");
      }
      return t.setAttribute("width", "0px"), t;
    }
  });
  D({
    type: "styling",
    names: [
      "\\(",
      "$"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      allowedInMath: false
    },
    handler(r, e) {
      var { funcName: t, parser: a } = r, n = a.mode;
      a.switchMode("math");
      var i = t === "\\(" ? "\\)" : "$", o = a.parseExpression(false, i);
      return a.expect(i), a.switchMode(n), {
        type: "styling",
        mode: a.mode,
        style: "text",
        body: o
      };
    }
  });
  D({
    type: "text",
    names: [
      "\\)",
      "\\]"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      allowedInMath: false
    },
    handler(r, e) {
      throw new T("Mismatched " + r.funcName);
    }
  });
  var Wr = (r, e) => {
    switch (e.style.size) {
      case I.DISPLAY.size:
        return r.display;
      case I.TEXT.size:
        return r.text;
      case I.SCRIPT.size:
        return r.script;
      case I.SCRIPTSCRIPT.size:
        return r.scriptscript;
      default:
        return r.text;
    }
  };
  D({
    type: "mathchoice",
    names: [
      "\\mathchoice"
    ],
    props: {
      numArgs: 4,
      primitive: true
    },
    handler: (r, e) => {
      var { parser: t } = r;
      return {
        type: "mathchoice",
        mode: t.mode,
        display: J(e[0]),
        text: J(e[1]),
        script: J(e[2]),
        scriptscript: J(e[3])
      };
    },
    htmlBuilder: (r, e) => {
      var t = Wr(r, e), a = t0(t, e, false);
      return w.makeFragment(a);
    },
    mathmlBuilder: (r, e) => {
      var t = Wr(r, e);
      return J0(t, e);
    }
  });
  var i1 = (r, e, t, a, n, i, o) => {
    r = w.makeSpan([], [
      r
    ]);
    var u = t && Y.isCharacterBox(t), m, d;
    if (e) {
      var p = $(e, a.havingStyle(n.sup()), a);
      d = {
        elem: p,
        kern: Math.max(a.fontMetrics().bigOpSpacing1, a.fontMetrics().bigOpSpacing3 - p.depth)
      };
    }
    if (t) {
      var b = $(t, a.havingStyle(n.sub()), a);
      m = {
        elem: b,
        kern: Math.max(a.fontMetrics().bigOpSpacing2, a.fontMetrics().bigOpSpacing4 - b.height)
      };
    }
    var y;
    if (d && m) {
      var x = a.fontMetrics().bigOpSpacing5 + m.elem.height + m.elem.depth + m.kern + r.depth + o;
      y = w.makeVList({
        positionType: "bottom",
        positionData: x,
        children: [
          {
            type: "kern",
            size: a.fontMetrics().bigOpSpacing5
          },
          {
            type: "elem",
            elem: m.elem,
            marginLeft: C(-i)
          },
          {
            type: "kern",
            size: m.kern
          },
          {
            type: "elem",
            elem: r
          },
          {
            type: "kern",
            size: d.kern
          },
          {
            type: "elem",
            elem: d.elem,
            marginLeft: C(i)
          },
          {
            type: "kern",
            size: a.fontMetrics().bigOpSpacing5
          }
        ]
      }, a);
    } else if (m) {
      var k = r.height - o;
      y = w.makeVList({
        positionType: "top",
        positionData: k,
        children: [
          {
            type: "kern",
            size: a.fontMetrics().bigOpSpacing5
          },
          {
            type: "elem",
            elem: m.elem,
            marginLeft: C(-i)
          },
          {
            type: "kern",
            size: m.kern
          },
          {
            type: "elem",
            elem: r
          }
        ]
      }, a);
    } else if (d) {
      var S = r.depth + o;
      y = w.makeVList({
        positionType: "bottom",
        positionData: S,
        children: [
          {
            type: "elem",
            elem: r
          },
          {
            type: "kern",
            size: d.kern
          },
          {
            type: "elem",
            elem: d.elem,
            marginLeft: C(i)
          },
          {
            type: "kern",
            size: a.fontMetrics().bigOpSpacing5
          }
        ]
      }, a);
    } else return r;
    var z = [
      y
    ];
    if (m && i !== 0 && !u) {
      var B = w.makeSpan([
        "mspace"
      ], [], a);
      B.style.marginRight = C(i), z.unshift(B);
    }
    return w.makeSpan([
      "mop",
      "op-limits"
    ], z, a);
  }, s1 = [
    "\\smallint"
  ], ge = (r, e) => {
    var t, a, n = false, i;
    r.type === "supsub" ? (t = r.sup, a = r.sub, i = G(r.base, "op"), n = true) : i = G(r, "op");
    var o = e.style, u = false;
    o.size === I.DISPLAY.size && i.symbol && !s1.includes(i.name) && (u = true);
    var m;
    if (i.symbol) {
      var d = u ? "Size2-Regular" : "Size1-Regular", p = "";
      if ((i.name === "\\oiint" || i.name === "\\oiiint") && (p = i.name.slice(1), i.name = p === "oiint" ? "\\iint" : "\\iiint"), m = w.makeSymbol(i.name, d, "math", e, [
        "mop",
        "op-symbol",
        u ? "large-op" : "small-op"
      ]), p.length > 0) {
        var b = m.italic, y = w.staticSvg(p + "Size" + (u ? "2" : "1"), e);
        m = w.makeVList({
          positionType: "individualShift",
          children: [
            {
              type: "elem",
              elem: m,
              shift: 0
            },
            {
              type: "elem",
              elem: y,
              shift: u ? 0.08 : 0
            }
          ]
        }, e), i.name = "\\" + p, m.classes.unshift("mop"), m.italic = b;
      }
    } else if (i.body) {
      var x = t0(i.body, e, true);
      x.length === 1 && x[0] instanceof b0 ? (m = x[0], m.classes[0] = "mop") : m = w.makeSpan([
        "mop"
      ], x, e);
    } else {
      for (var k = [], S = 1; S < i.name.length; S++) k.push(w.mathsym(i.name[S], i.mode, e));
      m = w.makeSpan([
        "mop"
      ], k, e);
    }
    var z = 0, B = 0;
    return (m instanceof b0 || i.name === "\\oiint" || i.name === "\\oiiint") && !i.suppressBaseShift && (z = (m.height - m.depth) / 2 - e.fontMetrics().axisHeight, B = m.italic), n ? i1(m, t, a, e, o, B, z) : (z && (m.style.position = "relative", m.style.top = C(z)), m);
  }, Ce = (r, e) => {
    var t;
    if (r.symbol) t = new c0("mo", [
      y0(r.name, r.mode)
    ]), s1.includes(r.name) && t.setAttribute("largeop", "false");
    else if (r.body) t = new c0("mo", m0(r.body, e));
    else {
      t = new c0("mi", [
        new B0(r.name.slice(1))
      ]);
      var a = new c0("mo", [
        y0("\u2061", "text")
      ]);
      r.parentIsSupSub ? t = new c0("mrow", [
        t,
        a
      ]) : t = Ra([
        t,
        a
      ]);
    }
    return t;
  }, o4 = {
    "\u220F": "\\prod",
    "\u2210": "\\coprod",
    "\u2211": "\\sum",
    "\u22C0": "\\bigwedge",
    "\u22C1": "\\bigvee",
    "\u22C2": "\\bigcap",
    "\u22C3": "\\bigcup",
    "\u2A00": "\\bigodot",
    "\u2A01": "\\bigoplus",
    "\u2A02": "\\bigotimes",
    "\u2A04": "\\biguplus",
    "\u2A06": "\\bigsqcup"
  };
  D({
    type: "op",
    names: [
      "\\coprod",
      "\\bigvee",
      "\\bigwedge",
      "\\biguplus",
      "\\bigcap",
      "\\bigcup",
      "\\intop",
      "\\prod",
      "\\sum",
      "\\bigotimes",
      "\\bigoplus",
      "\\bigodot",
      "\\bigsqcup",
      "\\smallint",
      "\u220F",
      "\u2210",
      "\u2211",
      "\u22C0",
      "\u22C1",
      "\u22C2",
      "\u22C3",
      "\u2A00",
      "\u2A01",
      "\u2A02",
      "\u2A04",
      "\u2A06"
    ],
    props: {
      numArgs: 0
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = a;
      return n.length === 1 && (n = o4[n]), {
        type: "op",
        mode: t.mode,
        limits: true,
        parentIsSupSub: false,
        symbol: true,
        name: n
      };
    },
    htmlBuilder: ge,
    mathmlBuilder: Ce
  });
  D({
    type: "op",
    names: [
      "\\mathop"
    ],
    props: {
      numArgs: 1,
      primitive: true
    },
    handler: (r, e) => {
      var { parser: t } = r, a = e[0];
      return {
        type: "op",
        mode: t.mode,
        limits: false,
        parentIsSupSub: false,
        symbol: false,
        body: J(a)
      };
    },
    htmlBuilder: ge,
    mathmlBuilder: Ce
  });
  var u4 = {
    "\u222B": "\\int",
    "\u222C": "\\iint",
    "\u222D": "\\iiint",
    "\u222E": "\\oint",
    "\u222F": "\\oiint",
    "\u2230": "\\oiiint"
  };
  D({
    type: "op",
    names: [
      "\\arcsin",
      "\\arccos",
      "\\arctan",
      "\\arctg",
      "\\arcctg",
      "\\arg",
      "\\ch",
      "\\cos",
      "\\cosec",
      "\\cosh",
      "\\cot",
      "\\cotg",
      "\\coth",
      "\\csc",
      "\\ctg",
      "\\cth",
      "\\deg",
      "\\dim",
      "\\exp",
      "\\hom",
      "\\ker",
      "\\lg",
      "\\ln",
      "\\log",
      "\\sec",
      "\\sin",
      "\\sinh",
      "\\sh",
      "\\tan",
      "\\tanh",
      "\\tg",
      "\\th"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      var { parser: e, funcName: t } = r;
      return {
        type: "op",
        mode: e.mode,
        limits: false,
        parentIsSupSub: false,
        symbol: false,
        name: t
      };
    },
    htmlBuilder: ge,
    mathmlBuilder: Ce
  });
  D({
    type: "op",
    names: [
      "\\det",
      "\\gcd",
      "\\inf",
      "\\lim",
      "\\max",
      "\\min",
      "\\Pr",
      "\\sup"
    ],
    props: {
      numArgs: 0
    },
    handler(r) {
      var { parser: e, funcName: t } = r;
      return {
        type: "op",
        mode: e.mode,
        limits: true,
        parentIsSupSub: false,
        symbol: false,
        name: t
      };
    },
    htmlBuilder: ge,
    mathmlBuilder: Ce
  });
  D({
    type: "op",
    names: [
      "\\int",
      "\\iint",
      "\\iiint",
      "\\oint",
      "\\oiint",
      "\\oiiint",
      "\u222B",
      "\u222C",
      "\u222D",
      "\u222E",
      "\u222F",
      "\u2230"
    ],
    props: {
      numArgs: 0,
      allowedInArgument: true
    },
    handler(r) {
      var { parser: e, funcName: t } = r, a = t;
      return a.length === 1 && (a = u4[a]), {
        type: "op",
        mode: e.mode,
        limits: false,
        parentIsSupSub: false,
        symbol: true,
        name: a
      };
    },
    htmlBuilder: ge,
    mathmlBuilder: Ce
  });
  var l1 = (r, e) => {
    var t, a, n = false, i;
    r.type === "supsub" ? (t = r.sup, a = r.sub, i = G(r.base, "operatorname"), n = true) : i = G(r, "operatorname");
    var o;
    if (i.body.length > 0) {
      for (var u = i.body.map((b) => {
        var y = b.text;
        return typeof y == "string" ? {
          type: "textord",
          mode: b.mode,
          text: y
        } : b;
      }), m = t0(u, e.withFont("mathrm"), true), d = 0; d < m.length; d++) {
        var p = m[d];
        p instanceof b0 && (p.text = p.text.replace(/\u2212/, "-").replace(/\u2217/, "*"));
      }
      o = w.makeSpan([
        "mop"
      ], m, e);
    } else o = w.makeSpan([
      "mop"
    ], [], e);
    return n ? i1(o, t, a, e, e.style, 0, 0) : o;
  }, h4 = (r, e) => {
    for (var t = m0(r.body, e.withFont("mathrm")), a = true, n = 0; n < t.length; n++) {
      var i = t[n];
      if (!(i instanceof A.SpaceNode)) if (i instanceof A.MathNode) switch (i.type) {
        case "mi":
        case "mn":
        case "ms":
        case "mspace":
        case "mtext":
          break;
        case "mo": {
          var o = i.children[0];
          i.children.length === 1 && o instanceof A.TextNode ? o.text = o.text.replace(/\u2212/, "-").replace(/\u2217/, "*") : a = false;
          break;
        }
        default:
          a = false;
      }
      else a = false;
    }
    if (a) {
      var u = t.map((p) => p.toText()).join("");
      t = [
        new A.TextNode(u)
      ];
    }
    var m = new A.MathNode("mi", t);
    m.setAttribute("mathvariant", "normal");
    var d = new A.MathNode("mo", [
      y0("\u2061", "text")
    ]);
    return r.parentIsSupSub ? new A.MathNode("mrow", [
      m,
      d
    ]) : A.newDocumentFragment([
      m,
      d
    ]);
  };
  D({
    type: "operatorname",
    names: [
      "\\operatorname@",
      "\\operatornamewithlimits"
    ],
    props: {
      numArgs: 1
    },
    handler: (r, e) => {
      var { parser: t, funcName: a } = r, n = e[0];
      return {
        type: "operatorname",
        mode: t.mode,
        body: J(n),
        alwaysHandleSupSub: a === "\\operatornamewithlimits",
        limits: false,
        parentIsSupSub: false
      };
    },
    htmlBuilder: l1,
    mathmlBuilder: h4
  });
  c("\\operatorname", "\\@ifstar\\operatornamewithlimits\\operatorname@");
  ae({
    type: "ordgroup",
    htmlBuilder(r, e) {
      return r.semisimple ? w.makeFragment(t0(r.body, e, false)) : w.makeSpan([
        "mord"
      ], t0(r.body, e, true), e);
    },
    mathmlBuilder(r, e) {
      return J0(r.body, e, true);
    }
  });
  D({
    type: "overline",
    names: [
      "\\overline"
    ],
    props: {
      numArgs: 1
    },
    handler(r, e) {
      var { parser: t } = r, a = e[0];
      return {
        type: "overline",
        mode: t.mode,
        body: a
      };
    },
    htmlBuilder(r, e) {
      var t = $(r.body, e.havingCrampedStyle()), a = w.makeLineSpan("overline-line", e), n = e.fontMetrics().defaultRuleThickness, i = w.makeVList({
        positionType: "firstBaseline",
        children: [
          {
            type: "elem",
            elem: t
          },
          {
            type: "kern",
            size: 3 * n
          },
          {
            type: "elem",
            elem: a
          },
          {
            type: "kern",
            size: n
          }
        ]
      }, e);
      return w.makeSpan([
        "mord",
        "overline"
      ], [
        i
      ], e);
    },
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mo", [
        new A.TextNode("\u203E")
      ]);
      t.setAttribute("stretchy", "true");
      var a = new A.MathNode("mover", [
        X(r.body, e),
        t
      ]);
      return a.setAttribute("accent", "true"), a;
    }
  });
  D({
    type: "phantom",
    names: [
      "\\phantom"
    ],
    props: {
      numArgs: 1,
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t } = r, a = e[0];
      return {
        type: "phantom",
        mode: t.mode,
        body: J(a)
      };
    },
    htmlBuilder: (r, e) => {
      var t = t0(r.body, e.withPhantom(), false);
      return w.makeFragment(t);
    },
    mathmlBuilder: (r, e) => {
      var t = m0(r.body, e);
      return new A.MathNode("mphantom", t);
    }
  });
  D({
    type: "hphantom",
    names: [
      "\\hphantom"
    ],
    props: {
      numArgs: 1,
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t } = r, a = e[0];
      return {
        type: "hphantom",
        mode: t.mode,
        body: a
      };
    },
    htmlBuilder: (r, e) => {
      var t = w.makeSpan([], [
        $(r.body, e.withPhantom())
      ]);
      if (t.height = 0, t.depth = 0, t.children) for (var a = 0; a < t.children.length; a++) t.children[a].height = 0, t.children[a].depth = 0;
      return t = w.makeVList({
        positionType: "firstBaseline",
        children: [
          {
            type: "elem",
            elem: t
          }
        ]
      }, e), w.makeSpan([
        "mord"
      ], [
        t
      ], e);
    },
    mathmlBuilder: (r, e) => {
      var t = m0(J(r.body), e), a = new A.MathNode("mphantom", t), n = new A.MathNode("mpadded", [
        a
      ]);
      return n.setAttribute("height", "0px"), n.setAttribute("depth", "0px"), n;
    }
  });
  D({
    type: "vphantom",
    names: [
      "\\vphantom"
    ],
    props: {
      numArgs: 1,
      allowedInText: true
    },
    handler: (r, e) => {
      var { parser: t } = r, a = e[0];
      return {
        type: "vphantom",
        mode: t.mode,
        body: a
      };
    },
    htmlBuilder: (r, e) => {
      var t = w.makeSpan([
        "inner"
      ], [
        $(r.body, e.withPhantom())
      ]), a = w.makeSpan([
        "fix"
      ], []);
      return w.makeSpan([
        "mord",
        "rlap"
      ], [
        t,
        a
      ], e);
    },
    mathmlBuilder: (r, e) => {
      var t = m0(J(r.body), e), a = new A.MathNode("mphantom", t), n = new A.MathNode("mpadded", [
        a
      ]);
      return n.setAttribute("width", "0px"), n;
    }
  });
  D({
    type: "raisebox",
    names: [
      "\\raisebox"
    ],
    props: {
      numArgs: 2,
      argTypes: [
        "size",
        "hbox"
      ],
      allowedInText: true
    },
    handler(r, e) {
      var { parser: t } = r, a = G(e[0], "size").value, n = e[1];
      return {
        type: "raisebox",
        mode: t.mode,
        dy: a,
        body: n
      };
    },
    htmlBuilder(r, e) {
      var t = $(r.body, e), a = K(r.dy, e);
      return w.makeVList({
        positionType: "shift",
        positionData: -a,
        children: [
          {
            type: "elem",
            elem: t
          }
        ]
      }, e);
    },
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mpadded", [
        X(r.body, e)
      ]), a = r.dy.number + r.dy.unit;
      return t.setAttribute("voffset", a), t;
    }
  });
  D({
    type: "internal",
    names: [
      "\\relax"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      allowedInArgument: true
    },
    handler(r) {
      var { parser: e } = r;
      return {
        type: "internal",
        mode: e.mode
      };
    }
  });
  D({
    type: "rule",
    names: [
      "\\rule"
    ],
    props: {
      numArgs: 2,
      numOptionalArgs: 1,
      allowedInText: true,
      allowedInMath: true,
      argTypes: [
        "size",
        "size",
        "size"
      ]
    },
    handler(r, e, t) {
      var { parser: a } = r, n = t[0], i = G(e[0], "size"), o = G(e[1], "size");
      return {
        type: "rule",
        mode: a.mode,
        shift: n && G(n, "size").value,
        width: i.value,
        height: o.value
      };
    },
    htmlBuilder(r, e) {
      var t = w.makeSpan([
        "mord",
        "rule"
      ], [], e), a = K(r.width, e), n = K(r.height, e), i = r.shift ? K(r.shift, e) : 0;
      return t.style.borderRightWidth = C(a), t.style.borderTopWidth = C(n), t.style.bottom = C(i), t.width = a, t.height = n + i, t.depth = -i, t.maxFontSize = n * 1.125 * e.sizeMultiplier, t;
    },
    mathmlBuilder(r, e) {
      var t = K(r.width, e), a = K(r.height, e), n = r.shift ? K(r.shift, e) : 0, i = e.color && e.getColor() || "black", o = new A.MathNode("mspace");
      o.setAttribute("mathbackground", i), o.setAttribute("width", C(t)), o.setAttribute("height", C(a));
      var u = new A.MathNode("mpadded", [
        o
      ]);
      return n >= 0 ? u.setAttribute("height", C(n)) : (u.setAttribute("height", C(n)), u.setAttribute("depth", C(-n))), u.setAttribute("voffset", C(n)), u;
    }
  });
  function o1(r, e, t) {
    for (var a = t0(r, e, false), n = e.sizeMultiplier / t.sizeMultiplier, i = 0; i < a.length; i++) {
      var o = a[i].classes.indexOf("sizing");
      o < 0 ? Array.prototype.push.apply(a[i].classes, e.sizingClasses(t)) : a[i].classes[o + 1] === "reset-size" + e.size && (a[i].classes[o + 1] = "reset-size" + t.size), a[i].height *= n, a[i].depth *= n;
    }
    return w.makeFragment(a);
  }
  var Zr = [
    "\\tiny",
    "\\sixptsize",
    "\\scriptsize",
    "\\footnotesize",
    "\\small",
    "\\normalsize",
    "\\large",
    "\\Large",
    "\\LARGE",
    "\\huge",
    "\\Huge"
  ], m4 = (r, e) => {
    var t = e.havingSize(r.size);
    return o1(r.body, t, e);
  };
  D({
    type: "sizing",
    names: Zr,
    props: {
      numArgs: 0,
      allowedInText: true
    },
    handler: (r, e) => {
      var { breakOnTokenText: t, funcName: a, parser: n } = r, i = n.parseExpression(false, t);
      return {
        type: "sizing",
        mode: n.mode,
        size: Zr.indexOf(a) + 1,
        body: i
      };
    },
    htmlBuilder: m4,
    mathmlBuilder: (r, e) => {
      var t = e.havingSize(r.size), a = m0(r.body, t), n = new A.MathNode("mstyle", a);
      return n.setAttribute("mathsize", C(t.sizeMultiplier)), n;
    }
  });
  D({
    type: "smash",
    names: [
      "\\smash"
    ],
    props: {
      numArgs: 1,
      numOptionalArgs: 1,
      allowedInText: true
    },
    handler: (r, e, t) => {
      var { parser: a } = r, n = false, i = false, o = t[0] && G(t[0], "ordgroup");
      if (o) for (var u = "", m = 0; m < o.body.length; ++m) {
        var d = o.body[m];
        if (u = d.text, u === "t") n = true;
        else if (u === "b") i = true;
        else {
          n = false, i = false;
          break;
        }
      }
      else n = true, i = true;
      var p = e[0];
      return {
        type: "smash",
        mode: a.mode,
        body: p,
        smashHeight: n,
        smashDepth: i
      };
    },
    htmlBuilder: (r, e) => {
      var t = w.makeSpan([], [
        $(r.body, e)
      ]);
      if (!r.smashHeight && !r.smashDepth) return t;
      if (r.smashHeight && (t.height = 0, t.children)) for (var a = 0; a < t.children.length; a++) t.children[a].height = 0;
      if (r.smashDepth && (t.depth = 0, t.children)) for (var n = 0; n < t.children.length; n++) t.children[n].depth = 0;
      var i = w.makeVList({
        positionType: "firstBaseline",
        children: [
          {
            type: "elem",
            elem: t
          }
        ]
      }, e);
      return w.makeSpan([
        "mord"
      ], [
        i
      ], e);
    },
    mathmlBuilder: (r, e) => {
      var t = new A.MathNode("mpadded", [
        X(r.body, e)
      ]);
      return r.smashHeight && t.setAttribute("height", "0px"), r.smashDepth && t.setAttribute("depth", "0px"), t;
    }
  });
  D({
    type: "sqrt",
    names: [
      "\\sqrt"
    ],
    props: {
      numArgs: 1,
      numOptionalArgs: 1
    },
    handler(r, e, t) {
      var { parser: a } = r, n = t[0], i = e[0];
      return {
        type: "sqrt",
        mode: a.mode,
        body: i,
        index: n
      };
    },
    htmlBuilder(r, e) {
      var t = $(r.body, e.havingCrampedStyle());
      t.height === 0 && (t.height = e.fontMetrics().xHeight), t = w.wrapFragment(t, e);
      var a = e.fontMetrics(), n = a.defaultRuleThickness, i = n;
      e.style.id < I.TEXT.id && (i = e.fontMetrics().xHeight);
      var o = n + i / 4, u = t.height + t.depth + o + n, { span: m, ruleWidth: d, advanceWidth: p } = F0.sqrtImage(u, e), b = m.height - d;
      b > t.height + t.depth + o && (o = (o + b - t.height - t.depth) / 2);
      var y = m.height - t.height - o - d;
      t.style.paddingLeft = C(p);
      var x = w.makeVList({
        positionType: "firstBaseline",
        children: [
          {
            type: "elem",
            elem: t,
            wrapperClasses: [
              "svg-align"
            ]
          },
          {
            type: "kern",
            size: -(t.height + y)
          },
          {
            type: "elem",
            elem: m
          },
          {
            type: "kern",
            size: d
          }
        ]
      }, e);
      if (r.index) {
        var k = e.havingStyle(I.SCRIPTSCRIPT), S = $(r.index, k, e), z = 0.6 * (x.height - x.depth), B = w.makeVList({
          positionType: "shift",
          positionData: -z,
          children: [
            {
              type: "elem",
              elem: S
            }
          ]
        }, e), N = w.makeSpan([
          "root"
        ], [
          B
        ]);
        return w.makeSpan([
          "mord",
          "sqrt"
        ], [
          N,
          x
        ], e);
      } else return w.makeSpan([
        "mord",
        "sqrt"
      ], [
        x
      ], e);
    },
    mathmlBuilder(r, e) {
      var { body: t, index: a } = r;
      return a ? new A.MathNode("mroot", [
        X(t, e),
        X(a, e)
      ]) : new A.MathNode("msqrt", [
        X(t, e)
      ]);
    }
  });
  var jr = {
    display: I.DISPLAY,
    text: I.TEXT,
    script: I.SCRIPT,
    scriptscript: I.SCRIPTSCRIPT
  };
  D({
    type: "styling",
    names: [
      "\\displaystyle",
      "\\textstyle",
      "\\scriptstyle",
      "\\scriptscriptstyle"
    ],
    props: {
      numArgs: 0,
      allowedInText: true,
      primitive: true
    },
    handler(r, e) {
      var { breakOnTokenText: t, funcName: a, parser: n } = r, i = n.parseExpression(true, t), o = a.slice(1, a.length - 5);
      return {
        type: "styling",
        mode: n.mode,
        style: o,
        body: i
      };
    },
    htmlBuilder(r, e) {
      var t = jr[r.style], a = e.havingStyle(t).withFont("");
      return o1(r.body, a, e);
    },
    mathmlBuilder(r, e) {
      var t = jr[r.style], a = e.havingStyle(t), n = m0(r.body, a), i = new A.MathNode("mstyle", n), o = {
        display: [
          "0",
          "true"
        ],
        text: [
          "0",
          "false"
        ],
        script: [
          "1",
          "false"
        ],
        scriptscript: [
          "2",
          "false"
        ]
      }, u = o[r.style];
      return i.setAttribute("scriptlevel", u[0]), i.setAttribute("displaystyle", u[1]), i;
    }
  });
  var c4 = function(e, t) {
    var a = e.base;
    if (a) if (a.type === "op") {
      var n = a.limits && (t.style.size === I.DISPLAY.size || a.alwaysHandleSupSub);
      return n ? ge : null;
    } else if (a.type === "operatorname") {
      var i = a.alwaysHandleSupSub && (t.style.size === I.DISPLAY.size || a.limits);
      return i ? l1 : null;
    } else {
      if (a.type === "accent") return Y.isCharacterBox(a.base) ? ar : null;
      if (a.type === "horizBrace") {
        var o = !e.sub;
        return o === a.isOver ? n1 : null;
      } else return null;
    }
    else return null;
  };
  ae({
    type: "supsub",
    htmlBuilder(r, e) {
      var t = c4(r, e);
      if (t) return t(r, e);
      var { base: a, sup: n, sub: i } = r, o = $(a, e), u, m, d = e.fontMetrics(), p = 0, b = 0, y = a && Y.isCharacterBox(a);
      if (n) {
        var x = e.havingStyle(e.style.sup());
        u = $(n, x, e), y || (p = o.height - x.fontMetrics().supDrop * x.sizeMultiplier / e.sizeMultiplier);
      }
      if (i) {
        var k = e.havingStyle(e.style.sub());
        m = $(i, k, e), y || (b = o.depth + k.fontMetrics().subDrop * k.sizeMultiplier / e.sizeMultiplier);
      }
      var S;
      e.style === I.DISPLAY ? S = d.sup1 : e.style.cramped ? S = d.sup3 : S = d.sup2;
      var z = e.sizeMultiplier, B = C(0.5 / d.ptPerEm / z), N = null;
      if (m) {
        var E = r.base && r.base.type === "op" && r.base.name && (r.base.name === "\\oiint" || r.base.name === "\\oiiint");
        (o instanceof b0 || E) && (N = C(-o.italic));
      }
      var H;
      if (u && m) {
        p = Math.max(p, S, u.depth + 0.25 * d.xHeight), b = Math.max(b, d.sub2);
        var O = d.defaultRuleThickness, L = 4 * O;
        if (p - u.depth - (m.height - b) < L) {
          b = L - (p - u.depth) + m.height;
          var V = 0.8 * d.xHeight - (p - u.depth);
          V > 0 && (p += V, b -= V);
        }
        var U = [
          {
            type: "elem",
            elem: m,
            shift: b,
            marginRight: B,
            marginLeft: N
          },
          {
            type: "elem",
            elem: u,
            shift: -p,
            marginRight: B
          }
        ];
        H = w.makeVList({
          positionType: "individualShift",
          children: U
        }, e);
      } else if (m) {
        b = Math.max(b, d.sub1, m.height - 0.8 * d.xHeight);
        var P = [
          {
            type: "elem",
            elem: m,
            marginLeft: N,
            marginRight: B
          }
        ];
        H = w.makeVList({
          positionType: "shift",
          positionData: b,
          children: P
        }, e);
      } else if (u) p = Math.max(p, S, u.depth + 0.25 * d.xHeight), H = w.makeVList({
        positionType: "shift",
        positionData: -p,
        children: [
          {
            type: "elem",
            elem: u,
            marginRight: B
          }
        ]
      }, e);
      else throw new Error("supsub must have either sup or sub.");
      var x0 = Gt(o, "right") || "mord";
      return w.makeSpan([
        x0
      ], [
        o,
        w.makeSpan([
          "msupsub"
        ], [
          H
        ])
      ], e);
    },
    mathmlBuilder(r, e) {
      var t = false, a, n;
      r.base && r.base.type === "horizBrace" && (n = !!r.sup, n === r.base.isOver && (t = true, a = r.base.isOver)), r.base && (r.base.type === "op" || r.base.type === "operatorname") && (r.base.parentIsSupSub = true);
      var i = [
        X(r.base, e)
      ];
      r.sub && i.push(X(r.sub, e)), r.sup && i.push(X(r.sup, e));
      var o;
      if (t) o = a ? "mover" : "munder";
      else if (r.sub) if (r.sup) {
        var d = r.base;
        d && d.type === "op" && d.limits && e.style === I.DISPLAY || d && d.type === "operatorname" && d.alwaysHandleSupSub && (e.style === I.DISPLAY || d.limits) ? o = "munderover" : o = "msubsup";
      } else {
        var m = r.base;
        m && m.type === "op" && m.limits && (e.style === I.DISPLAY || m.alwaysHandleSupSub) || m && m.type === "operatorname" && m.alwaysHandleSupSub && (m.limits || e.style === I.DISPLAY) ? o = "munder" : o = "msub";
      }
      else {
        var u = r.base;
        u && u.type === "op" && u.limits && (e.style === I.DISPLAY || u.alwaysHandleSupSub) || u && u.type === "operatorname" && u.alwaysHandleSupSub && (u.limits || e.style === I.DISPLAY) ? o = "mover" : o = "msup";
      }
      return new A.MathNode(o, i);
    }
  });
  ae({
    type: "atom",
    htmlBuilder(r, e) {
      return w.mathsym(r.text, r.mode, e, [
        "m" + r.family
      ]);
    },
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mo", [
        y0(r.text, r.mode)
      ]);
      if (r.family === "bin") {
        var a = tr(r, e);
        a === "bold-italic" && t.setAttribute("mathvariant", a);
      } else r.family === "punct" ? t.setAttribute("separator", "true") : (r.family === "open" || r.family === "close") && t.setAttribute("stretchy", "false");
      return t;
    }
  });
  var u1 = {
    mi: "italic",
    mn: "normal",
    mtext: "normal"
  };
  ae({
    type: "mathord",
    htmlBuilder(r, e) {
      return w.makeOrd(r, e, "mathord");
    },
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mi", [
        y0(r.text, r.mode, e)
      ]), a = tr(r, e) || "italic";
      return a !== u1[t.type] && t.setAttribute("mathvariant", a), t;
    }
  });
  ae({
    type: "textord",
    htmlBuilder(r, e) {
      return w.makeOrd(r, e, "textord");
    },
    mathmlBuilder(r, e) {
      var t = y0(r.text, r.mode, e), a = tr(r, e) || "normal", n;
      return r.mode === "text" ? n = new A.MathNode("mtext", [
        t
      ]) : /[0-9]/.test(r.text) ? n = new A.MathNode("mn", [
        t
      ]) : r.text === "\\prime" ? n = new A.MathNode("mo", [
        t
      ]) : n = new A.MathNode("mi", [
        t
      ]), a !== u1[n.type] && n.setAttribute("mathvariant", a), n;
    }
  });
  var Ct = {
    "\\nobreak": "nobreak",
    "\\allowbreak": "allowbreak"
  }, Dt = {
    " ": {},
    "\\ ": {},
    "~": {
      className: "nobreak"
    },
    "\\space": {},
    "\\nobreakspace": {
      className: "nobreak"
    }
  };
  ae({
    type: "spacing",
    htmlBuilder(r, e) {
      if (Dt.hasOwnProperty(r.text)) {
        var t = Dt[r.text].className || "";
        if (r.mode === "text") {
          var a = w.makeOrd(r, e, "textord");
          return a.classes.push(t), a;
        } else return w.makeSpan([
          "mspace",
          t
        ], [
          w.mathsym(r.text, r.mode, e)
        ], e);
      } else {
        if (Ct.hasOwnProperty(r.text)) return w.makeSpan([
          "mspace",
          Ct[r.text]
        ], [], e);
        throw new T('Unknown type of space "' + r.text + '"');
      }
    },
    mathmlBuilder(r, e) {
      var t;
      if (Dt.hasOwnProperty(r.text)) t = new A.MathNode("mtext", [
        new A.TextNode("\xA0")
      ]);
      else {
        if (Ct.hasOwnProperty(r.text)) return new A.MathNode("mspace");
        throw new T('Unknown type of space "' + r.text + '"');
      }
      return t;
    }
  });
  var Kr = () => {
    var r = new A.MathNode("mtd", []);
    return r.setAttribute("width", "50%"), r;
  };
  ae({
    type: "tag",
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mtable", [
        new A.MathNode("mtr", [
          Kr(),
          new A.MathNode("mtd", [
            J0(r.body, e)
          ]),
          Kr(),
          new A.MathNode("mtd", [
            J0(r.tag, e)
          ])
        ])
      ]);
      return t.setAttribute("width", "100%"), t;
    }
  });
  var _r = {
    "\\text": void 0,
    "\\textrm": "textrm",
    "\\textsf": "textsf",
    "\\texttt": "texttt",
    "\\textnormal": "textrm"
  }, Jr = {
    "\\textbf": "textbf",
    "\\textmd": "textmd"
  }, d4 = {
    "\\textit": "textit",
    "\\textup": "textup"
  }, Qr = (r, e) => {
    var t = r.font;
    if (t) {
      if (_r[t]) return e.withTextFontFamily(_r[t]);
      if (Jr[t]) return e.withTextFontWeight(Jr[t]);
      if (t === "\\emph") return e.fontShape === "textit" ? e.withTextFontShape("textup") : e.withTextFontShape("textit");
    } else return e;
    return e.withTextFontShape(d4[t]);
  };
  D({
    type: "text",
    names: [
      "\\text",
      "\\textrm",
      "\\textsf",
      "\\texttt",
      "\\textnormal",
      "\\textbf",
      "\\textmd",
      "\\textit",
      "\\textup",
      "\\emph"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "text"
      ],
      allowedInArgument: true,
      allowedInText: true
    },
    handler(r, e) {
      var { parser: t, funcName: a } = r, n = e[0];
      return {
        type: "text",
        mode: t.mode,
        body: J(n),
        font: a
      };
    },
    htmlBuilder(r, e) {
      var t = Qr(r, e), a = t0(r.body, t, true);
      return w.makeSpan([
        "mord",
        "text"
      ], a, t);
    },
    mathmlBuilder(r, e) {
      var t = Qr(r, e);
      return J0(r.body, t);
    }
  });
  D({
    type: "underline",
    names: [
      "\\underline"
    ],
    props: {
      numArgs: 1,
      allowedInText: true
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "underline",
        mode: t.mode,
        body: e[0]
      };
    },
    htmlBuilder(r, e) {
      var t = $(r.body, e), a = w.makeLineSpan("underline-line", e), n = e.fontMetrics().defaultRuleThickness, i = w.makeVList({
        positionType: "top",
        positionData: t.height,
        children: [
          {
            type: "kern",
            size: n
          },
          {
            type: "elem",
            elem: a
          },
          {
            type: "kern",
            size: 3 * n
          },
          {
            type: "elem",
            elem: t
          }
        ]
      }, e);
      return w.makeSpan([
        "mord",
        "underline"
      ], [
        i
      ], e);
    },
    mathmlBuilder(r, e) {
      var t = new A.MathNode("mo", [
        new A.TextNode("\u203E")
      ]);
      t.setAttribute("stretchy", "true");
      var a = new A.MathNode("munder", [
        X(r.body, e),
        t
      ]);
      return a.setAttribute("accentunder", "true"), a;
    }
  });
  D({
    type: "vcenter",
    names: [
      "\\vcenter"
    ],
    props: {
      numArgs: 1,
      argTypes: [
        "original"
      ],
      allowedInText: false
    },
    handler(r, e) {
      var { parser: t } = r;
      return {
        type: "vcenter",
        mode: t.mode,
        body: e[0]
      };
    },
    htmlBuilder(r, e) {
      var t = $(r.body, e), a = e.fontMetrics().axisHeight, n = 0.5 * (t.height - a - (t.depth + a));
      return w.makeVList({
        positionType: "shift",
        positionData: n,
        children: [
          {
            type: "elem",
            elem: t
          }
        ]
      }, e);
    },
    mathmlBuilder(r, e) {
      return new A.MathNode("mpadded", [
        X(r.body, e)
      ], [
        "vcenter"
      ]);
    }
  });
  D({
    type: "verb",
    names: [
      "\\verb"
    ],
    props: {
      numArgs: 0,
      allowedInText: true
    },
    handler(r, e, t) {
      throw new T("\\verb ended by end of line instead of matching delimiter");
    },
    htmlBuilder(r, e) {
      for (var t = ea(r), a = [], n = e.havingStyle(e.style.text()), i = 0; i < t.length; i++) {
        var o = t[i];
        o === "~" && (o = "\\textasciitilde"), a.push(w.makeSymbol(o, "Typewriter-Regular", r.mode, n, [
          "mord",
          "texttt"
        ]));
      }
      return w.makeSpan([
        "mord",
        "text"
      ].concat(n.sizingClasses(e)), w.tryCombineChars(a), n);
    },
    mathmlBuilder(r, e) {
      var t = new A.TextNode(ea(r)), a = new A.MathNode("mtext", [
        t
      ]);
      return a.setAttribute("mathvariant", "monospace"), a;
    }
  });
  var ea = (r) => r.body.replace(/ /g, r.star ? "\u2423" : "\xA0"), j0 = qa, h1 = `[ \r
	]`, f4 = "\\\\[a-zA-Z@]+", v4 = "\\\\[^\uD800-\uDFFF]", p4 = "(" + f4 + ")" + h1 + "*", g4 = `\\\\(
|[ \r	]+
?)[ \r	]*`, Xt = "[\u0300-\u036F]", b4 = new RegExp(Xt + "+$"), y4 = "(" + h1 + "+)|" + (g4 + "|") + "([!-\\[\\]-\u2027\u202A-\uD7FF\uF900-\uFFFF]" + (Xt + "*") + "|[\uD800-\uDBFF][\uDC00-\uDFFF]" + (Xt + "*") + "|\\\\verb\\*([^]).*?\\4|\\\\verb([^*a-zA-Z]).*?\\5" + ("|" + p4) + ("|" + v4 + ")");
  class ta {
    constructor(e, t) {
      this.input = void 0, this.settings = void 0, this.tokenRegex = void 0, this.catcodes = void 0, this.input = e, this.settings = t, this.tokenRegex = new RegExp(y4, "g"), this.catcodes = {
        "%": 14,
        "~": 13
      };
    }
    setCatcode(e, t) {
      this.catcodes[e] = t;
    }
    lex() {
      var e = this.input, t = this.tokenRegex.lastIndex;
      if (t === e.length) return new d0("EOF", new h0(this, t, t));
      var a = this.tokenRegex.exec(e);
      if (a === null || a.index !== t) throw new T("Unexpected character: '" + e[t] + "'", new d0(e[t], new h0(this, t, t + 1)));
      var n = a[6] || a[3] || (a[2] ? "\\ " : " ");
      if (this.catcodes[n] === 14) {
        var i = e.indexOf(`
`, this.tokenRegex.lastIndex);
        return i === -1 ? (this.tokenRegex.lastIndex = e.length, this.settings.reportNonstrict("commentAtEnd", "% comment has no terminating newline; LaTeX would fail because of commenting the end of math mode (e.g. $)")) : this.tokenRegex.lastIndex = i + 1, this.lex();
      }
      return new d0(n, new h0(this, t, this.tokenRegex.lastIndex));
    }
  }
  class x4 {
    constructor(e, t) {
      e === void 0 && (e = {}), t === void 0 && (t = {}), this.current = void 0, this.builtins = void 0, this.undefStack = void 0, this.current = t, this.builtins = e, this.undefStack = [];
    }
    beginGroup() {
      this.undefStack.push({});
    }
    endGroup() {
      if (this.undefStack.length === 0) throw new T("Unbalanced namespace destruction: attempt to pop global namespace; please report this as a bug");
      var e = this.undefStack.pop();
      for (var t in e) e.hasOwnProperty(t) && (e[t] == null ? delete this.current[t] : this.current[t] = e[t]);
    }
    endGroups() {
      for (; this.undefStack.length > 0; ) this.endGroup();
    }
    has(e) {
      return this.current.hasOwnProperty(e) || this.builtins.hasOwnProperty(e);
    }
    get(e) {
      return this.current.hasOwnProperty(e) ? this.current[e] : this.builtins[e];
    }
    set(e, t, a) {
      if (a === void 0 && (a = false), a) {
        for (var n = 0; n < this.undefStack.length; n++) delete this.undefStack[n][e];
        this.undefStack.length > 0 && (this.undefStack[this.undefStack.length - 1][e] = t);
      } else {
        var i = this.undefStack[this.undefStack.length - 1];
        i && !i.hasOwnProperty(e) && (i[e] = this.current[e]);
      }
      t == null ? delete this.current[e] : this.current[e] = t;
    }
  }
  var w4 = Qa;
  c("\\noexpand", function(r) {
    var e = r.popToken();
    return r.isExpandable(e.text) && (e.noexpand = true, e.treatAsRelax = true), {
      tokens: [
        e
      ],
      numArgs: 0
    };
  });
  c("\\expandafter", function(r) {
    var e = r.popToken();
    return r.expandOnce(true), {
      tokens: [
        e
      ],
      numArgs: 0
    };
  });
  c("\\@firstoftwo", function(r) {
    var e = r.consumeArgs(2);
    return {
      tokens: e[0],
      numArgs: 0
    };
  });
  c("\\@secondoftwo", function(r) {
    var e = r.consumeArgs(2);
    return {
      tokens: e[1],
      numArgs: 0
    };
  });
  c("\\@ifnextchar", function(r) {
    var e = r.consumeArgs(3);
    r.consumeSpaces();
    var t = r.future();
    return e[0].length === 1 && e[0][0].text === t.text ? {
      tokens: e[1],
      numArgs: 0
    } : {
      tokens: e[2],
      numArgs: 0
    };
  });
  c("\\@ifstar", "\\@ifnextchar *{\\@firstoftwo{#1}}");
  c("\\TextOrMath", function(r) {
    var e = r.consumeArgs(2);
    return r.mode === "text" ? {
      tokens: e[0],
      numArgs: 0
    } : {
      tokens: e[1],
      numArgs: 0
    };
  });
  var ra = {
    0: 0,
    1: 1,
    2: 2,
    3: 3,
    4: 4,
    5: 5,
    6: 6,
    7: 7,
    8: 8,
    9: 9,
    a: 10,
    A: 10,
    b: 11,
    B: 11,
    c: 12,
    C: 12,
    d: 13,
    D: 13,
    e: 14,
    E: 14,
    f: 15,
    F: 15
  };
  c("\\char", function(r) {
    var e = r.popToken(), t, a = "";
    if (e.text === "'") t = 8, e = r.popToken();
    else if (e.text === '"') t = 16, e = r.popToken();
    else if (e.text === "`") if (e = r.popToken(), e.text[0] === "\\") a = e.text.charCodeAt(1);
    else {
      if (e.text === "EOF") throw new T("\\char` missing argument");
      a = e.text.charCodeAt(0);
    }
    else t = 10;
    if (t) {
      if (a = ra[e.text], a == null || a >= t) throw new T("Invalid base-" + t + " digit " + e.text);
      for (var n; (n = ra[r.future().text]) != null && n < t; ) a *= t, a += n, r.popToken();
    }
    return "\\@char{" + a + "}";
  });
  var mr = (r, e, t, a) => {
    var n = r.consumeArg().tokens;
    if (n.length !== 1) throw new T("\\newcommand's first argument must be a macro name");
    var i = n[0].text, o = r.isDefined(i);
    if (o && !e) throw new T("\\newcommand{" + i + "} attempting to redefine " + (i + "; use \\renewcommand"));
    if (!o && !t) throw new T("\\renewcommand{" + i + "} when command " + i + " does not yet exist; use \\newcommand");
    var u = 0;
    if (n = r.consumeArg().tokens, n.length === 1 && n[0].text === "[") {
      for (var m = "", d = r.expandNextToken(); d.text !== "]" && d.text !== "EOF"; ) m += d.text, d = r.expandNextToken();
      if (!m.match(/^\s*[0-9]+\s*$/)) throw new T("Invalid number of arguments: " + m);
      u = parseInt(m), n = r.consumeArg().tokens;
    }
    return o && a || r.macros.set(i, {
      tokens: n,
      numArgs: u
    }), "";
  };
  c("\\newcommand", (r) => mr(r, false, true, false));
  c("\\renewcommand", (r) => mr(r, true, false, false));
  c("\\providecommand", (r) => mr(r, true, true, true));
  c("\\message", (r) => {
    var e = r.consumeArgs(1)[0];
    return console.log(e.reverse().map((t) => t.text).join("")), "";
  });
  c("\\errmessage", (r) => {
    var e = r.consumeArgs(1)[0];
    return console.error(e.reverse().map((t) => t.text).join("")), "";
  });
  c("\\show", (r) => {
    var e = r.popToken(), t = e.text;
    return console.log(e, r.macros.get(t), j0[t], W.math[t], W.text[t]), "";
  });
  c("\\bgroup", "{");
  c("\\egroup", "}");
  c("~", "\\nobreakspace");
  c("\\lq", "`");
  c("\\rq", "'");
  c("\\aa", "\\r a");
  c("\\AA", "\\r A");
  c("\\textcopyright", "\\html@mathml{\\textcircled{c}}{\\char`\xA9}");
  c("\\copyright", "\\TextOrMath{\\textcopyright}{\\text{\\textcopyright}}");
  c("\\textregistered", "\\html@mathml{\\textcircled{\\scriptsize R}}{\\char`\xAE}");
  c("\u212C", "\\mathscr{B}");
  c("\u2130", "\\mathscr{E}");
  c("\u2131", "\\mathscr{F}");
  c("\u210B", "\\mathscr{H}");
  c("\u2110", "\\mathscr{I}");
  c("\u2112", "\\mathscr{L}");
  c("\u2133", "\\mathscr{M}");
  c("\u211B", "\\mathscr{R}");
  c("\u212D", "\\mathfrak{C}");
  c("\u210C", "\\mathfrak{H}");
  c("\u2128", "\\mathfrak{Z}");
  c("\\Bbbk", "\\Bbb{k}");
  c("\xB7", "\\cdotp");
  c("\\llap", "\\mathllap{\\textrm{#1}}");
  c("\\rlap", "\\mathrlap{\\textrm{#1}}");
  c("\\clap", "\\mathclap{\\textrm{#1}}");
  c("\\mathstrut", "\\vphantom{(}");
  c("\\underbar", "\\underline{\\text{#1}}");
  c("\\not", '\\html@mathml{\\mathrel{\\mathrlap\\@not}}{\\char"338}');
  c("\\neq", "\\html@mathml{\\mathrel{\\not=}}{\\mathrel{\\char`\u2260}}");
  c("\\ne", "\\neq");
  c("\u2260", "\\neq");
  c("\\notin", "\\html@mathml{\\mathrel{{\\in}\\mathllap{/\\mskip1mu}}}{\\mathrel{\\char`\u2209}}");
  c("\u2209", "\\notin");
  c("\u2258", "\\html@mathml{\\mathrel{=\\kern{-1em}\\raisebox{0.4em}{$\\scriptsize\\frown$}}}{\\mathrel{\\char`\u2258}}");
  c("\u2259", "\\html@mathml{\\stackrel{\\tiny\\wedge}{=}}{\\mathrel{\\char`\u2258}}");
  c("\u225A", "\\html@mathml{\\stackrel{\\tiny\\vee}{=}}{\\mathrel{\\char`\u225A}}");
  c("\u225B", "\\html@mathml{\\stackrel{\\scriptsize\\star}{=}}{\\mathrel{\\char`\u225B}}");
  c("\u225D", "\\html@mathml{\\stackrel{\\tiny\\mathrm{def}}{=}}{\\mathrel{\\char`\u225D}}");
  c("\u225E", "\\html@mathml{\\stackrel{\\tiny\\mathrm{m}}{=}}{\\mathrel{\\char`\u225E}}");
  c("\u225F", "\\html@mathml{\\stackrel{\\tiny?}{=}}{\\mathrel{\\char`\u225F}}");
  c("\u27C2", "\\perp");
  c("\u203C", "\\mathclose{!\\mkern-0.8mu!}");
  c("\u220C", "\\notni");
  c("\u231C", "\\ulcorner");
  c("\u231D", "\\urcorner");
  c("\u231E", "\\llcorner");
  c("\u231F", "\\lrcorner");
  c("\xA9", "\\copyright");
  c("\xAE", "\\textregistered");
  c("\uFE0F", "\\textregistered");
  c("\\ulcorner", '\\html@mathml{\\@ulcorner}{\\mathop{\\char"231c}}');
  c("\\urcorner", '\\html@mathml{\\@urcorner}{\\mathop{\\char"231d}}');
  c("\\llcorner", '\\html@mathml{\\@llcorner}{\\mathop{\\char"231e}}');
  c("\\lrcorner", '\\html@mathml{\\@lrcorner}{\\mathop{\\char"231f}}');
  c("\\vdots", "{\\varvdots\\rule{0pt}{15pt}}");
  c("\u22EE", "\\vdots");
  c("\\varGamma", "\\mathit{\\Gamma}");
  c("\\varDelta", "\\mathit{\\Delta}");
  c("\\varTheta", "\\mathit{\\Theta}");
  c("\\varLambda", "\\mathit{\\Lambda}");
  c("\\varXi", "\\mathit{\\Xi}");
  c("\\varPi", "\\mathit{\\Pi}");
  c("\\varSigma", "\\mathit{\\Sigma}");
  c("\\varUpsilon", "\\mathit{\\Upsilon}");
  c("\\varPhi", "\\mathit{\\Phi}");
  c("\\varPsi", "\\mathit{\\Psi}");
  c("\\varOmega", "\\mathit{\\Omega}");
  c("\\substack", "\\begin{subarray}{c}#1\\end{subarray}");
  c("\\colon", "\\nobreak\\mskip2mu\\mathpunct{}\\mathchoice{\\mkern-3mu}{\\mkern-3mu}{}{}{:}\\mskip6mu\\relax");
  c("\\boxed", "\\fbox{$\\displaystyle{#1}$}");
  c("\\iff", "\\DOTSB\\;\\Longleftrightarrow\\;");
  c("\\implies", "\\DOTSB\\;\\Longrightarrow\\;");
  c("\\impliedby", "\\DOTSB\\;\\Longleftarrow\\;");
  c("\\dddot", "{\\overset{\\raisebox{-0.1ex}{\\normalsize ...}}{#1}}");
  c("\\ddddot", "{\\overset{\\raisebox{-0.1ex}{\\normalsize ....}}{#1}}");
  var aa = {
    ",": "\\dotsc",
    "\\not": "\\dotsb",
    "+": "\\dotsb",
    "=": "\\dotsb",
    "<": "\\dotsb",
    ">": "\\dotsb",
    "-": "\\dotsb",
    "*": "\\dotsb",
    ":": "\\dotsb",
    "\\DOTSB": "\\dotsb",
    "\\coprod": "\\dotsb",
    "\\bigvee": "\\dotsb",
    "\\bigwedge": "\\dotsb",
    "\\biguplus": "\\dotsb",
    "\\bigcap": "\\dotsb",
    "\\bigcup": "\\dotsb",
    "\\prod": "\\dotsb",
    "\\sum": "\\dotsb",
    "\\bigotimes": "\\dotsb",
    "\\bigoplus": "\\dotsb",
    "\\bigodot": "\\dotsb",
    "\\bigsqcup": "\\dotsb",
    "\\And": "\\dotsb",
    "\\longrightarrow": "\\dotsb",
    "\\Longrightarrow": "\\dotsb",
    "\\longleftarrow": "\\dotsb",
    "\\Longleftarrow": "\\dotsb",
    "\\longleftrightarrow": "\\dotsb",
    "\\Longleftrightarrow": "\\dotsb",
    "\\mapsto": "\\dotsb",
    "\\longmapsto": "\\dotsb",
    "\\hookrightarrow": "\\dotsb",
    "\\doteq": "\\dotsb",
    "\\mathbin": "\\dotsb",
    "\\mathrel": "\\dotsb",
    "\\relbar": "\\dotsb",
    "\\Relbar": "\\dotsb",
    "\\xrightarrow": "\\dotsb",
    "\\xleftarrow": "\\dotsb",
    "\\DOTSI": "\\dotsi",
    "\\int": "\\dotsi",
    "\\oint": "\\dotsi",
    "\\iint": "\\dotsi",
    "\\iiint": "\\dotsi",
    "\\iiiint": "\\dotsi",
    "\\idotsint": "\\dotsi",
    "\\DOTSX": "\\dotsx"
  };
  c("\\dots", function(r) {
    var e = "\\dotso", t = r.expandAfterFuture().text;
    return t in aa ? e = aa[t] : (t.slice(0, 4) === "\\not" || t in W.math && [
      "bin",
      "rel"
    ].includes(W.math[t].group)) && (e = "\\dotsb"), e;
  });
  var cr = {
    ")": true,
    "]": true,
    "\\rbrack": true,
    "\\}": true,
    "\\rbrace": true,
    "\\rangle": true,
    "\\rceil": true,
    "\\rfloor": true,
    "\\rgroup": true,
    "\\rmoustache": true,
    "\\right": true,
    "\\bigr": true,
    "\\biggr": true,
    "\\Bigr": true,
    "\\Biggr": true,
    $: true,
    ";": true,
    ".": true,
    ",": true
  };
  c("\\dotso", function(r) {
    var e = r.future().text;
    return e in cr ? "\\ldots\\," : "\\ldots";
  });
  c("\\dotsc", function(r) {
    var e = r.future().text;
    return e in cr && e !== "," ? "\\ldots\\," : "\\ldots";
  });
  c("\\cdots", function(r) {
    var e = r.future().text;
    return e in cr ? "\\@cdots\\," : "\\@cdots";
  });
  c("\\dotsb", "\\cdots");
  c("\\dotsm", "\\cdots");
  c("\\dotsi", "\\!\\cdots");
  c("\\dotsx", "\\ldots\\,");
  c("\\DOTSI", "\\relax");
  c("\\DOTSB", "\\relax");
  c("\\DOTSX", "\\relax");
  c("\\tmspace", "\\TextOrMath{\\kern#1#3}{\\mskip#1#2}\\relax");
  c("\\,", "\\tmspace+{3mu}{.1667em}");
  c("\\thinspace", "\\,");
  c("\\>", "\\mskip{4mu}");
  c("\\:", "\\tmspace+{4mu}{.2222em}");
  c("\\medspace", "\\:");
  c("\\;", "\\tmspace+{5mu}{.2777em}");
  c("\\thickspace", "\\;");
  c("\\!", "\\tmspace-{3mu}{.1667em}");
  c("\\negthinspace", "\\!");
  c("\\negmedspace", "\\tmspace-{4mu}{.2222em}");
  c("\\negthickspace", "\\tmspace-{5mu}{.277em}");
  c("\\enspace", "\\kern.5em ");
  c("\\enskip", "\\hskip.5em\\relax");
  c("\\quad", "\\hskip1em\\relax");
  c("\\qquad", "\\hskip2em\\relax");
  c("\\tag", "\\@ifstar\\tag@literal\\tag@paren");
  c("\\tag@paren", "\\tag@literal{({#1})}");
  c("\\tag@literal", (r) => {
    if (r.macros.get("\\df@tag")) throw new T("Multiple \\tag");
    return "\\gdef\\df@tag{\\text{#1}}";
  });
  c("\\bmod", "\\mathchoice{\\mskip1mu}{\\mskip1mu}{\\mskip5mu}{\\mskip5mu}\\mathbin{\\rm mod}\\mathchoice{\\mskip1mu}{\\mskip1mu}{\\mskip5mu}{\\mskip5mu}");
  c("\\pod", "\\allowbreak\\mathchoice{\\mkern18mu}{\\mkern8mu}{\\mkern8mu}{\\mkern8mu}(#1)");
  c("\\pmod", "\\pod{{\\rm mod}\\mkern6mu#1}");
  c("\\mod", "\\allowbreak\\mathchoice{\\mkern18mu}{\\mkern12mu}{\\mkern12mu}{\\mkern12mu}{\\rm mod}\\,\\,#1");
  c("\\newline", "\\\\\\relax");
  c("\\TeX", "\\textrm{\\html@mathml{T\\kern-.1667em\\raisebox{-.5ex}{E}\\kern-.125emX}{TeX}}");
  var m1 = C(T0["Main-Regular"][84][1] - 0.7 * T0["Main-Regular"][65][1]);
  c("\\LaTeX", "\\textrm{\\html@mathml{" + ("L\\kern-.36em\\raisebox{" + m1 + "}{\\scriptstyle A}") + "\\kern-.15em\\TeX}{LaTeX}}");
  c("\\KaTeX", "\\textrm{\\html@mathml{" + ("K\\kern-.17em\\raisebox{" + m1 + "}{\\scriptstyle A}") + "\\kern-.15em\\TeX}{KaTeX}}");
  c("\\hspace", "\\@ifstar\\@hspacer\\@hspace");
  c("\\@hspace", "\\hskip #1\\relax");
  c("\\@hspacer", "\\rule{0pt}{0pt}\\hskip #1\\relax");
  c("\\ordinarycolon", ":");
  c("\\vcentcolon", "\\mathrel{\\mathop\\ordinarycolon}");
  c("\\dblcolon", '\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-.9mu}\\vcentcolon}}{\\mathop{\\char"2237}}');
  c("\\coloneqq", '\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}=}}{\\mathop{\\char"2254}}');
  c("\\Coloneqq", '\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}=}}{\\mathop{\\char"2237\\char"3d}}');
  c("\\coloneq", '\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\mathrel{-}}}{\\mathop{\\char"3a\\char"2212}}');
  c("\\Coloneq", '\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\mathrel{-}}}{\\mathop{\\char"2237\\char"2212}}');
  c("\\eqqcolon", '\\html@mathml{\\mathrel{=\\mathrel{\\mkern-1.2mu}\\vcentcolon}}{\\mathop{\\char"2255}}');
  c("\\Eqqcolon", '\\html@mathml{\\mathrel{=\\mathrel{\\mkern-1.2mu}\\dblcolon}}{\\mathop{\\char"3d\\char"2237}}');
  c("\\eqcolon", '\\html@mathml{\\mathrel{\\mathrel{-}\\mathrel{\\mkern-1.2mu}\\vcentcolon}}{\\mathop{\\char"2239}}');
  c("\\Eqcolon", '\\html@mathml{\\mathrel{\\mathrel{-}\\mathrel{\\mkern-1.2mu}\\dblcolon}}{\\mathop{\\char"2212\\char"2237}}');
  c("\\colonapprox", '\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\approx}}{\\mathop{\\char"3a\\char"2248}}');
  c("\\Colonapprox", '\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\approx}}{\\mathop{\\char"2237\\char"2248}}');
  c("\\colonsim", '\\html@mathml{\\mathrel{\\vcentcolon\\mathrel{\\mkern-1.2mu}\\sim}}{\\mathop{\\char"3a\\char"223c}}');
  c("\\Colonsim", '\\html@mathml{\\mathrel{\\dblcolon\\mathrel{\\mkern-1.2mu}\\sim}}{\\mathop{\\char"2237\\char"223c}}');
  c("\u2237", "\\dblcolon");
  c("\u2239", "\\eqcolon");
  c("\u2254", "\\coloneqq");
  c("\u2255", "\\eqqcolon");
  c("\u2A74", "\\Coloneqq");
  c("\\ratio", "\\vcentcolon");
  c("\\coloncolon", "\\dblcolon");
  c("\\colonequals", "\\coloneqq");
  c("\\coloncolonequals", "\\Coloneqq");
  c("\\equalscolon", "\\eqqcolon");
  c("\\equalscoloncolon", "\\Eqqcolon");
  c("\\colonminus", "\\coloneq");
  c("\\coloncolonminus", "\\Coloneq");
  c("\\minuscolon", "\\eqcolon");
  c("\\minuscoloncolon", "\\Eqcolon");
  c("\\coloncolonapprox", "\\Colonapprox");
  c("\\coloncolonsim", "\\Colonsim");
  c("\\simcolon", "\\mathrel{\\sim\\mathrel{\\mkern-1.2mu}\\vcentcolon}");
  c("\\simcoloncolon", "\\mathrel{\\sim\\mathrel{\\mkern-1.2mu}\\dblcolon}");
  c("\\approxcolon", "\\mathrel{\\approx\\mathrel{\\mkern-1.2mu}\\vcentcolon}");
  c("\\approxcoloncolon", "\\mathrel{\\approx\\mathrel{\\mkern-1.2mu}\\dblcolon}");
  c("\\notni", "\\html@mathml{\\not\\ni}{\\mathrel{\\char`\u220C}}");
  c("\\limsup", "\\DOTSB\\operatorname*{lim\\,sup}");
  c("\\liminf", "\\DOTSB\\operatorname*{lim\\,inf}");
  c("\\injlim", "\\DOTSB\\operatorname*{inj\\,lim}");
  c("\\projlim", "\\DOTSB\\operatorname*{proj\\,lim}");
  c("\\varlimsup", "\\DOTSB\\operatorname*{\\overline{lim}}");
  c("\\varliminf", "\\DOTSB\\operatorname*{\\underline{lim}}");
  c("\\varinjlim", "\\DOTSB\\operatorname*{\\underrightarrow{lim}}");
  c("\\varprojlim", "\\DOTSB\\operatorname*{\\underleftarrow{lim}}");
  c("\\gvertneqq", "\\html@mathml{\\@gvertneqq}{\u2269}");
  c("\\lvertneqq", "\\html@mathml{\\@lvertneqq}{\u2268}");
  c("\\ngeqq", "\\html@mathml{\\@ngeqq}{\u2271}");
  c("\\ngeqslant", "\\html@mathml{\\@ngeqslant}{\u2271}");
  c("\\nleqq", "\\html@mathml{\\@nleqq}{\u2270}");
  c("\\nleqslant", "\\html@mathml{\\@nleqslant}{\u2270}");
  c("\\nshortmid", "\\html@mathml{\\@nshortmid}{\u2224}");
  c("\\nshortparallel", "\\html@mathml{\\@nshortparallel}{\u2226}");
  c("\\nsubseteqq", "\\html@mathml{\\@nsubseteqq}{\u2288}");
  c("\\nsupseteqq", "\\html@mathml{\\@nsupseteqq}{\u2289}");
  c("\\varsubsetneq", "\\html@mathml{\\@varsubsetneq}{\u228A}");
  c("\\varsubsetneqq", "\\html@mathml{\\@varsubsetneqq}{\u2ACB}");
  c("\\varsupsetneq", "\\html@mathml{\\@varsupsetneq}{\u228B}");
  c("\\varsupsetneqq", "\\html@mathml{\\@varsupsetneqq}{\u2ACC}");
  c("\\imath", "\\html@mathml{\\@imath}{\u0131}");
  c("\\jmath", "\\html@mathml{\\@jmath}{\u0237}");
  c("\\llbracket", "\\html@mathml{\\mathopen{[\\mkern-3.2mu[}}{\\mathopen{\\char`\u27E6}}");
  c("\\rrbracket", "\\html@mathml{\\mathclose{]\\mkern-3.2mu]}}{\\mathclose{\\char`\u27E7}}");
  c("\u27E6", "\\llbracket");
  c("\u27E7", "\\rrbracket");
  c("\\lBrace", "\\html@mathml{\\mathopen{\\{\\mkern-3.2mu[}}{\\mathopen{\\char`\u2983}}");
  c("\\rBrace", "\\html@mathml{\\mathclose{]\\mkern-3.2mu\\}}}{\\mathclose{\\char`\u2984}}");
  c("\u2983", "\\lBrace");
  c("\u2984", "\\rBrace");
  c("\\minuso", "\\mathbin{\\html@mathml{{\\mathrlap{\\mathchoice{\\kern{0.145em}}{\\kern{0.145em}}{\\kern{0.1015em}}{\\kern{0.0725em}}\\circ}{-}}}{\\char`\u29B5}}");
  c("\u29B5", "\\minuso");
  c("\\darr", "\\downarrow");
  c("\\dArr", "\\Downarrow");
  c("\\Darr", "\\Downarrow");
  c("\\lang", "\\langle");
  c("\\rang", "\\rangle");
  c("\\uarr", "\\uparrow");
  c("\\uArr", "\\Uparrow");
  c("\\Uarr", "\\Uparrow");
  c("\\N", "\\mathbb{N}");
  c("\\R", "\\mathbb{R}");
  c("\\Z", "\\mathbb{Z}");
  c("\\alef", "\\aleph");
  c("\\alefsym", "\\aleph");
  c("\\Alpha", "\\mathrm{A}");
  c("\\Beta", "\\mathrm{B}");
  c("\\bull", "\\bullet");
  c("\\Chi", "\\mathrm{X}");
  c("\\clubs", "\\clubsuit");
  c("\\cnums", "\\mathbb{C}");
  c("\\Complex", "\\mathbb{C}");
  c("\\Dagger", "\\ddagger");
  c("\\diamonds", "\\diamondsuit");
  c("\\empty", "\\emptyset");
  c("\\Epsilon", "\\mathrm{E}");
  c("\\Eta", "\\mathrm{H}");
  c("\\exist", "\\exists");
  c("\\harr", "\\leftrightarrow");
  c("\\hArr", "\\Leftrightarrow");
  c("\\Harr", "\\Leftrightarrow");
  c("\\hearts", "\\heartsuit");
  c("\\image", "\\Im");
  c("\\infin", "\\infty");
  c("\\Iota", "\\mathrm{I}");
  c("\\isin", "\\in");
  c("\\Kappa", "\\mathrm{K}");
  c("\\larr", "\\leftarrow");
  c("\\lArr", "\\Leftarrow");
  c("\\Larr", "\\Leftarrow");
  c("\\lrarr", "\\leftrightarrow");
  c("\\lrArr", "\\Leftrightarrow");
  c("\\Lrarr", "\\Leftrightarrow");
  c("\\Mu", "\\mathrm{M}");
  c("\\natnums", "\\mathbb{N}");
  c("\\Nu", "\\mathrm{N}");
  c("\\Omicron", "\\mathrm{O}");
  c("\\plusmn", "\\pm");
  c("\\rarr", "\\rightarrow");
  c("\\rArr", "\\Rightarrow");
  c("\\Rarr", "\\Rightarrow");
  c("\\real", "\\Re");
  c("\\reals", "\\mathbb{R}");
  c("\\Reals", "\\mathbb{R}");
  c("\\Rho", "\\mathrm{P}");
  c("\\sdot", "\\cdot");
  c("\\sect", "\\S");
  c("\\spades", "\\spadesuit");
  c("\\sub", "\\subset");
  c("\\sube", "\\subseteq");
  c("\\supe", "\\supseteq");
  c("\\Tau", "\\mathrm{T}");
  c("\\thetasym", "\\vartheta");
  c("\\weierp", "\\wp");
  c("\\Zeta", "\\mathrm{Z}");
  c("\\argmin", "\\DOTSB\\operatorname*{arg\\,min}");
  c("\\argmax", "\\DOTSB\\operatorname*{arg\\,max}");
  c("\\plim", "\\DOTSB\\mathop{\\operatorname{plim}}\\limits");
  c("\\bra", "\\mathinner{\\langle{#1}|}");
  c("\\ket", "\\mathinner{|{#1}\\rangle}");
  c("\\braket", "\\mathinner{\\langle{#1}\\rangle}");
  c("\\Bra", "\\left\\langle#1\\right|");
  c("\\Ket", "\\left|#1\\right\\rangle");
  var c1 = (r) => (e) => {
    var t = e.consumeArg().tokens, a = e.consumeArg().tokens, n = e.consumeArg().tokens, i = e.consumeArg().tokens, o = e.macros.get("|"), u = e.macros.get("\\|");
    e.macros.beginGroup();
    var m = (b) => (y) => {
      r && (y.macros.set("|", o), n.length && y.macros.set("\\|", u));
      var x = b;
      if (!b && n.length) {
        var k = y.future();
        k.text === "|" && (y.popToken(), x = true);
      }
      return {
        tokens: x ? n : a,
        numArgs: 0
      };
    };
    e.macros.set("|", m(false)), n.length && e.macros.set("\\|", m(true));
    var d = e.consumeArg().tokens, p = e.expandTokens([
      ...i,
      ...d,
      ...t
    ]);
    return e.macros.endGroup(), {
      tokens: p.reverse(),
      numArgs: 0
    };
  };
  c("\\bra@ket", c1(false));
  c("\\bra@set", c1(true));
  c("\\Braket", "\\bra@ket{\\left\\langle}{\\,\\middle\\vert\\,}{\\,\\middle\\vert\\,}{\\right\\rangle}");
  c("\\Set", "\\bra@set{\\left\\{\\:}{\\;\\middle\\vert\\;}{\\;\\middle\\Vert\\;}{\\:\\right\\}}");
  c("\\set", "\\bra@set{\\{\\,}{\\mid}{}{\\,\\}}");
  c("\\angln", "{\\angl n}");
  c("\\blue", "\\textcolor{##6495ed}{#1}");
  c("\\orange", "\\textcolor{##ffa500}{#1}");
  c("\\pink", "\\textcolor{##ff00af}{#1}");
  c("\\red", "\\textcolor{##df0030}{#1}");
  c("\\green", "\\textcolor{##28ae7b}{#1}");
  c("\\gray", "\\textcolor{gray}{#1}");
  c("\\purple", "\\textcolor{##9d38bd}{#1}");
  c("\\blueA", "\\textcolor{##ccfaff}{#1}");
  c("\\blueB", "\\textcolor{##80f6ff}{#1}");
  c("\\blueC", "\\textcolor{##63d9ea}{#1}");
  c("\\blueD", "\\textcolor{##11accd}{#1}");
  c("\\blueE", "\\textcolor{##0c7f99}{#1}");
  c("\\tealA", "\\textcolor{##94fff5}{#1}");
  c("\\tealB", "\\textcolor{##26edd5}{#1}");
  c("\\tealC", "\\textcolor{##01d1c1}{#1}");
  c("\\tealD", "\\textcolor{##01a995}{#1}");
  c("\\tealE", "\\textcolor{##208170}{#1}");
  c("\\greenA", "\\textcolor{##b6ffb0}{#1}");
  c("\\greenB", "\\textcolor{##8af281}{#1}");
  c("\\greenC", "\\textcolor{##74cf70}{#1}");
  c("\\greenD", "\\textcolor{##1fab54}{#1}");
  c("\\greenE", "\\textcolor{##0d923f}{#1}");
  c("\\goldA", "\\textcolor{##ffd0a9}{#1}");
  c("\\goldB", "\\textcolor{##ffbb71}{#1}");
  c("\\goldC", "\\textcolor{##ff9c39}{#1}");
  c("\\goldD", "\\textcolor{##e07d10}{#1}");
  c("\\goldE", "\\textcolor{##a75a05}{#1}");
  c("\\redA", "\\textcolor{##fca9a9}{#1}");
  c("\\redB", "\\textcolor{##ff8482}{#1}");
  c("\\redC", "\\textcolor{##f9685d}{#1}");
  c("\\redD", "\\textcolor{##e84d39}{#1}");
  c("\\redE", "\\textcolor{##bc2612}{#1}");
  c("\\maroonA", "\\textcolor{##ffbde0}{#1}");
  c("\\maroonB", "\\textcolor{##ff92c6}{#1}");
  c("\\maroonC", "\\textcolor{##ed5fa6}{#1}");
  c("\\maroonD", "\\textcolor{##ca337c}{#1}");
  c("\\maroonE", "\\textcolor{##9e034e}{#1}");
  c("\\purpleA", "\\textcolor{##ddd7ff}{#1}");
  c("\\purpleB", "\\textcolor{##c6b9fc}{#1}");
  c("\\purpleC", "\\textcolor{##aa87ff}{#1}");
  c("\\purpleD", "\\textcolor{##7854ab}{#1}");
  c("\\purpleE", "\\textcolor{##543b78}{#1}");
  c("\\mintA", "\\textcolor{##f5f9e8}{#1}");
  c("\\mintB", "\\textcolor{##edf2df}{#1}");
  c("\\mintC", "\\textcolor{##e0e5cc}{#1}");
  c("\\grayA", "\\textcolor{##f6f7f7}{#1}");
  c("\\grayB", "\\textcolor{##f0f1f2}{#1}");
  c("\\grayC", "\\textcolor{##e3e5e6}{#1}");
  c("\\grayD", "\\textcolor{##d6d8da}{#1}");
  c("\\grayE", "\\textcolor{##babec2}{#1}");
  c("\\grayF", "\\textcolor{##888d93}{#1}");
  c("\\grayG", "\\textcolor{##626569}{#1}");
  c("\\grayH", "\\textcolor{##3b3e40}{#1}");
  c("\\grayI", "\\textcolor{##21242c}{#1}");
  c("\\kaBlue", "\\textcolor{##314453}{#1}");
  c("\\kaGreen", "\\textcolor{##71B307}{#1}");
  var d1 = {
    "^": true,
    _: true,
    "\\limits": true,
    "\\nolimits": true
  };
  class k4 {
    constructor(e, t, a) {
      this.settings = void 0, this.expansionCount = void 0, this.lexer = void 0, this.macros = void 0, this.stack = void 0, this.mode = void 0, this.settings = t, this.expansionCount = 0, this.feed(e), this.macros = new x4(w4, t.macros), this.mode = a, this.stack = [];
    }
    feed(e) {
      this.lexer = new ta(e, this.settings);
    }
    switchMode(e) {
      this.mode = e;
    }
    beginGroup() {
      this.macros.beginGroup();
    }
    endGroup() {
      this.macros.endGroup();
    }
    endGroups() {
      this.macros.endGroups();
    }
    future() {
      return this.stack.length === 0 && this.pushToken(this.lexer.lex()), this.stack[this.stack.length - 1];
    }
    popToken() {
      return this.future(), this.stack.pop();
    }
    pushToken(e) {
      this.stack.push(e);
    }
    pushTokens(e) {
      this.stack.push(...e);
    }
    scanArgument(e) {
      var t, a, n;
      if (e) {
        if (this.consumeSpaces(), this.future().text !== "[") return null;
        t = this.popToken(), { tokens: n, end: a } = this.consumeArg([
          "]"
        ]);
      } else ({ tokens: n, start: t, end: a } = this.consumeArg());
      return this.pushToken(new d0("EOF", a.loc)), this.pushTokens(n), new d0("", h0.range(t, a));
    }
    consumeSpaces() {
      for (; ; ) {
        var e = this.future();
        if (e.text === " ") this.stack.pop();
        else break;
      }
    }
    consumeArg(e) {
      var t = [], a = e && e.length > 0;
      a || this.consumeSpaces();
      var n = this.future(), i, o = 0, u = 0;
      do {
        if (i = this.popToken(), t.push(i), i.text === "{") ++o;
        else if (i.text === "}") {
          if (--o, o === -1) throw new T("Extra }", i);
        } else if (i.text === "EOF") throw new T("Unexpected end of input in a macro argument, expected '" + (e && a ? e[u] : "}") + "'", i);
        if (e && a) if ((o === 0 || o === 1 && e[u] === "{") && i.text === e[u]) {
          if (++u, u === e.length) {
            t.splice(-u, u);
            break;
          }
        } else u = 0;
      } while (o !== 0 || a);
      return n.text === "{" && t[t.length - 1].text === "}" && (t.pop(), t.shift()), t.reverse(), {
        tokens: t,
        start: n,
        end: i
      };
    }
    consumeArgs(e, t) {
      if (t) {
        if (t.length !== e + 1) throw new T("The length of delimiters doesn't match the number of args!");
        for (var a = t[0], n = 0; n < a.length; n++) {
          var i = this.popToken();
          if (a[n] !== i.text) throw new T("Use of the macro doesn't match its definition", i);
        }
      }
      for (var o = [], u = 0; u < e; u++) o.push(this.consumeArg(t && t[u + 1]).tokens);
      return o;
    }
    countExpansion(e) {
      if (this.expansionCount += e, this.expansionCount > this.settings.maxExpand) throw new T("Too many expansions: infinite loop or need to increase maxExpand setting");
    }
    expandOnce(e) {
      var t = this.popToken(), a = t.text, n = t.noexpand ? null : this._getExpansion(a);
      if (n == null || e && n.unexpandable) {
        if (e && n == null && a[0] === "\\" && !this.isDefined(a)) throw new T("Undefined control sequence: " + a);
        return this.pushToken(t), false;
      }
      this.countExpansion(1);
      var i = n.tokens, o = this.consumeArgs(n.numArgs, n.delimiters);
      if (n.numArgs) {
        i = i.slice();
        for (var u = i.length - 1; u >= 0; --u) {
          var m = i[u];
          if (m.text === "#") {
            if (u === 0) throw new T("Incomplete placeholder at end of macro body", m);
            if (m = i[--u], m.text === "#") i.splice(u + 1, 1);
            else if (/^[1-9]$/.test(m.text)) i.splice(u, 2, ...o[+m.text - 1]);
            else throw new T("Not a valid argument number", m);
          }
        }
      }
      return this.pushTokens(i), i.length;
    }
    expandAfterFuture() {
      return this.expandOnce(), this.future();
    }
    expandNextToken() {
      for (; ; ) if (this.expandOnce() === false) {
        var e = this.stack.pop();
        return e.treatAsRelax && (e.text = "\\relax"), e;
      }
      throw new Error();
    }
    expandMacro(e) {
      return this.macros.has(e) ? this.expandTokens([
        new d0(e)
      ]) : void 0;
    }
    expandTokens(e) {
      var t = [], a = this.stack.length;
      for (this.pushTokens(e); this.stack.length > a; ) if (this.expandOnce(true) === false) {
        var n = this.stack.pop();
        n.treatAsRelax && (n.noexpand = false, n.treatAsRelax = false), t.push(n);
      }
      return this.countExpansion(t.length), t;
    }
    expandMacroAsText(e) {
      var t = this.expandMacro(e);
      return t && t.map((a) => a.text).join("");
    }
    _getExpansion(e) {
      var t = this.macros.get(e);
      if (t == null) return t;
      if (e.length === 1) {
        var a = this.lexer.catcodes[e];
        if (a != null && a !== 13) return;
      }
      var n = typeof t == "function" ? t(this) : t;
      if (typeof n == "string") {
        var i = 0;
        if (n.indexOf("#") !== -1) for (var o = n.replace(/##/g, ""); o.indexOf("#" + (i + 1)) !== -1; ) ++i;
        for (var u = new ta(n, this.settings), m = [], d = u.lex(); d.text !== "EOF"; ) m.push(d), d = u.lex();
        m.reverse();
        var p = {
          tokens: m,
          numArgs: i
        };
        return p;
      }
      return n;
    }
    isDefined(e) {
      return this.macros.has(e) || j0.hasOwnProperty(e) || W.math.hasOwnProperty(e) || W.text.hasOwnProperty(e) || d1.hasOwnProperty(e);
    }
    isExpandable(e) {
      var t = this.macros.get(e);
      return t != null ? typeof t == "string" || typeof t == "function" || !t.unexpandable : j0.hasOwnProperty(e) && !j0[e].primitive;
    }
  }
  var na = /^[₊₋₌₍₎₀₁₂₃₄₅₆₇₈₉ₐₑₕᵢⱼₖₗₘₙₒₚᵣₛₜᵤᵥₓᵦᵧᵨᵩᵪ]/, Ge = Object.freeze({
    "\u208A": "+",
    "\u208B": "-",
    "\u208C": "=",
    "\u208D": "(",
    "\u208E": ")",
    "\u2080": "0",
    "\u2081": "1",
    "\u2082": "2",
    "\u2083": "3",
    "\u2084": "4",
    "\u2085": "5",
    "\u2086": "6",
    "\u2087": "7",
    "\u2088": "8",
    "\u2089": "9",
    "\u2090": "a",
    "\u2091": "e",
    "\u2095": "h",
    "\u1D62": "i",
    "\u2C7C": "j",
    "\u2096": "k",
    "\u2097": "l",
    "\u2098": "m",
    "\u2099": "n",
    "\u2092": "o",
    "\u209A": "p",
    "\u1D63": "r",
    "\u209B": "s",
    "\u209C": "t",
    "\u1D64": "u",
    "\u1D65": "v",
    "\u2093": "x",
    "\u1D66": "\u03B2",
    "\u1D67": "\u03B3",
    "\u1D68": "\u03C1",
    "\u1D69": "\u03D5",
    "\u1D6A": "\u03C7",
    "\u207A": "+",
    "\u207B": "-",
    "\u207C": "=",
    "\u207D": "(",
    "\u207E": ")",
    "\u2070": "0",
    "\xB9": "1",
    "\xB2": "2",
    "\xB3": "3",
    "\u2074": "4",
    "\u2075": "5",
    "\u2076": "6",
    "\u2077": "7",
    "\u2078": "8",
    "\u2079": "9",
    "\u1D2C": "A",
    "\u1D2E": "B",
    "\u1D30": "D",
    "\u1D31": "E",
    "\u1D33": "G",
    "\u1D34": "H",
    "\u1D35": "I",
    "\u1D36": "J",
    "\u1D37": "K",
    "\u1D38": "L",
    "\u1D39": "M",
    "\u1D3A": "N",
    "\u1D3C": "O",
    "\u1D3E": "P",
    "\u1D3F": "R",
    "\u1D40": "T",
    "\u1D41": "U",
    "\u2C7D": "V",
    "\u1D42": "W",
    "\u1D43": "a",
    "\u1D47": "b",
    "\u1D9C": "c",
    "\u1D48": "d",
    "\u1D49": "e",
    "\u1DA0": "f",
    "\u1D4D": "g",
    \u02B0: "h",
    "\u2071": "i",
    \u02B2: "j",
    "\u1D4F": "k",
    \u02E1: "l",
    "\u1D50": "m",
    \u207F: "n",
    "\u1D52": "o",
    "\u1D56": "p",
    \u02B3: "r",
    \u02E2: "s",
    "\u1D57": "t",
    "\u1D58": "u",
    "\u1D5B": "v",
    \u02B7: "w",
    \u02E3: "x",
    \u02B8: "y",
    "\u1DBB": "z",
    "\u1D5D": "\u03B2",
    "\u1D5E": "\u03B3",
    "\u1D5F": "\u03B4",
    "\u1D60": "\u03D5",
    "\u1D61": "\u03C7",
    "\u1DBF": "\u03B8"
  }), Nt = {
    "\u0301": {
      text: "\\'",
      math: "\\acute"
    },
    "\u0300": {
      text: "\\`",
      math: "\\grave"
    },
    "\u0308": {
      text: '\\"',
      math: "\\ddot"
    },
    "\u0303": {
      text: "\\~",
      math: "\\tilde"
    },
    "\u0304": {
      text: "\\=",
      math: "\\bar"
    },
    "\u0306": {
      text: "\\u",
      math: "\\breve"
    },
    "\u030C": {
      text: "\\v",
      math: "\\check"
    },
    "\u0302": {
      text: "\\^",
      math: "\\hat"
    },
    "\u0307": {
      text: "\\.",
      math: "\\dot"
    },
    "\u030A": {
      text: "\\r",
      math: "\\mathring"
    },
    "\u030B": {
      text: "\\H"
    },
    "\u0327": {
      text: "\\c"
    }
  }, ia = {
    \u00E1: "a\u0301",
    \u00E0: "a\u0300",
    \u00E4: "a\u0308",
    \u01DF: "a\u0308\u0304",
    \u00E3: "a\u0303",
    \u0101: "a\u0304",
    \u0103: "a\u0306",
    \u1EAF: "a\u0306\u0301",
    \u1EB1: "a\u0306\u0300",
    \u1EB5: "a\u0306\u0303",
    \u01CE: "a\u030C",
    \u00E2: "a\u0302",
    \u1EA5: "a\u0302\u0301",
    \u1EA7: "a\u0302\u0300",
    \u1EAB: "a\u0302\u0303",
    \u0227: "a\u0307",
    \u01E1: "a\u0307\u0304",
    \u00E5: "a\u030A",
    \u01FB: "a\u030A\u0301",
    \u1E03: "b\u0307",
    \u0107: "c\u0301",
    \u1E09: "c\u0327\u0301",
    \u010D: "c\u030C",
    \u0109: "c\u0302",
    \u010B: "c\u0307",
    \u00E7: "c\u0327",
    \u010F: "d\u030C",
    \u1E0B: "d\u0307",
    \u1E11: "d\u0327",
    \u00E9: "e\u0301",
    \u00E8: "e\u0300",
    \u00EB: "e\u0308",
    \u1EBD: "e\u0303",
    \u0113: "e\u0304",
    \u1E17: "e\u0304\u0301",
    \u1E15: "e\u0304\u0300",
    \u0115: "e\u0306",
    \u1E1D: "e\u0327\u0306",
    \u011B: "e\u030C",
    \u00EA: "e\u0302",
    \u1EBF: "e\u0302\u0301",
    \u1EC1: "e\u0302\u0300",
    \u1EC5: "e\u0302\u0303",
    \u0117: "e\u0307",
    \u0229: "e\u0327",
    \u1E1F: "f\u0307",
    \u01F5: "g\u0301",
    \u1E21: "g\u0304",
    \u011F: "g\u0306",
    \u01E7: "g\u030C",
    \u011D: "g\u0302",
    \u0121: "g\u0307",
    \u0123: "g\u0327",
    \u1E27: "h\u0308",
    \u021F: "h\u030C",
    \u0125: "h\u0302",
    \u1E23: "h\u0307",
    \u1E29: "h\u0327",
    \u00ED: "i\u0301",
    \u00EC: "i\u0300",
    \u00EF: "i\u0308",
    \u1E2F: "i\u0308\u0301",
    \u0129: "i\u0303",
    \u012B: "i\u0304",
    \u012D: "i\u0306",
    \u01D0: "i\u030C",
    \u00EE: "i\u0302",
    \u01F0: "j\u030C",
    \u0135: "j\u0302",
    \u1E31: "k\u0301",
    \u01E9: "k\u030C",
    \u0137: "k\u0327",
    \u013A: "l\u0301",
    \u013E: "l\u030C",
    \u013C: "l\u0327",
    \u1E3F: "m\u0301",
    \u1E41: "m\u0307",
    \u0144: "n\u0301",
    \u01F9: "n\u0300",
    \u00F1: "n\u0303",
    \u0148: "n\u030C",
    \u1E45: "n\u0307",
    \u0146: "n\u0327",
    \u00F3: "o\u0301",
    \u00F2: "o\u0300",
    \u00F6: "o\u0308",
    \u022B: "o\u0308\u0304",
    \u00F5: "o\u0303",
    \u1E4D: "o\u0303\u0301",
    \u1E4F: "o\u0303\u0308",
    \u022D: "o\u0303\u0304",
    \u014D: "o\u0304",
    \u1E53: "o\u0304\u0301",
    \u1E51: "o\u0304\u0300",
    \u014F: "o\u0306",
    \u01D2: "o\u030C",
    \u00F4: "o\u0302",
    \u1ED1: "o\u0302\u0301",
    \u1ED3: "o\u0302\u0300",
    \u1ED7: "o\u0302\u0303",
    \u022F: "o\u0307",
    \u0231: "o\u0307\u0304",
    \u0151: "o\u030B",
    \u1E55: "p\u0301",
    \u1E57: "p\u0307",
    \u0155: "r\u0301",
    \u0159: "r\u030C",
    \u1E59: "r\u0307",
    \u0157: "r\u0327",
    \u015B: "s\u0301",
    \u1E65: "s\u0301\u0307",
    \u0161: "s\u030C",
    \u1E67: "s\u030C\u0307",
    \u015D: "s\u0302",
    \u1E61: "s\u0307",
    \u015F: "s\u0327",
    \u1E97: "t\u0308",
    \u0165: "t\u030C",
    \u1E6B: "t\u0307",
    \u0163: "t\u0327",
    \u00FA: "u\u0301",
    \u00F9: "u\u0300",
    \u00FC: "u\u0308",
    \u01D8: "u\u0308\u0301",
    \u01DC: "u\u0308\u0300",
    \u01D6: "u\u0308\u0304",
    \u01DA: "u\u0308\u030C",
    \u0169: "u\u0303",
    \u1E79: "u\u0303\u0301",
    \u016B: "u\u0304",
    \u1E7B: "u\u0304\u0308",
    \u016D: "u\u0306",
    \u01D4: "u\u030C",
    \u00FB: "u\u0302",
    \u016F: "u\u030A",
    \u0171: "u\u030B",
    \u1E7D: "v\u0303",
    \u1E83: "w\u0301",
    \u1E81: "w\u0300",
    \u1E85: "w\u0308",
    \u0175: "w\u0302",
    \u1E87: "w\u0307",
    \u1E98: "w\u030A",
    \u1E8D: "x\u0308",
    \u1E8B: "x\u0307",
    \u00FD: "y\u0301",
    \u1EF3: "y\u0300",
    \u00FF: "y\u0308",
    \u1EF9: "y\u0303",
    \u0233: "y\u0304",
    \u0177: "y\u0302",
    \u1E8F: "y\u0307",
    \u1E99: "y\u030A",
    \u017A: "z\u0301",
    \u017E: "z\u030C",
    \u1E91: "z\u0302",
    \u017C: "z\u0307",
    \u00C1: "A\u0301",
    \u00C0: "A\u0300",
    \u00C4: "A\u0308",
    \u01DE: "A\u0308\u0304",
    \u00C3: "A\u0303",
    \u0100: "A\u0304",
    \u0102: "A\u0306",
    \u1EAE: "A\u0306\u0301",
    \u1EB0: "A\u0306\u0300",
    \u1EB4: "A\u0306\u0303",
    \u01CD: "A\u030C",
    \u00C2: "A\u0302",
    \u1EA4: "A\u0302\u0301",
    \u1EA6: "A\u0302\u0300",
    \u1EAA: "A\u0302\u0303",
    \u0226: "A\u0307",
    \u01E0: "A\u0307\u0304",
    \u00C5: "A\u030A",
    \u01FA: "A\u030A\u0301",
    \u1E02: "B\u0307",
    \u0106: "C\u0301",
    \u1E08: "C\u0327\u0301",
    \u010C: "C\u030C",
    \u0108: "C\u0302",
    \u010A: "C\u0307",
    \u00C7: "C\u0327",
    \u010E: "D\u030C",
    \u1E0A: "D\u0307",
    \u1E10: "D\u0327",
    \u00C9: "E\u0301",
    \u00C8: "E\u0300",
    \u00CB: "E\u0308",
    \u1EBC: "E\u0303",
    \u0112: "E\u0304",
    \u1E16: "E\u0304\u0301",
    \u1E14: "E\u0304\u0300",
    \u0114: "E\u0306",
    \u1E1C: "E\u0327\u0306",
    \u011A: "E\u030C",
    \u00CA: "E\u0302",
    \u1EBE: "E\u0302\u0301",
    \u1EC0: "E\u0302\u0300",
    \u1EC4: "E\u0302\u0303",
    \u0116: "E\u0307",
    \u0228: "E\u0327",
    \u1E1E: "F\u0307",
    \u01F4: "G\u0301",
    \u1E20: "G\u0304",
    \u011E: "G\u0306",
    \u01E6: "G\u030C",
    \u011C: "G\u0302",
    \u0120: "G\u0307",
    \u0122: "G\u0327",
    \u1E26: "H\u0308",
    \u021E: "H\u030C",
    \u0124: "H\u0302",
    \u1E22: "H\u0307",
    \u1E28: "H\u0327",
    \u00CD: "I\u0301",
    \u00CC: "I\u0300",
    \u00CF: "I\u0308",
    \u1E2E: "I\u0308\u0301",
    \u0128: "I\u0303",
    \u012A: "I\u0304",
    \u012C: "I\u0306",
    \u01CF: "I\u030C",
    \u00CE: "I\u0302",
    \u0130: "I\u0307",
    \u0134: "J\u0302",
    \u1E30: "K\u0301",
    \u01E8: "K\u030C",
    \u0136: "K\u0327",
    \u0139: "L\u0301",
    \u013D: "L\u030C",
    \u013B: "L\u0327",
    \u1E3E: "M\u0301",
    \u1E40: "M\u0307",
    \u0143: "N\u0301",
    \u01F8: "N\u0300",
    \u00D1: "N\u0303",
    \u0147: "N\u030C",
    \u1E44: "N\u0307",
    \u0145: "N\u0327",
    \u00D3: "O\u0301",
    \u00D2: "O\u0300",
    \u00D6: "O\u0308",
    \u022A: "O\u0308\u0304",
    \u00D5: "O\u0303",
    \u1E4C: "O\u0303\u0301",
    \u1E4E: "O\u0303\u0308",
    \u022C: "O\u0303\u0304",
    \u014C: "O\u0304",
    \u1E52: "O\u0304\u0301",
    \u1E50: "O\u0304\u0300",
    \u014E: "O\u0306",
    \u01D1: "O\u030C",
    \u00D4: "O\u0302",
    \u1ED0: "O\u0302\u0301",
    \u1ED2: "O\u0302\u0300",
    \u1ED6: "O\u0302\u0303",
    \u022E: "O\u0307",
    \u0230: "O\u0307\u0304",
    \u0150: "O\u030B",
    \u1E54: "P\u0301",
    \u1E56: "P\u0307",
    \u0154: "R\u0301",
    \u0158: "R\u030C",
    \u1E58: "R\u0307",
    \u0156: "R\u0327",
    \u015A: "S\u0301",
    \u1E64: "S\u0301\u0307",
    \u0160: "S\u030C",
    \u1E66: "S\u030C\u0307",
    \u015C: "S\u0302",
    \u1E60: "S\u0307",
    \u015E: "S\u0327",
    \u0164: "T\u030C",
    \u1E6A: "T\u0307",
    \u0162: "T\u0327",
    \u00DA: "U\u0301",
    \u00D9: "U\u0300",
    \u00DC: "U\u0308",
    \u01D7: "U\u0308\u0301",
    \u01DB: "U\u0308\u0300",
    \u01D5: "U\u0308\u0304",
    \u01D9: "U\u0308\u030C",
    \u0168: "U\u0303",
    \u1E78: "U\u0303\u0301",
    \u016A: "U\u0304",
    \u1E7A: "U\u0304\u0308",
    \u016C: "U\u0306",
    \u01D3: "U\u030C",
    \u00DB: "U\u0302",
    \u016E: "U\u030A",
    \u0170: "U\u030B",
    \u1E7C: "V\u0303",
    \u1E82: "W\u0301",
    \u1E80: "W\u0300",
    \u1E84: "W\u0308",
    \u0174: "W\u0302",
    \u1E86: "W\u0307",
    \u1E8C: "X\u0308",
    \u1E8A: "X\u0307",
    \u00DD: "Y\u0301",
    \u1EF2: "Y\u0300",
    \u0178: "Y\u0308",
    \u1EF8: "Y\u0303",
    \u0232: "Y\u0304",
    \u0176: "Y\u0302",
    \u1E8E: "Y\u0307",
    \u0179: "Z\u0301",
    \u017D: "Z\u030C",
    \u1E90: "Z\u0302",
    \u017B: "Z\u0307",
    \u03AC: "\u03B1\u0301",
    \u1F70: "\u03B1\u0300",
    \u1FB1: "\u03B1\u0304",
    \u1FB0: "\u03B1\u0306",
    \u03AD: "\u03B5\u0301",
    \u1F72: "\u03B5\u0300",
    \u03AE: "\u03B7\u0301",
    \u1F74: "\u03B7\u0300",
    \u03AF: "\u03B9\u0301",
    \u1F76: "\u03B9\u0300",
    \u03CA: "\u03B9\u0308",
    \u0390: "\u03B9\u0308\u0301",
    \u1FD2: "\u03B9\u0308\u0300",
    \u1FD1: "\u03B9\u0304",
    \u1FD0: "\u03B9\u0306",
    \u03CC: "\u03BF\u0301",
    \u1F78: "\u03BF\u0300",
    \u03CD: "\u03C5\u0301",
    \u1F7A: "\u03C5\u0300",
    \u03CB: "\u03C5\u0308",
    \u03B0: "\u03C5\u0308\u0301",
    \u1FE2: "\u03C5\u0308\u0300",
    \u1FE1: "\u03C5\u0304",
    \u1FE0: "\u03C5\u0306",
    \u03CE: "\u03C9\u0301",
    \u1F7C: "\u03C9\u0300",
    \u038E: "\u03A5\u0301",
    \u1FEA: "\u03A5\u0300",
    \u03AB: "\u03A5\u0308",
    \u1FE9: "\u03A5\u0304",
    \u1FE8: "\u03A5\u0306",
    \u038F: "\u03A9\u0301",
    \u1FFA: "\u03A9\u0300"
  };
  class lt {
    constructor(e, t) {
      this.mode = void 0, this.gullet = void 0, this.settings = void 0, this.leftrightDepth = void 0, this.nextToken = void 0, this.mode = "math", this.gullet = new k4(e, t, this.mode), this.settings = t, this.leftrightDepth = 0;
    }
    expect(e, t) {
      if (t === void 0 && (t = true), this.fetch().text !== e) throw new T("Expected '" + e + "', got '" + this.fetch().text + "'", this.fetch());
      t && this.consume();
    }
    consume() {
      this.nextToken = null;
    }
    fetch() {
      return this.nextToken == null && (this.nextToken = this.gullet.expandNextToken()), this.nextToken;
    }
    switchMode(e) {
      this.mode = e, this.gullet.switchMode(e);
    }
    parse() {
      this.settings.globalGroup || this.gullet.beginGroup(), this.settings.colorIsTextColor && this.gullet.macros.set("\\color", "\\textcolor");
      try {
        var e = this.parseExpression(false);
        return this.expect("EOF"), this.settings.globalGroup || this.gullet.endGroup(), e;
      } finally {
        this.gullet.endGroups();
      }
    }
    subparse(e) {
      var t = this.nextToken;
      this.consume(), this.gullet.pushToken(new d0("}")), this.gullet.pushTokens(e);
      var a = this.parseExpression(false);
      return this.expect("}"), this.nextToken = t, a;
    }
    parseExpression(e, t) {
      for (var a = []; ; ) {
        this.mode === "math" && this.consumeSpaces();
        var n = this.fetch();
        if (lt.endOfExpression.indexOf(n.text) !== -1 || t && n.text === t || e && j0[n.text] && j0[n.text].infix) break;
        var i = this.parseAtom(t);
        if (i) {
          if (i.type === "internal") continue;
        } else break;
        a.push(i);
      }
      return this.mode === "text" && this.formLigatures(a), this.handleInfixNodes(a);
    }
    handleInfixNodes(e) {
      for (var t = -1, a, n = 0; n < e.length; n++) if (e[n].type === "infix") {
        if (t !== -1) throw new T("only one infix operator per group", e[n].token);
        t = n, a = e[n].replaceWith;
      }
      if (t !== -1 && a) {
        var i, o, u = e.slice(0, t), m = e.slice(t + 1);
        u.length === 1 && u[0].type === "ordgroup" ? i = u[0] : i = {
          type: "ordgroup",
          mode: this.mode,
          body: u
        }, m.length === 1 && m[0].type === "ordgroup" ? o = m[0] : o = {
          type: "ordgroup",
          mode: this.mode,
          body: m
        };
        var d;
        return a === "\\\\abovefrac" ? d = this.callFunction(a, [
          i,
          e[t],
          o
        ], []) : d = this.callFunction(a, [
          i,
          o
        ], []), [
          d
        ];
      } else return e;
    }
    handleSupSubscript(e) {
      var t = this.fetch(), a = t.text;
      this.consume(), this.consumeSpaces();
      var n;
      do {
        var i;
        n = this.parseGroup(e);
      } while (((i = n) == null ? void 0 : i.type) === "internal");
      if (!n) throw new T("Expected group after '" + a + "'", t);
      return n;
    }
    formatUnsupportedCmd(e) {
      for (var t = [], a = 0; a < e.length; a++) t.push({
        type: "textord",
        mode: "text",
        text: e[a]
      });
      var n = {
        type: "text",
        mode: this.mode,
        body: t
      }, i = {
        type: "color",
        mode: this.mode,
        color: this.settings.errorColor,
        body: [
          n
        ]
      };
      return i;
    }
    parseAtom(e) {
      var t = this.parseGroup("atom", e);
      if ((t == null ? void 0 : t.type) === "internal" || this.mode === "text") return t;
      for (var a, n; ; ) {
        this.consumeSpaces();
        var i = this.fetch();
        if (i.text === "\\limits" || i.text === "\\nolimits") {
          if (t && t.type === "op") {
            var o = i.text === "\\limits";
            t.limits = o, t.alwaysHandleSupSub = true;
          } else if (t && t.type === "operatorname") t.alwaysHandleSupSub && (t.limits = i.text === "\\limits");
          else throw new T("Limit controls must follow a math operator", i);
          this.consume();
        } else if (i.text === "^") {
          if (a) throw new T("Double superscript", i);
          a = this.handleSupSubscript("superscript");
        } else if (i.text === "_") {
          if (n) throw new T("Double subscript", i);
          n = this.handleSupSubscript("subscript");
        } else if (i.text === "'") {
          if (a) throw new T("Double superscript", i);
          var u = {
            type: "textord",
            mode: this.mode,
            text: "\\prime"
          }, m = [
            u
          ];
          for (this.consume(); this.fetch().text === "'"; ) m.push(u), this.consume();
          this.fetch().text === "^" && m.push(this.handleSupSubscript("superscript")), a = {
            type: "ordgroup",
            mode: this.mode,
            body: m
          };
        } else if (Ge[i.text]) {
          var d = na.test(i.text), p = [];
          for (p.push(new d0(Ge[i.text])), this.consume(); ; ) {
            var b = this.fetch().text;
            if (!Ge[b] || na.test(b) !== d) break;
            p.unshift(new d0(Ge[b])), this.consume();
          }
          var y = this.subparse(p);
          d ? n = {
            type: "ordgroup",
            mode: "math",
            body: y
          } : a = {
            type: "ordgroup",
            mode: "math",
            body: y
          };
        } else break;
      }
      return a || n ? {
        type: "supsub",
        mode: this.mode,
        base: t,
        sup: a,
        sub: n
      } : t;
    }
    parseFunction(e, t) {
      var a = this.fetch(), n = a.text, i = j0[n];
      if (!i) return null;
      if (this.consume(), t && t !== "atom" && !i.allowedInArgument) throw new T("Got function '" + n + "' with no arguments" + (t ? " as " + t : ""), a);
      if (this.mode === "text" && !i.allowedInText) throw new T("Can't use function '" + n + "' in text mode", a);
      if (this.mode === "math" && i.allowedInMath === false) throw new T("Can't use function '" + n + "' in math mode", a);
      var { args: o, optArgs: u } = this.parseArguments(n, i);
      return this.callFunction(n, o, u, a, e);
    }
    callFunction(e, t, a, n, i) {
      var o = {
        funcName: e,
        parser: this,
        token: n,
        breakOnTokenText: i
      }, u = j0[e];
      if (u && u.handler) return u.handler(o, t, a);
      throw new T("No function handler for " + e);
    }
    parseArguments(e, t) {
      var a = t.numArgs + t.numOptionalArgs;
      if (a === 0) return {
        args: [],
        optArgs: []
      };
      for (var n = [], i = [], o = 0; o < a; o++) {
        var u = t.argTypes && t.argTypes[o], m = o < t.numOptionalArgs;
        (t.primitive && u == null || t.type === "sqrt" && o === 1 && i[0] == null) && (u = "primitive");
        var d = this.parseGroupOfType("argument to '" + e + "'", u, m);
        if (m) i.push(d);
        else if (d != null) n.push(d);
        else throw new T("Null argument, please report this as a bug");
      }
      return {
        args: n,
        optArgs: i
      };
    }
    parseGroupOfType(e, t, a) {
      switch (t) {
        case "color":
          return this.parseColorGroup(a);
        case "size":
          return this.parseSizeGroup(a);
        case "url":
          return this.parseUrlGroup(a);
        case "math":
        case "text":
          return this.parseArgumentGroup(a, t);
        case "hbox": {
          var n = this.parseArgumentGroup(a, "text");
          return n != null ? {
            type: "styling",
            mode: n.mode,
            body: [
              n
            ],
            style: "text"
          } : null;
        }
        case "raw": {
          var i = this.parseStringGroup("raw", a);
          return i != null ? {
            type: "raw",
            mode: "text",
            string: i.text
          } : null;
        }
        case "primitive": {
          if (a) throw new T("A primitive argument cannot be optional");
          var o = this.parseGroup(e);
          if (o == null) throw new T("Expected group as " + e, this.fetch());
          return o;
        }
        case "original":
        case null:
        case void 0:
          return this.parseArgumentGroup(a);
        default:
          throw new T("Unknown group type as " + e, this.fetch());
      }
    }
    consumeSpaces() {
      for (; this.fetch().text === " "; ) this.consume();
    }
    parseStringGroup(e, t) {
      var a = this.gullet.scanArgument(t);
      if (a == null) return null;
      for (var n = "", i; (i = this.fetch()).text !== "EOF"; ) n += i.text, this.consume();
      return this.consume(), a.text = n, a;
    }
    parseRegexGroup(e, t) {
      for (var a = this.fetch(), n = a, i = "", o; (o = this.fetch()).text !== "EOF" && e.test(i + o.text); ) n = o, i += n.text, this.consume();
      if (i === "") throw new T("Invalid " + t + ": '" + a.text + "'", a);
      return a.range(n, i);
    }
    parseColorGroup(e) {
      var t = this.parseStringGroup("color", e);
      if (t == null) return null;
      var a = /^(#[a-f0-9]{3,4}|#[a-f0-9]{6}|#[a-f0-9]{8}|[a-f0-9]{6}|[a-z]+)$/i.exec(t.text);
      if (!a) throw new T("Invalid color: '" + t.text + "'", t);
      var n = a[0];
      return /^[0-9a-f]{6}$/i.test(n) && (n = "#" + n), {
        type: "color-token",
        mode: this.mode,
        color: n
      };
    }
    parseSizeGroup(e) {
      var t, a = false;
      if (this.gullet.consumeSpaces(), !e && this.gullet.future().text !== "{" ? t = this.parseRegexGroup(/^[-+]? *(?:$|\d+|\d+\.\d*|\.\d*) *[a-z]{0,2} *$/, "size") : t = this.parseStringGroup("size", e), !t) return null;
      !e && t.text.length === 0 && (t.text = "0pt", a = true);
      var n = /([-+]?) *(\d+(?:\.\d*)?|\.\d+) *([a-z]{2})/.exec(t.text);
      if (!n) throw new T("Invalid size: '" + t.text + "'", t);
      var i = {
        number: +(n[1] + n[2]),
        unit: n[3]
      };
      if (!Sa(i)) throw new T("Invalid unit: '" + i.unit + "'", t);
      return {
        type: "size",
        mode: this.mode,
        value: i,
        isBlank: a
      };
    }
    parseUrlGroup(e) {
      this.gullet.lexer.setCatcode("%", 13), this.gullet.lexer.setCatcode("~", 12);
      var t = this.parseStringGroup("url", e);
      if (this.gullet.lexer.setCatcode("%", 14), this.gullet.lexer.setCatcode("~", 13), t == null) return null;
      var a = t.text.replace(/\\([#$%&~_^{}])/g, "$1");
      return {
        type: "url",
        mode: this.mode,
        url: a
      };
    }
    parseArgumentGroup(e, t) {
      var a = this.gullet.scanArgument(e);
      if (a == null) return null;
      var n = this.mode;
      t && this.switchMode(t), this.gullet.beginGroup();
      var i = this.parseExpression(false, "EOF");
      this.expect("EOF"), this.gullet.endGroup();
      var o = {
        type: "ordgroup",
        mode: this.mode,
        loc: a.loc,
        body: i
      };
      return t && this.switchMode(n), o;
    }
    parseGroup(e, t) {
      var a = this.fetch(), n = a.text, i;
      if (n === "{" || n === "\\begingroup") {
        this.consume();
        var o = n === "{" ? "}" : "\\endgroup";
        this.gullet.beginGroup();
        var u = this.parseExpression(false, o), m = this.fetch();
        this.expect(o), this.gullet.endGroup(), i = {
          type: "ordgroup",
          mode: this.mode,
          loc: h0.range(a, m),
          body: u,
          semisimple: n === "\\begingroup" || void 0
        };
      } else if (i = this.parseFunction(t, e) || this.parseSymbol(), i == null && n[0] === "\\" && !d1.hasOwnProperty(n)) {
        if (this.settings.throwOnError) throw new T("Undefined control sequence: " + n, a);
        i = this.formatUnsupportedCmd(n), this.consume();
      }
      return i;
    }
    formLigatures(e) {
      for (var t = e.length - 1, a = 0; a < t; ++a) {
        var n = e[a], i = n.text;
        i === "-" && e[a + 1].text === "-" && (a + 1 < t && e[a + 2].text === "-" ? (e.splice(a, 3, {
          type: "textord",
          mode: "text",
          loc: h0.range(n, e[a + 2]),
          text: "---"
        }), t -= 2) : (e.splice(a, 2, {
          type: "textord",
          mode: "text",
          loc: h0.range(n, e[a + 1]),
          text: "--"
        }), t -= 1)), (i === "'" || i === "`") && e[a + 1].text === i && (e.splice(a, 2, {
          type: "textord",
          mode: "text",
          loc: h0.range(n, e[a + 1]),
          text: i + i
        }), t -= 1);
      }
    }
    parseSymbol() {
      var e = this.fetch(), t = e.text;
      if (/^\\verb[^a-zA-Z]/.test(t)) {
        this.consume();
        var a = t.slice(5), n = a.charAt(0) === "*";
        if (n && (a = a.slice(1)), a.length < 2 || a.charAt(0) !== a.slice(-1)) throw new T(`\\verb assertion failed --
                    please report what input caused this bug`);
        return a = a.slice(1, -1), {
          type: "verb",
          mode: "text",
          body: a,
          star: n
        };
      }
      ia.hasOwnProperty(t[0]) && !W[this.mode][t[0]] && (this.settings.strict && this.mode === "math" && this.settings.reportNonstrict("unicodeTextInMathMode", 'Accented Unicode text character "' + t[0] + '" used in math mode', e), t = ia[t[0]] + t.slice(1));
      var i = b4.exec(t);
      i && (t = t.substring(0, i.index), t === "i" ? t = "\u0131" : t === "j" && (t = "\u0237"));
      var o;
      if (W[this.mode][t]) {
        this.settings.strict && this.mode === "math" && Vt.indexOf(t) >= 0 && this.settings.reportNonstrict("unicodeTextInMathMode", 'Latin-1/Unicode text character "' + t[0] + '" used in math mode', e);
        var u = W[this.mode][t].group, m = h0.range(e), d;
        if (hi.hasOwnProperty(u)) {
          var p = u;
          d = {
            type: "atom",
            mode: this.mode,
            family: p,
            loc: m,
            text: t
          };
        } else d = {
          type: u,
          mode: this.mode,
          loc: m,
          text: t
        };
        o = d;
      } else if (t.charCodeAt(0) >= 128) this.settings.strict && (ka(t.charCodeAt(0)) ? this.mode === "math" && this.settings.reportNonstrict("unicodeTextInMathMode", 'Unicode text character "' + t[0] + '" used in math mode', e) : this.settings.reportNonstrict("unknownSymbol", 'Unrecognized Unicode character "' + t[0] + '"' + (" (" + t.charCodeAt(0) + ")"), e)), o = {
        type: "textord",
        mode: "text",
        loc: h0.range(e),
        text: t
      };
      else return null;
      if (this.consume(), i) for (var b = 0; b < i[0].length; b++) {
        var y = i[0][b];
        if (!Nt[y]) throw new T("Unknown accent ' " + y + "'", e);
        var x = Nt[y][this.mode] || Nt[y].text;
        if (!x) throw new T("Accent " + y + " unsupported in " + this.mode + " mode", e);
        o = {
          type: "accent",
          mode: this.mode,
          loc: h0.range(e),
          label: x,
          isStretchy: false,
          isShifty: true,
          base: o
        };
      }
      return o;
    }
  }
  lt.endOfExpression = [
    "}",
    "\\endgroup",
    "\\end",
    "\\right",
    "&"
  ];
  var dr = function(e, t) {
    if (!(typeof e == "string" || e instanceof String)) throw new TypeError("KaTeX can only parse string typed expression");
    var a = new lt(e, t);
    delete a.gullet.macros.current["\\df@tag"];
    var n = a.parse();
    if (delete a.gullet.macros.current["\\current@color"], delete a.gullet.macros.current["\\color"], a.gullet.macros.get("\\df@tag")) {
      if (!t.displayMode) throw new T("\\tag works only in display equations");
      n = [
        {
          type: "tag",
          mode: "text",
          body: n,
          tag: a.subparse([
            new d0("\\df@tag")
          ])
        }
      ];
    }
    return n;
  }, f1 = function(e, t, a) {
    t.textContent = "";
    var n = fr(e, a).toNode();
    t.appendChild(n);
  };
  typeof document < "u" && document.compatMode !== "CSS1Compat" && (typeof console < "u" && console.warn("Warning: KaTeX doesn't work in quirks mode. Make sure your website has a suitable doctype."), f1 = function() {
    throw new T("KaTeX doesn't work in quirks mode.");
  });
  var S4 = function(e, t) {
    var a = fr(e, t).toMarkup();
    return a;
  }, M4 = function(e, t) {
    var a = new jt(t);
    return dr(e, a);
  }, v1 = function(e, t, a) {
    if (a.throwOnError || !(e instanceof T)) throw e;
    var n = w.makeSpan([
      "katex-error"
    ], [
      new b0(t)
    ]);
    return n.setAttribute("title", e.toString()), n.setAttribute("style", "color:" + a.errorColor), n;
  }, fr = function(e, t) {
    var a = new jt(t);
    try {
      var n = dr(e, a);
      return Ei(n, e, a);
    } catch (i) {
      return v1(i, e, a);
    }
  }, z4 = function(e, t) {
    var a = new jt(t);
    try {
      var n = dr(e, a);
      return Ri(n, e, a);
    } catch (i) {
      return v1(i, e, a);
    }
  }, A4 = "0.16.28", T4 = {
    Span: Be,
    Anchor: Jt,
    SymbolNode: b0,
    SvgNode: H0,
    PathNode: _0,
    LineNode: Pt
  }, sa = {
    version: A4,
    render: f1,
    renderToString: S4,
    ParseError: T,
    SETTINGS_SCHEMA: Ye,
    __parse: M4,
    __renderToDomTree: fr,
    __renderToHTMLTree: z4,
    __setFontMetrics: ri,
    __defineSymbol: s,
    __defineFunction: D,
    __defineMacro: c,
    __domTree: T4
  }, B4 = C0('<div class="math-container flex w-full" style="font-size: 0.9rem;"><!></div>');
  function la(r, e) {
    Qe(e, false);
    let t = ga(e, "expr", 8, "");
    Wt();
    var a = B4(), n = i0(a);
    fn(n, () => (ce(sa), ce(t()), A0(() => sa.renderToString(t(), {
      throwOnError: false,
      displayMode: true
    })))), a0(a), z0(r, a), et();
  }
  const C4 = (r) => {
    var e = D4();
    z0(r, e);
  };
  var D4 = C0('<div class="absolute top-4 right-4 z-10 opacity-0 transition-opacity duration-200 group-hover:opacity-100"><ul class="menu menu-horizontal bg-base-200/50 rounded-box gap-1 p-0"><li><button class="hover:bg-base-300 flex items-center justify-center p-2 transition-colors" title="Copy"><i class="fa-solid fa-copy text-sm"></i></button></li> <li><button class="hover:bg-error/20 hover:text-error flex items-center justify-center p-2 transition-colors" title="Delete"><i class="fa-solid fa-trash text-sm"></i></button></li></ul></div>'), N4 = C0('<div class="bg-base-200 rounded-t-sm pt-2 pl-8 overflow-x-auto"><!></div>'), q4 = C0('<div class="rounded-t-sm bg-red-200 py-4 pl-8 overflow-x-auto"><p> </p></div>'), E4 = C0('<div class="border-base-200 bg-white rounded-b-sm border pl-8 overflow-x-auto"><!></div>'), R4 = C0('<div class=" rounded-b-sm border bg-white border-red-200 p-2 pl-8 overflow-x-auto"><b class="mr-2">Error:</b> </div>'), I4 = C0('<div class="group w-full rounded-sm shadow-md"><div><!> <!></div> <!></div>');
  function O4(r, e) {
    Qe(e, false);
    let t = ga(e, "entry", 8, void 0);
    const a = () => "base-200";
    Wt();
    var n = I4(), i = i0(n), o = i0(i);
    C4(o);
    var u = de(o, 2);
    {
      var m = (x) => {
        var k = N4(), S = i0(k);
        la(S, {
          get expr() {
            return ce(t()), A0(() => t().evalResult.input);
          }
        }), a0(k), z0(x, k);
      }, d = (x) => {
        var k = q4(), S = i0(k), z = i0(S, true);
        a0(S), a0(k), Se(() => kr(z, (ce(t()), A0(() => t().parseError.input)))), z0(x, k);
      };
      It(u, (x) => {
        t() && "evalResult" in t() ? x(m) : t() && "parseError" in t() && x(d, 1);
      });
    }
    a0(i);
    var p = de(i, 2);
    {
      var b = (x) => {
        var k = E4(), S = i0(k);
        {
          let z = ua(() => (ce(t()), A0(() => "=" + t().evalResult.output)));
          la(S, {
            get expr() {
              return p0(z);
            }
          });
        }
        a0(k), z0(x, k);
      }, y = (x) => {
        var k = R4(), S = de(i0(k), 1, true);
        a0(k), Se(() => kr(S, (ce(t()), A0(() => t().parseError.msg)))), z0(x, k);
      };
      It(p, (x) => {
        t() && "evalResult" in t() ? x(b) : t() && "parseError" in t() && x(y, 1);
      });
    }
    a0(n), Se((x) => xn(i, 1, `bg-${x ?? ""} relative rounded-t-sm`), [
      () => A0(a)
    ]), xa(1, n, () => Tn, () => ({
      y: 20,
      duration: 400,
      easing: Bn
    })), z0(r, n), et();
  }
  const F4 = "" + new URL("../assets/tortoise_beatle.DpGwMlf7.svg", import.meta.url).href;
  var H4 = C0('<div class="scale-200 opacity-80"><img alt="App Logo"/></div>'), L4 = C0('<div class="content-area absolute inset-0 -z-10 flex items-center justify-center"><!></div> <div class="flex h-screen flex-col overflow-hidden"><div class="grow overflow-y-auto p-8"><div class="flex flex-col gap-4"></div></div> <div class="sticky bottom-0 p-8 pt-4"><!></div></div>', 1);
  X4 = function(r, e) {
    Qe(e, false);
    const t = () => rn(ba, "$appState", a), [a, n] = tn();
    let i = ha();
    async function o() {
      await pa(), p0(i).scrollTo({
        top: p0(i).scrollHeight,
        behavior: "smooth"
      });
    }
    _1(() => (t(), p0(i)), () => {
      var _a2;
      ((_a2 = t().data) == null ? void 0 : _a2.history) && p0(i) && o();
    }), J1(), Wt();
    var u = L4(), m = Q1(u), d = i0(m);
    {
      var p = (z) => {
        var B = H4(), N = i0(B);
        a0(B), Se(() => on(N, "src", F4)), xa(2, B, () => An, () => ({
          duration: 100
        })), z0(z, B);
      };
      It(d, (z) => {
        t(), A0(() => {
          var _a2;
          return ((_a2 = t().data) == null ? void 0 : _a2.history.length) == 0;
        }) && z(p);
      });
    }
    a0(m);
    var b = de(m, 2), y = i0(b), x = i0(y);
    mn(x, 5, () => (t(), A0(() => {
      var _a2;
      return (_a2 = t().data) == null ? void 0 : _a2.history;
    })), un, (z, B) => {
      O4(z, {
        get entry() {
          return p0(B);
        }
      });
    }), a0(x), a0(y), en(y, (z) => Rt(i, z), () => p0(i));
    var k = de(y, 2), S = i0(k);
    Sn(S, {}), a0(k), a0(b), z0(r, u), et(), n();
  };
});
export {
  __tla,
  X4 as component
};
