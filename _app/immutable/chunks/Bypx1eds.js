import { h as p, aQ as A, aR as S, N as T, aS as M, aT as E, aU as L, aV as x, aW as U } from "./KQ4vooii.js";
let ee, Z, h;
let __tla = (async () => {
  const W = /* @__PURE__ */ Symbol("is custom element"), I = /* @__PURE__ */ Symbol("is html"), N = S ? "link" : "LINK";
  Z = function(e) {
    if (p) {
      var t = false, n = () => {
        if (!t) {
          if (t = true, e.hasAttribute("value")) {
            var o = e.value;
            h(e, "value", null), e.value = o;
          }
          if (e.hasAttribute("checked")) {
            var s = e.checked;
            h(e, "checked", null), e.checked = s;
          }
        }
      };
      e.__on_r = n, T(n), M();
    }
  };
  h = function(e, t, n, o) {
    var s = k(e);
    p && (s[t] = e.getAttribute(t), t === "src" || t === "srcset" || t === "href" && e.nodeName === N) || s[t] !== (s[t] = n) && (t === "loading" && (e[E] = n), n == null ? e.removeAttribute(t) : typeof n != "string" && C(e).includes(t) ? e[t] = n : e.setAttribute(t, n));
  };
  function k(e) {
    return e.__attributes ?? (e.__attributes = {
      [W]: e.nodeName.includes("-"),
      [I]: e.namespaceURI === A
    });
  }
  var m = /* @__PURE__ */ new Map();
  function C(e) {
    var t = e.getAttribute("is") || e.nodeName, n = m.get(t);
    if (n) return n;
    m.set(t, n = []);
    for (var o, s = e, a = Element.prototype; a !== s; ) {
      o = x(s);
      for (var r in o) o[r].set && n.push(r);
      s = L(s);
    }
    return n;
  }
  const O = "" + new URL("../assets/kernel_bg.CHhouLKg.wasm", import.meta.url).href, B = async (e = {}, t) => {
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
      const o = j(e, _.__wbindgen_malloc, _.__wbindgen_realloc), s = w, a = _.eval_input(o, s);
      return t = a[0], n = a[1], H(a[0], a[1]);
    } finally {
      _.__wbindgen_free(t, n, 1);
    }
  }
  function D() {
    const e = _.__wbindgen_externrefs, t = e.grow(4);
    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
  }
  function H(e, t) {
    return e = e >>> 0, G(e, t);
  }
  let l = null;
  function u() {
    return (l === null || l.byteLength === 0) && (l = new Uint8Array(_.memory.buffer)), l;
  }
  function j(e, t, n) {
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
  const F = 2146435072;
  let g = 0;
  function G(e, t) {
    return g += t, g >= F && (b = new TextDecoder("utf-8", {
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
  function K(e) {
    _ = e;
  }
  URL = globalThis.URL;
  const f = await B({
    "./kernel_bg.js": {
      __wbindgen_init_externref_table: D
    }
  }, O), P = f.memory, V = f.eval_input, X = f.__wbindgen_externrefs, Y = f.__wbindgen_malloc, $ = f.__wbindgen_realloc, q = f.__wbindgen_free, v = f.__wbindgen_start, z = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbindgen_externrefs: X,
    __wbindgen_free: q,
    __wbindgen_malloc: Y,
    __wbindgen_realloc: $,
    __wbindgen_start: v,
    eval_input: V,
    memory: P
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  K(z);
  v();
  function J() {
    const { subscribe: e, set: t, update: n } = U({
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
          n((y) => (o(y.data, c), {
            ...y,
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
  ee = J();
})();
export {
  __tla,
  ee as a,
  Z as r,
  h as s
};
