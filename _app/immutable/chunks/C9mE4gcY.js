import { h as p, aR as A, aS as S, N as T, aT as E, aU as M, aV as x, aW as L, aX as W } from "./BfX1JZpI.js";
let te, Q, ee, y;
let __tla = (async () => {
  const I = /* @__PURE__ */ Symbol("is custom element"), N = /* @__PURE__ */ Symbol("is html"), U = S ? "link" : "LINK";
  ee = function(e) {
    if (p) {
      var t = false, n = () => {
        if (!t) {
          if (t = true, e.hasAttribute("value")) {
            var o = e.value;
            y(e, "value", null), e.value = o;
          }
          if (e.hasAttribute("checked")) {
            var s = e.checked;
            y(e, "checked", null), e.checked = s;
          }
        }
      };
      e.__on_r = n, T(n), E();
    }
  };
  y = function(e, t, n, o) {
    var s = O(e);
    p && (s[t] = e.getAttribute(t), t === "src" || t === "srcset" || t === "href" && e.nodeName === U) || s[t] !== (s[t] = n) && (t === "loading" && (e[M] = n), n == null ? e.removeAttribute(t) : typeof n != "string" && k(e).includes(t) ? e[t] = n : e.setAttribute(t, n));
  };
  function O(e) {
    return e.__attributes ?? (e.__attributes = {
      [I]: e.nodeName.includes("-"),
      [N]: e.namespaceURI === A
    });
  }
  var h = /* @__PURE__ */ new Map();
  function k(e) {
    var t = e.getAttribute("is") || e.nodeName, n = h.get(t);
    if (n) return n;
    h.set(t, n = []);
    for (var o, s = e, a = Element.prototype; a !== s; ) {
      o = L(s);
      for (var r in o) o[r].set && n.push(r);
      s = x(s);
    }
    return n;
  }
  const B = "" + new URL("../assets/kernel_bg.DWIi-7ne.wasm", import.meta.url).href, C = async (e = {}, t) => {
    let n;
    if (t.startsWith("data:")) {
      const o = t.replace(/^data:.*?base64,/, "");
      let s;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") s = Buffer.from(o, "base64");
      else if (typeof atob == "function") {
        const a = atob(o);
        s = new Uint8Array(a.length);
        for (let r = 0; r < a.length; r++) s[r] = a.charCodeAt(r);
      } else throw new Error("Cannot decode base64-encoded data URL");
      n = await WebAssembly.instantiate(s, e);
    } else {
      const o = await fetch(t), s = o.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && s.startsWith("application/wasm")) n = await WebAssembly.instantiateStreaming(o, e);
      else {
        const a = await o.arrayBuffer();
        n = await WebAssembly.instantiate(a, e);
      }
    }
    return n.instance.exports;
  };
  function R(e) {
    let t, n;
    try {
      const o = F(e, _.__wbindgen_malloc, _.__wbindgen_realloc), s = w, a = _.eval_input(o, s);
      return t = a[0], n = a[1], j(a[0], a[1]);
    } finally {
      _.__wbindgen_free(t, n, 1);
    }
  }
  function D() {
    const e = _.__wbindgen_externrefs, t = e.grow(4);
    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
  }
  function j(e, t) {
    return e = e >>> 0, H(e, t);
  }
  let l = null;
  function u() {
    return (l === null || l.byteLength === 0) && (l = new Uint8Array(_.memory.buffer)), l;
  }
  function F(e, t, n) {
    if (n === void 0) {
      const i = d.encode(e), c = t(i.length, 1) >>> 0;
      return u().subarray(c, c + i.length).set(i), w = i.length, c;
    }
    let o = e.length, s = t(o, 1) >>> 0;
    const a = u();
    let r = 0;
    for (; r < o; r++) {
      const i = e.charCodeAt(r);
      if (i > 127) break;
      a[s + r] = i;
    }
    if (r !== o) {
      r !== 0 && (e = e.slice(r)), s = n(s, o, o = r + e.length * 3, 1) >>> 0;
      const i = u().subarray(s + r, s + o), c = d.encodeInto(e, i);
      r += c.written, s = n(s, o, r, 1) >>> 0;
    }
    return w = r, s;
  }
  let b = new TextDecoder("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  b.decode();
  const G = 2146435072;
  let g = 0;
  function H(e, t) {
    return g += t, g >= G && (b = new TextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true
    }), b.decode(), g = t), b.decode(u().subarray(e, e + t));
  }
  const d = new TextEncoder();
  "encodeInto" in d || (d.encodeInto = function(e, t) {
    const n = d.encode(e);
    return t.set(n), {
      read: e.length,
      written: n.length
    };
  });
  let w = 0, _;
  function X(e) {
    _ = e;
  }
  URL = globalThis.URL;
  const f = await C({
    "./kernel_bg.js": {
      __wbindgen_init_externref_table: D
    }
  }, B), J = f.memory, K = f.eval_input, P = f.__wbindgen_externrefs, V = f.__wbindgen_malloc, Y = f.__wbindgen_realloc, $ = f.__wbindgen_free, v = f.__wbindgen_start, q = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbindgen_externrefs: P,
    __wbindgen_free: $,
    __wbindgen_malloc: V,
    __wbindgen_realloc: Y,
    __wbindgen_start: v,
    eval_input: K,
    memory: J
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  X(q);
  v();
  function z() {
    const { subscribe: e, set: t, update: n } = W({
      data: {
        history: []
      },
      connected: false
    });
    function o(r, i) {
      let c = r.history.pop();
      c !== void 0 && !("parseError" in c) && r.history.push(c), r.history.push(i);
    }
    async function s() {
      return n((r) => ({
        ...r,
        connected: true
      })), {
        send: async (r) => {
          const i = await R(r), c = typeof i == "string" ? JSON.parse(i) : i;
          n((m) => (o(m.data, c), {
            ...m,
            connected: true
          }));
        }
      };
    }
    let a = {
      send: () => {
      }
    };
    return typeof window < "u" && s().then((r) => a = r), {
      subscribe: e,
      send: (r) => a.send(r)
    };
  }
  Q = z();
  te = function(e) {
    const t = {
      eval: e
    };
    Q.send(JSON.stringify(t));
  };
})();
export {
  __tla,
  te as a,
  Q as b,
  ee as r,
  y as s
};
