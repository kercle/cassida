import { h, aQ as A, aR as v, N as S, aS as T, aT as M, aU as x, aV as E, aW as L } from "./YVIW7mlL.js";
let Z, Q, m;
let __tla = (async () => {
  const U = /* @__PURE__ */ Symbol("is custom element"), W = /* @__PURE__ */ Symbol("is html"), I = v ? "link" : "LINK";
  Q = function(e) {
    if (h) {
      var t = false, n = () => {
        if (!t) {
          if (t = true, e.hasAttribute("value")) {
            var s = e.value;
            m(e, "value", null), e.value = s;
          }
          if (e.hasAttribute("checked")) {
            var r = e.checked;
            m(e, "checked", null), e.checked = r;
          }
        }
      };
      e.__on_r = n, S(n), T();
    }
  };
  m = function(e, t, n, s) {
    var r = N(e);
    h && (r[t] = e.getAttribute(t), t === "src" || t === "srcset" || t === "href" && e.nodeName === I) || r[t] !== (r[t] = n) && (t === "loading" && (e[M] = n), n == null ? e.removeAttribute(t) : typeof n != "string" && k(e).includes(t) ? e[t] = n : e.setAttribute(t, n));
  };
  function N(e) {
    return e.__attributes ?? (e.__attributes = {
      [U]: e.nodeName.includes("-"),
      [W]: e.namespaceURI === A
    });
  }
  var y = /* @__PURE__ */ new Map();
  function k(e) {
    var t = e.getAttribute("is") || e.nodeName, n = y.get(t);
    if (n) return n;
    y.set(t, n = []);
    for (var s, r = e, a = Element.prototype; a !== r; ) {
      s = E(r);
      for (var o in s) s[o].set && n.push(o);
      r = x(r);
    }
    return n;
  }
  const C = "" + new URL("../assets/kernel_bg.CdbbP2Yi.wasm", import.meta.url).href, O = async (e = {}, t) => {
    let n;
    if (t.startsWith("data:")) {
      const s = t.replace(/^data:.*?base64,/, "");
      let r;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") r = Buffer.from(s, "base64");
      else if (typeof atob == "function") {
        const a = atob(s);
        r = new Uint8Array(a.length);
        for (let o = 0; o < a.length; o++) r[o] = a.charCodeAt(o);
      } else throw new Error("Cannot decode base64-encoded data URL");
      n = await WebAssembly.instantiate(r, e);
    } else {
      const s = await fetch(t), r = s.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && r.startsWith("application/wasm")) n = await WebAssembly.instantiateStreaming(s, e);
      else {
        const a = await s.arrayBuffer();
        n = await WebAssembly.instantiate(a, e);
      }
    }
    return n.instance.exports;
  };
  function B(e) {
    let t, n;
    try {
      const s = j(e, _.__wbindgen_malloc, _.__wbindgen_realloc), r = g, a = _.eval_input(s, r);
      return t = a[0], n = a[1], D(a[0], a[1]);
    } finally {
      _.__wbindgen_free(t, n, 1);
    }
  }
  function R() {
    const e = _.__wbindgen_externrefs, t = e.grow(4);
    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
  }
  function D(e, t) {
    return e = e >>> 0, G(e, t);
  }
  let l = null;
  function u() {
    return (l === null || l.byteLength === 0) && (l = new Uint8Array(_.memory.buffer)), l;
  }
  function j(e, t, n) {
    if (n === void 0) {
      const i = d.encode(e), c = t(i.length, 1) >>> 0;
      return u().subarray(c, c + i.length).set(i), g = i.length, c;
    }
    let s = e.length, r = t(s, 1) >>> 0;
    const a = u();
    let o = 0;
    for (; o < s; o++) {
      const i = e.charCodeAt(o);
      if (i > 127) break;
      a[r + o] = i;
    }
    if (o !== s) {
      o !== 0 && (e = e.slice(o)), r = n(r, s, s = o + e.length * 3, 1) >>> 0;
      const i = u().subarray(r + o, r + s), c = d.encodeInto(e, i);
      o += c.written, r = n(r, s, o, 1) >>> 0;
    }
    return g = o, r;
  }
  let b = new TextDecoder("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  b.decode();
  const F = 2146435072;
  let w = 0;
  function G(e, t) {
    return w += t, w >= F && (b = new TextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true
    }), b.decode(), w = t), b.decode(u().subarray(e, e + t));
  }
  const d = new TextEncoder();
  "encodeInto" in d || (d.encodeInto = function(e, t) {
    const n = d.encode(e);
    return t.set(n), {
      read: e.length,
      written: n.length
    };
  });
  let g = 0, _;
  function H(e) {
    _ = e;
  }
  URL = globalThis.URL;
  const f = await O({
    "./kernel_bg.js": {
      __wbindgen_init_externref_table: R
    }
  }, C), P = f.memory, Y = f.eval_input, K = f.__wbindgen_externrefs, V = f.__wbindgen_malloc, X = f.__wbindgen_realloc, $ = f.__wbindgen_free, p = f.__wbindgen_start, q = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbindgen_externrefs: K,
    __wbindgen_free: $,
    __wbindgen_malloc: V,
    __wbindgen_realloc: X,
    __wbindgen_start: p,
    eval_input: Y,
    memory: P
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  H(q);
  p();
  function z() {
    const { subscribe: e, set: t, update: n } = L({
      data: {
        history: []
      },
      connected: false
    });
    async function s() {
      return n((a) => ({
        ...a,
        connected: true
      })), {
        send: async (a) => {
          const o = await B(a), i = typeof o == "string" ? JSON.parse(o) : o;
          n((c) => (c.data.history.push(i), {
            ...c,
            connected: true
          }));
        }
      };
    }
    let r = {
      send: () => {
      }
    };
    return typeof window < "u" && s().then((a) => r = a), {
      subscribe: e,
      send: (a) => r.send(a)
    };
  }
  Z = z();
})();
export {
  __tla,
  Z as a,
  Q as r,
  m as s
};
