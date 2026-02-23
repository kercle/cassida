import { h as b, aQ as h, aR as p, N as y, aS as v, aT as S, aU as T, aV as A, aW as L } from "./BfYh-9ES.js";
let q, Y, l;
let __tla = (async () => {
  const k = /* @__PURE__ */ Symbol("is custom element"), x = /* @__PURE__ */ Symbol("is html"), M = p ? "link" : "LINK";
  Y = function(e) {
    if (b) {
      var t = false, n = () => {
        if (!t) {
          if (t = true, e.hasAttribute("value")) {
            var o = e.value;
            l(e, "value", null), e.value = o;
          }
          if (e.hasAttribute("checked")) {
            var s = e.checked;
            l(e, "checked", null), e.checked = s;
          }
        }
      };
      e.__on_r = n, y(n), v();
    }
  };
  l = function(e, t, n, o) {
    var s = E(e);
    b && (s[t] = e.getAttribute(t), t === "src" || t === "srcset" || t === "href" && e.nodeName === M) || s[t] !== (s[t] = n) && (t === "loading" && (e[S] = n), n == null ? e.removeAttribute(t) : typeof n != "string" && N(e).includes(t) ? e[t] = n : e.setAttribute(t, n));
  };
  function E(e) {
    return e.__attributes ?? (e.__attributes = {
      [k]: e.nodeName.includes("-"),
      [x]: e.namespaceURI === h
    });
  }
  var u = /* @__PURE__ */ new Map();
  function N(e) {
    var t = e.getAttribute("is") || e.nodeName, n = u.get(t);
    if (n) return n;
    u.set(t, n = []);
    for (var o, s = e, r = Element.prototype; r !== s; ) {
      o = A(s);
      for (var a in o) o[a].set && n.push(a);
      s = T(s);
    }
    return n;
  }
  const W = "" + new URL("../assets/kernel_bg.ByYg-8Lj.wasm", import.meta.url).href, I = async (e = {}, t) => {
    let n;
    if (t.startsWith("data:")) {
      const o = t.replace(/^data:.*?base64,/, "");
      let s;
      if (typeof Buffer == "function" && typeof Buffer.from == "function") s = Buffer.from(o, "base64");
      else if (typeof atob == "function") {
        const r = atob(o);
        s = new Uint8Array(r.length);
        for (let a = 0; a < r.length; a++) s[a] = r.charCodeAt(a);
      } else throw new Error("Cannot decode base64-encoded data URL");
      n = await WebAssembly.instantiate(s, e);
    } else {
      const o = await fetch(t), s = o.headers.get("Content-Type") || "";
      if ("instantiateStreaming" in WebAssembly && s.startsWith("application/wasm")) n = await WebAssembly.instantiateStreaming(o, e);
      else {
        const r = await o.arrayBuffer();
        n = await WebAssembly.instantiate(r, e);
      }
    }
    return n.instance.exports;
  };
  function U() {
    const e = g.__wbindgen_externrefs, t = e.grow(4);
    e.set(0, void 0), e.set(t + 0, void 0), e.set(t + 1, null), e.set(t + 2, true), e.set(t + 3, false);
  }
  let B = new TextDecoder("utf-8", {
    ignoreBOM: true,
    fatal: true
  });
  B.decode();
  const f = new TextEncoder();
  "encodeInto" in f || (f.encodeInto = function(e, t) {
    const n = f.encode(e);
    return t.set(n), {
      read: e.length,
      written: n.length
    };
  });
  let g;
  function O(e) {
    g = e;
  }
  URL = globalThis.URL;
  const i = await I({
    "./kernel_bg.js": {
      __wbindgen_init_externref_table: U
    }
  }, W), R = i.memory, C = i.eval_input, j = i.__wbindgen_externrefs, D = i.__wbindgen_malloc, G = i.__wbindgen_realloc, H = i.__wbindgen_free, w = i.__wbindgen_start, $ = Object.freeze(Object.defineProperty({
    __proto__: null,
    __wbindgen_externrefs: j,
    __wbindgen_free: H,
    __wbindgen_malloc: D,
    __wbindgen_realloc: G,
    __wbindgen_start: w,
    eval_input: C,
    memory: R
  }, Symbol.toStringTag, {
    value: "Module"
  }));
  O($);
  w();
  function K() {
    const { subscribe: e, set: t, update: n } = L({
      data: {
        history: []
      },
      connected: false
    });
    let o;
    function s() {
      const m = `${location.protocol === "https:" ? "wss" : "ws"}://${location.host}/ws`;
      return o = new WebSocket(m), o.onopen = () => n((c) => ({
        ...c,
        connected: true
      })), o.onmessage = (c) => {
        try {
          const _ = JSON.parse(c.data);
          n((d) => (d.data.history.push(_), {
            ...d,
            connected: true
          }));
        } catch (_) {
          console.log("Error parsing message:", _);
        }
      }, o.onclose = () => {
        n((c) => ({
          ...c,
          connected: false
        })), setTimeout(s, 3e3);
      }, {
        send: (c) => o == null ? void 0 : o.send(c)
      };
    }
    let r = {
      send: () => {
      }
    };
    return typeof window < "u" && (r = s()), {
      subscribe: e,
      send: (a) => r.send(a)
    };
  }
  q = K();
})();
export {
  __tla,
  q as a,
  Y as r,
  l as s
};
