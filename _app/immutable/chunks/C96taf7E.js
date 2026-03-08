var __typeError = (msg) => {
  throw TypeError(msg);
};
var __accessCheck = (obj, member, msg) => member.has(obj) || __typeError("Cannot " + msg);
var __privateGet = (obj, member, getter) => (__accessCheck(obj, member, "read from private field"), getter ? getter.call(obj) : member.get(obj));
var __privateAdd = (obj, member, value) => member.has(obj) ? __typeError("Cannot add the same private member more than once") : member instanceof WeakSet ? member.add(obj) : member.set(obj, value);
var _a, _b, _e2, _t2, _n, _a2, _r, _o, _s, _i, _c, _e3, _d, _e4, _e5;
import { aW as be, aN as A, n as U, V as T, aM as ee, bg as dt } from "./YVIW7mlL.js";
import { o as je } from "./BgSKXl0c.js";
class ke {
  constructor(t, n) {
    this.status = t, typeof n == "string" ? this.body = { message: n } : n ? this.body = n : this.body = { message: `Error: ${t}` };
  }
  toString() {
    return JSON.stringify(this.body);
  }
}
class Se {
  constructor(t, n) {
    this.status = t, this.location = n;
  }
}
class Ee extends Error {
  constructor(t, n, a) {
    super(a), this.status = t, this.text = n;
  }
}
new URL("sveltekit-internal://");
function ht(e, t) {
  return e === "/" || t === "ignore" ? e : t === "never" ? e.endsWith("/") ? e.slice(0, -1) : e : t === "always" && !e.endsWith("/") ? e + "/" : e;
}
function pt(e) {
  return e.split("%25").map(decodeURI).join("%25");
}
function gt(e) {
  for (const t in e) e[t] = decodeURIComponent(e[t]);
  return e;
}
function de({ href: e }) {
  return e.split("#")[0];
}
function _t(...e) {
  let t = 5381;
  for (const n of e) if (typeof n == "string") {
    let a = n.length;
    for (; a; ) t = t * 33 ^ n.charCodeAt(--a);
  } else if (ArrayBuffer.isView(n)) {
    const a = new Uint8Array(n.buffer, n.byteOffset, n.byteLength);
    let r = a.length;
    for (; r; ) t = t * 33 ^ a[--r];
  } else throw new TypeError("value must be a string or TypedArray");
  return (t >>> 0).toString(36);
}
new TextEncoder();
new TextDecoder();
function mt(e) {
  const t = atob(e), n = new Uint8Array(t.length);
  for (let a = 0; a < t.length; a++) n[a] = t.charCodeAt(a);
  return n;
}
const wt = window.fetch;
window.fetch = (e, t) => ((e instanceof Request ? e.method : (t == null ? void 0 : t.method) || "GET") !== "GET" && W.delete(Re(e)), wt(e, t));
const W = /* @__PURE__ */ new Map();
function vt(e, t) {
  const n = Re(e, t), a = document.querySelector(n);
  if (a == null ? void 0 : a.textContent) {
    a.remove();
    let { body: r, ...i } = JSON.parse(a.textContent);
    const o = a.getAttribute("data-ttl");
    return o && W.set(n, { body: r, init: i, ttl: 1e3 * Number(o) }), a.getAttribute("data-b64") !== null && (r = mt(r)), Promise.resolve(new Response(r, i));
  }
  return window.fetch(e, t);
}
function yt(e, t, n) {
  if (W.size > 0) {
    const a = Re(e, n), r = W.get(a);
    if (r) {
      if (performance.now() < r.ttl && ["default", "force-cache", "only-if-cached", void 0].includes(n == null ? void 0 : n.cache)) return new Response(r.body, r.init);
      W.delete(a);
    }
  }
  return window.fetch(t, n);
}
function Re(e, t) {
  let a = `script[data-sveltekit-fetched][data-url=${JSON.stringify(e instanceof Request ? e.url : e)}]`;
  if ((t == null ? void 0 : t.headers) || (t == null ? void 0 : t.body)) {
    const r = [];
    t.headers && r.push([...new Headers(t.headers)].join(",")), t.body && (typeof t.body == "string" || ArrayBuffer.isView(t.body)) && r.push(t.body), a += `[data-hash="${_t(...r)}"]`;
  }
  return a;
}
const bt = /^(\[)?(\.\.\.)?(\w+)(?:=(\w+))?(\])?$/;
function kt(e) {
  const t = [];
  return { pattern: e === "/" ? /^\/$/ : new RegExp(`^${Et(e).map((a) => {
    const r = /^\[\.\.\.(\w+)(?:=(\w+))?\]$/.exec(a);
    if (r) return t.push({ name: r[1], matcher: r[2], optional: false, rest: true, chained: true }), "(?:/([^]*))?";
    const i = /^\[\[(\w+)(?:=(\w+))?\]\]$/.exec(a);
    if (i) return t.push({ name: i[1], matcher: i[2], optional: true, rest: false, chained: true }), "(?:/([^/]+))?";
    if (!a) return;
    const o = a.split(/\[(.+?)\](?!\])/);
    return "/" + o.map((c, l) => {
      if (l % 2) {
        if (c.startsWith("x+")) return he(String.fromCharCode(parseInt(c.slice(2), 16)));
        if (c.startsWith("u+")) return he(String.fromCharCode(...c.slice(2).split("-").map((m) => parseInt(m, 16))));
        const f = bt.exec(c), [, h, w, u, p] = f;
        return t.push({ name: u, matcher: p, optional: !!h, rest: !!w, chained: w ? l === 1 && o[0] === "" : false }), w ? "([^]*?)" : h ? "([^/]*)?" : "([^/]+?)";
      }
      return he(c);
    }).join("");
  }).join("")}/?$`), params: t };
}
function St(e) {
  return e !== "" && !/^\([^)]+\)$/.test(e);
}
function Et(e) {
  return e.slice(1).split("/").filter(St);
}
function Rt(e, t, n) {
  const a = {}, r = e.slice(1), i = r.filter((s) => s !== void 0);
  let o = 0;
  for (let s = 0; s < t.length; s += 1) {
    const c = t[s];
    let l = r[s - o];
    if (c.chained && c.rest && o && (l = r.slice(s - o, s + 1).filter((f) => f).join("/"), o = 0), l === void 0) if (c.rest) l = "";
    else continue;
    if (!c.matcher || n[c.matcher](l)) {
      a[c.name] = l;
      const f = t[s + 1], h = r[s + 1];
      f && !f.rest && f.optional && h && c.chained && (o = 0), !f && !h && Object.keys(a).length === i.length && (o = 0);
      continue;
    }
    if (c.optional && c.chained) {
      o++;
      continue;
    }
    return;
  }
  if (!o) return a;
}
function he(e) {
  return e.normalize().replace(/[[\]]/g, "\\$&").replace(/%/g, "%25").replace(/\//g, "%2[Ff]").replace(/\?/g, "%3[Ff]").replace(/#/g, "%23").replace(/[.*+?^${}()|\\]/g, "\\$&");
}
function xt({ nodes: e, server_loads: t, dictionary: n, matchers: a }) {
  const r = new Set(t);
  return Object.entries(n).map(([s, [c, l, f]]) => {
    const { pattern: h, params: w } = kt(s), u = { id: s, exec: (p) => {
      const m = h.exec(p);
      if (m) return Rt(m, w, a);
    }, errors: [1, ...f || []].map((p) => e[p]), layouts: [0, ...l || []].map(o), leaf: i(c) };
    return u.errors.length = u.layouts.length = Math.max(u.errors.length, u.layouts.length), u;
  });
  function i(s) {
    const c = s < 0;
    return c && (s = ~s), [c, e[s]];
  }
  function o(s) {
    return s === void 0 ? s : [r.has(s), e[s]];
  }
}
function We(e, t = JSON.parse) {
  try {
    return t(sessionStorage[e]);
  } catch {
  }
}
function Ne(e, t, n = JSON.stringify) {
  const a = n(t);
  try {
    sessionStorage[e] = a;
  } catch {
  }
}
const x = ((_a = globalThis.__sveltekit_hh7rax) == null ? void 0 : _a.base) ?? "/toy-cas", Lt = ((_b = globalThis.__sveltekit_hh7rax) == null ? void 0 : _b.assets) ?? x ?? "", At = "1773003419427", Fe = "sveltekit:snapshot", Ge = "sveltekit:scroll", Ye = "sveltekit:states", Ut = "sveltekit:pageurl", K = "sveltekit:history", G = "sveltekit:navigation", j = { tap: 1, hover: 2, viewport: 3, eager: 4, off: -1, false: -1 }, xe = location.origin;
function ze(e) {
  if (e instanceof URL) return e;
  let t = document.baseURI;
  if (!t) {
    const n = document.getElementsByTagName("base");
    t = n.length ? n[0].href : document.URL;
  }
  return new URL(e, t);
}
function D() {
  return { x: pageXOffset, y: pageYOffset };
}
function V(e, t) {
  return e.getAttribute(`data-sveltekit-${t}`);
}
const De = { ...j, "": j.hover };
function He(e) {
  let t = e.assignedSlot ?? e.parentNode;
  return (t == null ? void 0 : t.nodeType) === 11 && (t = t.host), t;
}
function Je(e, t) {
  for (; e && e !== t; ) {
    if (e.nodeName.toUpperCase() === "A" && e.hasAttribute("href")) return e;
    e = He(e);
  }
}
function _e(e, t, n) {
  let a;
  try {
    if (a = new URL(e instanceof SVGAElement ? e.href.baseVal : e.href, document.baseURI), n && a.hash.match(/^#[^/]/)) {
      const s = location.hash.split("#")[1] || "/";
      a.hash = `#${s}${a.hash}`;
    }
  } catch {
  }
  const r = e instanceof SVGAElement ? e.target.baseVal : e.target, i = !a || !!r || le(a, t, n) || (e.getAttribute("rel") || "").split(/\s+/).includes("external"), o = (a == null ? void 0 : a.origin) === xe && e.hasAttribute("download");
  return { url: a, external: i, target: r, download: o };
}
function te(e) {
  let t = null, n = null, a = null, r = null, i = null, o = null, s = e;
  for (; s && s !== document.documentElement; ) a === null && (a = V(s, "preload-code")), r === null && (r = V(s, "preload-data")), t === null && (t = V(s, "keepfocus")), n === null && (n = V(s, "noscroll")), i === null && (i = V(s, "reload")), o === null && (o = V(s, "replacestate")), s = He(s);
  function c(l) {
    switch (l) {
      case "":
      case "true":
        return true;
      case "off":
      case "false":
        return false;
      default:
        return;
    }
  }
  return { preload_code: De[a ?? "off"], preload_data: De[r ?? "off"], keepfocus: c(t), noscroll: c(n), reload: c(i), replace_state: c(o) };
}
function qe(e) {
  const t = be(e);
  let n = true;
  function a() {
    n = true, t.update((o) => o);
  }
  function r(o) {
    n = false, t.set(o);
  }
  function i(o) {
    let s;
    return t.subscribe((c) => {
      (s === void 0 || n && c !== s) && o(s = c);
    });
  }
  return { notify: a, set: r, subscribe: i };
}
const Xe = { v: () => {
} };
function Tt() {
  const { set: e, subscribe: t } = be(false);
  let n;
  async function a() {
    clearTimeout(n);
    try {
      const r = await fetch(`${Lt}/_app/version.json`, { headers: { pragma: "no-cache", "cache-control": "no-cache" } });
      if (!r.ok) return false;
      const o = (await r.json()).version !== At;
      return o && (e(true), Xe.v(), clearTimeout(n)), o;
    } catch {
      return false;
    }
  }
  return { subscribe: t, check: a };
}
function le(e, t, n) {
  return e.origin !== xe || !e.pathname.startsWith(t) ? true : n ? e.pathname !== location.pathname : false;
}
function rn(e) {
}
const Qe = /* @__PURE__ */ new Set(["load", "prerender", "csr", "ssr", "trailingSlash", "config"]);
[...Qe];
const It = /* @__PURE__ */ new Set([...Qe]);
[...It];
function Ot(e) {
  return e.filter((t) => t != null);
}
function Le(e) {
  return e instanceof ke || e instanceof Ee ? e.status : 500;
}
function Pt(e) {
  return e instanceof Ee ? e.text : "Internal Error";
}
let k, Y, pe;
const $t = je.toString().includes("$$") || /function \w+\(\) \{\}/.test(je.toString());
$t ? (k = { data: {}, form: null, error: null, params: {}, route: { id: null }, state: {}, status: -1, url: new URL("https://example.com") }, Y = { current: null }, pe = { current: false }) : (k = new (_c = class {
  constructor() {
    __privateAdd(this, _e2, A({}));
    __privateAdd(this, _t2, A(null));
    __privateAdd(this, _n, A(null));
    __privateAdd(this, _a2, A({}));
    __privateAdd(this, _r, A({ id: null }));
    __privateAdd(this, _o, A({}));
    __privateAdd(this, _s, A(-1));
    __privateAdd(this, _i, A(new URL("https://example.com")));
  }
  get data() {
    return U(__privateGet(this, _e2));
  }
  set data(t) {
    T(__privateGet(this, _e2), t);
  }
  get form() {
    return U(__privateGet(this, _t2));
  }
  set form(t) {
    T(__privateGet(this, _t2), t);
  }
  get error() {
    return U(__privateGet(this, _n));
  }
  set error(t) {
    T(__privateGet(this, _n), t);
  }
  get params() {
    return U(__privateGet(this, _a2));
  }
  set params(t) {
    T(__privateGet(this, _a2), t);
  }
  get route() {
    return U(__privateGet(this, _r));
  }
  set route(t) {
    T(__privateGet(this, _r), t);
  }
  get state() {
    return U(__privateGet(this, _o));
  }
  set state(t) {
    T(__privateGet(this, _o), t);
  }
  get status() {
    return U(__privateGet(this, _s));
  }
  set status(t) {
    T(__privateGet(this, _s), t);
  }
  get url() {
    return U(__privateGet(this, _i));
  }
  set url(t) {
    T(__privateGet(this, _i), t);
  }
}, _e2 = new WeakMap(), _t2 = new WeakMap(), _n = new WeakMap(), _a2 = new WeakMap(), _r = new WeakMap(), _o = new WeakMap(), _s = new WeakMap(), _i = new WeakMap(), _c)(), Y = new (_d = class {
  constructor() {
    __privateAdd(this, _e3, A(null));
  }
  get current() {
    return U(__privateGet(this, _e3));
  }
  set current(t) {
    T(__privateGet(this, _e3), t);
  }
}, _e3 = new WeakMap(), _d)(), pe = new (_e5 = class {
  constructor() {
    __privateAdd(this, _e4, A(false));
  }
  get current() {
    return U(__privateGet(this, _e4));
  }
  set current(t) {
    T(__privateGet(this, _e4), t);
  }
}, _e4 = new WeakMap(), _e5)(), Xe.v = () => pe.current = true);
function Ct(e) {
  Object.assign(k, e);
}
const jt = /* @__PURE__ */ new Set(["icon", "shortcut icon", "apple-touch-icon"]), C = We(Ge) ?? {}, z = We(Fe) ?? {}, $ = { url: qe({}), page: qe({}), navigating: be(null), updated: Tt() };
function Ae(e) {
  C[e] = D();
}
function Nt(e, t) {
  let n = e + 1;
  for (; C[n]; ) delete C[n], n += 1;
  for (n = t + 1; z[n]; ) delete z[n], n += 1;
}
function H(e, t = false) {
  return t ? location.replace(e.href) : location.href = e.href, new Promise(() => {
  });
}
async function Ze() {
  if ("serviceWorker" in navigator) {
    const e = await navigator.serviceWorker.getRegistration(x || "/");
    e && await e.update();
  }
}
function Ve() {
}
let Ue, me, ne, I, we, y;
const ae = [], re = [];
let L = null;
function ve() {
  var _a3;
  (_a3 = L == null ? void 0 : L.fork) == null ? void 0 : _a3.then((e) => e == null ? void 0 : e.discard()), L = null;
}
const Z = /* @__PURE__ */ new Map(), et = /* @__PURE__ */ new Set(), Dt = /* @__PURE__ */ new Set(), F = /* @__PURE__ */ new Set();
let _ = { branch: [], error: null, url: null }, tt = false, oe = false, Ke = true, J = false, B = false, nt = false, Te = false, at, v, R, N;
const se = /* @__PURE__ */ new Set(), Me = /* @__PURE__ */ new Map();
async function ln(e, t, n) {
  var _a3, _b2, _c2, _d2, _e6;
  ((_a3 = globalThis.__sveltekit_hh7rax) == null ? void 0 : _a3.data) && globalThis.__sveltekit_hh7rax.data, document.URL !== location.href && (location.href = location.href), y = e, await ((_c2 = (_b2 = e.hooks).init) == null ? void 0 : _c2.call(_b2)), Ue = xt(e), I = document.documentElement, we = t, me = e.nodes[0], ne = e.nodes[1], me(), ne(), v = (_d2 = history.state) == null ? void 0 : _d2[K], R = (_e6 = history.state) == null ? void 0 : _e6[G], v || (v = R = Date.now(), history.replaceState({ ...history.state, [K]: v, [G]: R }, ""));
  const a = C[v];
  function r() {
    a && (history.scrollRestoration = "manual", scrollTo(a.x, a.y));
  }
  n ? (r(), await Qt(we, n)) : (await M({ type: "enter", url: ze(y.hash ? tn(new URL(location.href)) : location.href), replace_state: true }), r()), Xt();
}
function qt() {
  ae.length = 0, Te = false;
}
function rt(e) {
  re.some((t) => t == null ? void 0 : t.snapshot) && (z[e] = re.map((t) => {
    var _a3;
    return (_a3 = t == null ? void 0 : t.snapshot) == null ? void 0 : _a3.capture();
  }));
}
function ot(e) {
  var _a3;
  (_a3 = z[e]) == null ? void 0 : _a3.forEach((t, n) => {
    var _a4, _b2;
    (_b2 = (_a4 = re[n]) == null ? void 0 : _a4.snapshot) == null ? void 0 : _b2.restore(t);
  });
}
function Be() {
  Ae(v), Ne(Ge, C), rt(R), Ne(Fe, z);
}
async function Vt(e, t, n, a) {
  let r;
  t.invalidateAll && ve(), await M({ type: "goto", url: ze(e), keepfocus: t.keepFocus, noscroll: t.noScroll, replace_state: t.replaceState, state: t.state, redirect_count: n, nav_token: a, accept: () => {
    t.invalidateAll && (Te = true, r = [...Me.keys()]), t.invalidate && t.invalidate.forEach(Jt);
  } }), t.invalidateAll && ee().then(ee).then(() => {
    Me.forEach(({ resource: i }, o) => {
      var _a3;
      (r == null ? void 0 : r.includes(o)) && ((_a3 = i.refresh) == null ? void 0 : _a3.call(i));
    });
  });
}
async function Kt(e) {
  if (e.id !== (L == null ? void 0 : L.id)) {
    ve();
    const t = {};
    se.add(t), L = { id: e.id, token: t, promise: it({ ...e, preload: t }).then((n) => (se.delete(t), n.type === "loaded" && n.state.error && ve(), n)), fork: null };
  }
  return L.promise;
}
async function ge(e) {
  var _a3;
  const t = (_a3 = await ue(e, false)) == null ? void 0 : _a3.route;
  t && await Promise.all([...t.layouts, t.leaf].map((n) => n == null ? void 0 : n[1]()));
}
async function st(e, t, n) {
  var _a3;
  _ = e.state;
  const a = document.querySelector("style[data-sveltekit]");
  if (a && a.remove(), Object.assign(k, e.props.page), at = new y.root({ target: t, props: { ...e.props, stores: $, components: re }, hydrate: n, sync: false }), await Promise.resolve(), ot(R), n) {
    const r = { from: null, to: { params: _.params, route: { id: ((_a3 = _.route) == null ? void 0 : _a3.id) ?? null }, url: new URL(location.href), scroll: C[v] ?? D() }, willUnload: false, type: "enter", complete: Promise.resolve() };
    F.forEach((i) => i(r));
  }
  oe = true;
}
function ie({ url: e, params: t, branch: n, status: a, error: r, route: i, form: o }) {
  let s = "never";
  if (x && (e.pathname === x || e.pathname === x + "/")) s = "always";
  else for (const u of n) (u == null ? void 0 : u.slash) !== void 0 && (s = u.slash);
  e.pathname = ht(e.pathname, s), e.search = e.search;
  const c = { type: "loaded", state: { url: e, params: t, branch: n, error: r, route: i }, props: { constructors: Ot(n).map((u) => u.node.component), page: Ce(k) } };
  o !== void 0 && (c.props.form = o);
  let l = {}, f = !k, h = 0;
  for (let u = 0; u < Math.max(n.length, _.branch.length); u += 1) {
    const p = n[u], m = _.branch[u];
    (p == null ? void 0 : p.data) !== (m == null ? void 0 : m.data) && (f = true), p && (l = { ...l, ...p.data }, f && (c.props[`data_${h}`] = l), h += 1);
  }
  return (!_.url || e.href !== _.url.href || _.error !== r || o !== void 0 && o !== k.form || f) && (c.props.page = { error: r, params: t, route: { id: (i == null ? void 0 : i.id) ?? null }, state: {}, status: a, url: new URL(e), form: o ?? null, data: f ? l : k.data }), c;
}
async function Ie({ loader: e, parent: t, url: n, params: a, route: r, server_data_node: i }) {
  var _a3, _b2;
  let o = null;
  const s = { dependencies: /* @__PURE__ */ new Set(), params: /* @__PURE__ */ new Set(), parent: false, route: false, url: false, search_params: /* @__PURE__ */ new Set() }, c = await e();
  return { node: c, loader: e, server: i, universal: ((_a3 = c.universal) == null ? void 0 : _a3.load) ? { type: "data", data: o, uses: s } : null, data: o ?? (i == null ? void 0 : i.data) ?? null, slash: ((_b2 = c.universal) == null ? void 0 : _b2.trailingSlash) ?? (i == null ? void 0 : i.slash) };
}
function Mt(e, t, n) {
  let a = e instanceof Request ? e.url : e;
  const r = new URL(a, n);
  r.origin === n.origin && (a = r.href.slice(n.origin.length));
  const i = oe ? yt(a, r.href, t) : vt(a, t);
  return { resolved: r, promise: i };
}
function Bt(e, t, n, a, r, i) {
  if (Te) return true;
  if (!r) return false;
  if (r.parent && e || r.route && t || r.url && n) return true;
  for (const o of r.search_params) if (a.has(o)) return true;
  for (const o of r.params) if (i[o] !== _.params[o]) return true;
  for (const o of r.dependencies) if (ae.some((s) => s(new URL(o)))) return true;
  return false;
}
function Oe(e, t) {
  return (e == null ? void 0 : e.type) === "data" ? e : (e == null ? void 0 : e.type) === "skip" ? t ?? null : null;
}
function Wt(e, t) {
  if (!e) return new Set(t.searchParams.keys());
  const n = /* @__PURE__ */ new Set([...e.searchParams.keys(), ...t.searchParams.keys()]);
  for (const a of n) {
    const r = e.searchParams.getAll(a), i = t.searchParams.getAll(a);
    r.every((o) => i.includes(o)) && i.every((o) => r.includes(o)) && n.delete(a);
  }
  return n;
}
function Ft({ error: e, url: t, route: n, params: a }) {
  return { type: "loaded", state: { error: e, url: t, route: n, params: a, branch: [] }, props: { page: Ce(k), constructors: [] } };
}
async function it({ id: e, invalidating: t, url: n, params: a, route: r, preload: i }) {
  if ((L == null ? void 0 : L.id) === e) return se.delete(L.token), L.promise;
  const { errors: o, layouts: s, leaf: c } = r, l = [...s, c];
  o.forEach((g) => g == null ? void 0 : g().catch(() => {
  })), l.forEach((g) => g == null ? void 0 : g[1]().catch(() => {
  }));
  const f = _.url ? e !== ce(_.url) : false, h = _.route ? r.id !== _.route.id : false, w = Wt(_.url, n);
  let u = false;
  const p = l.map(async (g, d) => {
    var _a3;
    if (!g) return;
    const S = _.branch[d];
    return g[1] === (S == null ? void 0 : S.loader) && !Bt(u, h, f, w, (_a3 = S.universal) == null ? void 0 : _a3.uses, a) ? S : (u = true, Ie({ loader: g[1], url: n, params: a, route: r, parent: async () => {
      var _a4;
      const O = {};
      for (let P = 0; P < d; P += 1) Object.assign(O, (_a4 = await p[P]) == null ? void 0 : _a4.data);
      return O;
    }, server_data_node: Oe(g[0] ? { type: "skip" } : null, g[0] ? S == null ? void 0 : S.server : void 0) }));
  });
  for (const g of p) g.catch(() => {
  });
  const m = [];
  for (let g = 0; g < l.length; g += 1) if (l[g]) try {
    m.push(await p[g]);
  } catch (d) {
    if (d instanceof Se) return { type: "redirect", location: d.location };
    if (se.has(i)) return Ft({ error: await X(d, { params: a, url: n, route: { id: r.id } }), url: n, params: a, route: r });
    let S = Le(d), E;
    if (d instanceof ke) E = d.body;
    else {
      if (await $.updated.check()) return await Ze(), await H(n);
      E = await X(d, { params: a, url: n, route: { id: r.id } });
    }
    const O = await Gt(g, m, o);
    return O ? ie({ url: n, params: a, branch: m.slice(0, O.idx).concat(O.node), status: S, error: E, route: r }) : await lt(n, { id: r.id }, E, S);
  }
  else m.push(void 0);
  return ie({ url: n, params: a, branch: m, status: 200, error: null, route: r, form: t ? void 0 : null });
}
async function Gt(e, t, n) {
  for (; e--; ) if (n[e]) {
    let a = e;
    for (; !t[a]; ) a -= 1;
    try {
      return { idx: a + 1, node: { node: await n[e](), loader: n[e], data: {}, server: null, universal: null } };
    } catch {
      continue;
    }
  }
}
async function Pe({ status: e, error: t, url: n, route: a }) {
  const r = {};
  let i = null;
  try {
    const o = await Ie({ loader: me, url: n, params: r, route: a, parent: () => Promise.resolve({}), server_data_node: Oe(i) }), s = { node: await ne(), loader: ne, universal: null, server: null, data: null };
    return ie({ url: n, params: r, branch: [o, s], status: e, error: t, route: null });
  } catch (o) {
    if (o instanceof Se) return Vt(new URL(o.location, location.href), {}, 0);
    throw o;
  }
}
async function Yt(e) {
  const t = e.href;
  if (Z.has(t)) return Z.get(t);
  let n;
  try {
    const a = (async () => {
      let r = await y.hooks.reroute({ url: new URL(e), fetch: async (i, o) => Mt(i, o, e).promise }) ?? e;
      if (typeof r == "string") {
        const i = new URL(e);
        y.hash ? i.hash = r : i.pathname = r, r = i;
      }
      return r;
    })();
    Z.set(t, a), n = await a;
  } catch {
    Z.delete(t);
    return;
  }
  return n;
}
async function ue(e, t) {
  if (e && !le(e, x, y.hash)) {
    const n = await Yt(e);
    if (!n) return;
    const a = zt(n);
    for (const r of Ue) {
      const i = r.exec(a);
      if (i) return { id: ce(e), invalidating: t, route: r, params: gt(i), url: e };
    }
  }
}
function zt(e) {
  return pt(y.hash ? e.hash.replace(/^#/, "").replace(/[?#].+/, "") : e.pathname.slice(x.length)) || "/";
}
function ce(e) {
  return (y.hash ? e.hash.replace(/^#/, "") : e.pathname) + e.search;
}
function ct({ url: e, type: t, intent: n, delta: a, event: r, scroll: i }) {
  let o = false;
  const s = $e(_, n, e, t, i ?? null);
  a !== void 0 && (s.navigation.delta = a), r !== void 0 && (s.navigation.event = r);
  const c = { ...s.navigation, cancel: () => {
    o = true, s.reject(new Error("navigation cancelled"));
  } };
  return J || et.forEach((l) => l(c)), o ? null : s;
}
async function M({ type: e, url: t, popped: n, keepfocus: a, noscroll: r, replace_state: i, state: o = {}, redirect_count: s = 0, nav_token: c = {}, accept: l = Ve, block: f = Ve, event: h }) {
  var _a3;
  const w = N;
  N = c;
  const u = await ue(t, false), p = e === "enter" ? $e(_, u, t, e) : ct({ url: t, type: e, delta: n == null ? void 0 : n.delta, intent: u, scroll: n == null ? void 0 : n.scroll, event: h });
  if (!p) {
    f(), N === c && (N = w);
    return;
  }
  const m = v, g = R;
  l(), J = true, oe && p.navigation.type !== "enter" && $.navigating.set(Y.current = p.navigation);
  let d = u && await it(u);
  if (!d) {
    if (le(t, x, y.hash)) return await H(t, i);
    d = await lt(t, { id: null }, await X(new Ee(404, "Not Found", `Not found: ${t.pathname}`), { url: t, params: {}, route: { id: null } }), 404, i);
  }
  if (t = (u == null ? void 0 : u.url) || t, N !== c) return p.reject(new Error("navigation aborted")), false;
  if (d.type === "redirect") {
    if (s < 20) {
      await M({ type: e, url: new URL(d.location, t), popped: n, keepfocus: a, noscroll: r, replace_state: i, state: o, redirect_count: s + 1, nav_token: c }), p.fulfil(void 0);
      return;
    }
    d = await Pe({ status: 500, error: await X(new Error("Redirect loop"), { url: t, params: {}, route: { id: null } }), url: t, route: { id: null } });
  } else d.props.page.status >= 400 && await $.updated.check() && (await Ze(), await H(t, i));
  if (qt(), Ae(m), rt(g), d.props.page.url.pathname !== t.pathname && (t.pathname = d.props.page.url.pathname), o = n ? n.state : o, !n) {
    const b = i ? 0 : 1, Q = { [K]: v += b, [G]: R += b, [Ye]: o };
    (i ? history.replaceState : history.pushState).call(history, Q, "", t), i || Nt(v, R);
  }
  const S = u && (L == null ? void 0 : L.id) === u.id ? L.fork : null;
  L = null, d.props.page.state = o;
  let E;
  if (oe) {
    const b = (await Promise.all(Array.from(Dt, (q) => q(p.navigation)))).filter((q) => typeof q == "function");
    if (b.length > 0) {
      let q = function() {
        b.forEach((fe) => {
          F.delete(fe);
        });
      };
      b.push(q), b.forEach((fe) => {
        F.add(fe);
      });
    }
    _ = d.state, d.props.page && (d.props.page.url = t);
    const Q = S && await S;
    Q ? E = Q.commit() : (at.$set(d.props), Ct(d.props.page), E = (_a3 = dt) == null ? void 0 : _a3()), nt = true;
  } else await st(d, we, false);
  const { activeElement: O } = document;
  await E, await ee(), await ee();
  let P = null;
  if (Ke) {
    const b = n ? n.scroll : r ? D() : null;
    b ? scrollTo(b.x, b.y) : (P = t.hash && document.getElementById(ut(t))) ? P.scrollIntoView() : scrollTo(0, 0);
  }
  const ft = document.activeElement !== O && document.activeElement !== document.body;
  !a && !ft && en(t, !P), Ke = true, d.props.page && Object.assign(k, d.props.page), J = false, e === "popstate" && ot(R), p.fulfil(void 0), p.navigation.to && (p.navigation.to.scroll = D()), F.forEach((b) => b(p.navigation)), $.navigating.set(Y.current = null);
}
async function lt(e, t, n, a, r) {
  return e.origin === xe && e.pathname === location.pathname && !tt ? await Pe({ status: a, error: n, url: e, route: t }) : await H(e, r);
}
function Ht() {
  let e, t = { element: void 0, href: void 0 }, n;
  I.addEventListener("mousemove", (s) => {
    const c = s.target;
    clearTimeout(e), e = setTimeout(() => {
      i(c, j.hover);
    }, 20);
  });
  function a(s) {
    s.defaultPrevented || i(s.composedPath()[0], j.tap);
  }
  I.addEventListener("mousedown", a), I.addEventListener("touchstart", a, { passive: true });
  const r = new IntersectionObserver((s) => {
    for (const c of s) c.isIntersecting && (ge(new URL(c.target.href)), r.unobserve(c.target));
  }, { threshold: 0 });
  async function i(s, c) {
    const l = Je(s, I), f = l === t.element && (l == null ? void 0 : l.href) === t.href && c >= n;
    if (!l || f) return;
    const { url: h, external: w, download: u } = _e(l, x, y.hash);
    if (w || u) return;
    const p = te(l), m = h && ce(_.url) === ce(h);
    if (!(p.reload || m)) if (c <= p.preload_data) {
      t = { element: l, href: l.href }, n = j.tap;
      const g = await ue(h, false);
      if (!g) return;
      Kt(g);
    } else c <= p.preload_code && (t = { element: l, href: l.href }, n = c, ge(h));
  }
  function o() {
    r.disconnect();
    for (const s of I.querySelectorAll("a")) {
      const { url: c, external: l, download: f } = _e(s, x, y.hash);
      if (l || f) continue;
      const h = te(s);
      h.reload || (h.preload_code === j.viewport && r.observe(s), h.preload_code === j.eager && ge(c));
    }
  }
  F.add(o), o();
}
function X(e, t) {
  if (e instanceof ke) return e.body;
  const n = Le(e), a = Pt(e);
  return y.hooks.handleError({ error: e, event: t, status: n, message: a }) ?? { message: a };
}
function Jt(e) {
  if (typeof e == "function") ae.push(e);
  else {
    const { href: t } = new URL(e, location.href);
    ae.push((n) => n.href === t);
  }
}
function Xt() {
  var _a3;
  history.scrollRestoration = "manual", addEventListener("beforeunload", (t) => {
    let n = false;
    if (Be(), !J) {
      const a = $e(_, void 0, null, "leave"), r = { ...a.navigation, cancel: () => {
        n = true, a.reject(new Error("navigation cancelled"));
      } };
      et.forEach((i) => i(r));
    }
    n ? (t.preventDefault(), t.returnValue = "") : history.scrollRestoration = "auto";
  }), addEventListener("visibilitychange", () => {
    document.visibilityState === "hidden" && Be();
  }), ((_a3 = navigator.connection) == null ? void 0 : _a3.saveData) || Ht(), I.addEventListener("click", async (t) => {
    if (t.button || t.which !== 1 || t.metaKey || t.ctrlKey || t.shiftKey || t.altKey || t.defaultPrevented) return;
    const n = Je(t.composedPath()[0], I);
    if (!n) return;
    const { url: a, external: r, target: i, download: o } = _e(n, x, y.hash);
    if (!a) return;
    if (i === "_parent" || i === "_top") {
      if (window.parent !== window) return;
    } else if (i && i !== "_self") return;
    const s = te(n);
    if (!(n instanceof SVGAElement) && a.protocol !== location.protocol && !(a.protocol === "https:" || a.protocol === "http:") || o) return;
    const [l, f] = (y.hash ? a.hash.replace(/^#/, "") : a.href).split("#"), h = l === de(location);
    if (r || s.reload && (!h || !f)) {
      ct({ url: a, type: "link", event: t }) ? J = true : t.preventDefault();
      return;
    }
    if (f !== void 0 && h) {
      const [, w] = _.url.href.split("#");
      if (w === f) {
        if (t.preventDefault(), f === "" || f === "top" && n.ownerDocument.getElementById("top") === null) scrollTo({ top: 0 });
        else {
          const u = n.ownerDocument.getElementById(decodeURIComponent(f));
          u && (u.scrollIntoView(), u.focus());
        }
        return;
      }
      if (B = true, Ae(v), e(a), !s.replace_state) return;
      B = false;
    }
    t.preventDefault(), await new Promise((w) => {
      requestAnimationFrame(() => {
        setTimeout(w, 0);
      }), setTimeout(w, 100);
    }), await M({ type: "link", url: a, keepfocus: s.keepfocus, noscroll: s.noscroll, replace_state: s.replace_state ?? a.href === location.href, event: t });
  }), I.addEventListener("submit", (t) => {
    if (t.defaultPrevented) return;
    const n = HTMLFormElement.prototype.cloneNode.call(t.target), a = t.submitter;
    if (((a == null ? void 0 : a.formTarget) || n.target) === "_blank" || ((a == null ? void 0 : a.formMethod) || n.method) !== "get") return;
    const o = new URL((a == null ? void 0 : a.hasAttribute("formaction")) && (a == null ? void 0 : a.formAction) || n.action);
    if (le(o, x, false)) return;
    const s = t.target, c = te(s);
    if (c.reload) return;
    t.preventDefault(), t.stopPropagation();
    const l = new FormData(s, a);
    o.search = new URLSearchParams(l).toString(), M({ type: "form", url: o, keepfocus: c.keepfocus, noscroll: c.noscroll, replace_state: c.replace_state ?? o.href === location.href, event: t });
  }), addEventListener("popstate", async (t) => {
    var _a4;
    if (!ye) {
      if ((_a4 = t.state) == null ? void 0 : _a4[K]) {
        const n = t.state[K];
        if (N = {}, n === v) return;
        const a = C[n], r = t.state[Ye] ?? {}, i = new URL(t.state[Ut] ?? location.href), o = t.state[G], s = _.url ? de(location) === de(_.url) : false;
        if (o === R && (nt || s)) {
          r !== k.state && (k.state = r), e(i), C[v] = D(), a && scrollTo(a.x, a.y), v = n;
          return;
        }
        const l = n - v;
        await M({ type: "popstate", url: i, popped: { state: r, scroll: a, delta: l }, accept: () => {
          v = n, R = o;
        }, block: () => {
          history.go(-l);
        }, nav_token: N, event: t });
      } else if (!B) {
        const n = new URL(location.href);
        e(n), y.hash && location.reload();
      }
    }
  }), addEventListener("hashchange", () => {
    B && (B = false, history.replaceState({ ...history.state, [K]: ++v, [G]: R }, "", location.href));
  });
  for (const t of document.querySelectorAll("link")) jt.has(t.rel) && (t.href = t.href);
  addEventListener("pageshow", (t) => {
    t.persisted && $.navigating.set(Y.current = null);
  });
  function e(t) {
    _.url = k.url = t, $.page.set(Ce(k)), $.page.notify();
  }
}
async function Qt(e, { status: t = 200, error: n, node_ids: a, params: r, route: i, server_route: o, data: s, form: c }) {
  tt = true;
  const l = new URL(location.href);
  let f;
  ({ params: r = {}, route: i = { id: null } } = await ue(l, false) || {}), f = Ue.find(({ id: u }) => u === i.id);
  let h, w = true;
  try {
    const u = a.map(async (m, g) => {
      const d = s[g];
      return (d == null ? void 0 : d.uses) && (d.uses = Zt(d.uses)), Ie({ loader: y.nodes[m], url: l, params: r, route: i, parent: async () => {
        const S = {};
        for (let E = 0; E < g; E += 1) Object.assign(S, (await u[E]).data);
        return S;
      }, server_data_node: Oe(d) });
    }), p = await Promise.all(u);
    if (f) {
      const m = f.layouts;
      for (let g = 0; g < m.length; g++) m[g] || p.splice(g, 0, void 0);
    }
    h = ie({ url: l, params: r, branch: p, status: t, error: n, form: c, route: f ?? null });
  } catch (u) {
    if (u instanceof Se) {
      await H(new URL(u.location, location.href));
      return;
    }
    h = await Pe({ status: Le(u), error: await X(u, { url: l, params: r, route: i }), url: l, route: i }), e.textContent = "", w = false;
  }
  h.props.page && (h.props.page.state = {}), await st(h, e, w);
}
function Zt(e) {
  return { dependencies: new Set((e == null ? void 0 : e.dependencies) ?? []), params: new Set((e == null ? void 0 : e.params) ?? []), parent: !!(e == null ? void 0 : e.parent), route: !!(e == null ? void 0 : e.route), url: !!(e == null ? void 0 : e.url), search_params: new Set((e == null ? void 0 : e.search_params) ?? []) };
}
let ye = false;
function en(e, t = true) {
  const n = document.querySelector("[autofocus]");
  if (n) n.focus();
  else {
    const a = ut(e);
    if (a && document.getElementById(a)) {
      const { x: i, y: o } = D();
      setTimeout(() => {
        const s = history.state;
        ye = true, location.replace(new URL(`#${a}`, location.href)), history.replaceState(s, "", e), t && scrollTo(i, o), ye = false;
      });
    } else {
      const i = document.body, o = i.getAttribute("tabindex");
      i.tabIndex = -1, i.focus({ preventScroll: true, focusVisible: false }), o !== null ? i.setAttribute("tabindex", o) : i.removeAttribute("tabindex");
    }
    const r = getSelection();
    if (r && r.type !== "None") {
      const i = [];
      for (let o = 0; o < r.rangeCount; o += 1) i.push(r.getRangeAt(o));
      setTimeout(() => {
        if (r.rangeCount === i.length) {
          for (let o = 0; o < r.rangeCount; o += 1) {
            const s = i[o], c = r.getRangeAt(o);
            if (s.commonAncestorContainer !== c.commonAncestorContainer || s.startContainer !== c.startContainer || s.endContainer !== c.endContainer || s.startOffset !== c.startOffset || s.endOffset !== c.endOffset) return;
          }
          r.removeAllRanges();
        }
      });
    }
  }
}
function $e(e, t, n, a, r = null) {
  var _a3, _b2;
  let i, o;
  const s = new Promise((l, f) => {
    i = l, o = f;
  });
  return s.catch(() => {
  }), { navigation: { from: { params: e.params, route: { id: ((_a3 = e.route) == null ? void 0 : _a3.id) ?? null }, url: e.url, scroll: D() }, to: n && { params: (t == null ? void 0 : t.params) ?? null, route: { id: ((_b2 = t == null ? void 0 : t.route) == null ? void 0 : _b2.id) ?? null }, url: n, scroll: r }, willUnload: !t, type: a, complete: s }, fulfil: i, reject: o };
}
function Ce(e) {
  return { data: e.data, error: e.error, form: e.form, params: e.params, route: e.route, state: e.state, status: e.status, url: e.url };
}
function tn(e) {
  const t = new URL(e);
  return t.hash = decodeURIComponent(e.hash), t;
}
function ut(e) {
  let t;
  if (y.hash) {
    const [, , n] = e.hash.split("#", 3);
    t = n ?? "";
  } else t = e.hash.slice(1);
  return decodeURIComponent(t);
}
export {
  ln as a,
  rn as l,
  k as p,
  $ as s
};
