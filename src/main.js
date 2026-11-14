"use strict";

/* AVIFnim Viewer — frontend (Tauri 2 + webview)
 * Sem bundler: usa window.__TAURI__ (withGlobalTauri = true).
 */

const TAURI = window.__TAURI__ || {};
const invoke = TAURI.core ? TAURI.core.invoke : async () => { throw new Error("Tauri indisponivel"); };
const listen = TAURI.event ? TAURI.event.listen : async () => () => {};
const getCurrentWebview = TAURI.webview ? TAURI.webview.getCurrentWebview : () => null;

const SEQ_EXTS = ["png", "jpg", "jpeg"];
const BG_ORDER = ["dark", "light", "gray"];
const PRELOAD_AHEAD = 12;
const SEQ_BUFFER_MAX = 48;

const DEFAULT_SETTINGS = {
  theme: "auto",
  lang: "pt",
  prefetch_n: 2,
  prefetch_mb: 128,
  background: "dark",
  checkerboard: true,
  seq_fps: 60,
  show_tips: true,
  promo_seen: false,
  banner_dismissed: false,
  reset_view_on_navigate: true,
  view_count: 0,
  associations: { avif: false, webp: false, gif: false, pnm: false },
};

const I18N = {
  pt: {
    "toolbar.open": "Abrir (Ctrl+O)",
    "toolbar.prev": "Anterior (←)",
    "toolbar.next": "Próximo (→)",
    "toolbar.zoomOut": "Diminuir zoom (−)",
    "toolbar.zoomIn": "Aumentar zoom (+)",
    "toolbar.fit": "Fit",
    "toolbar.actual": "Tamanho real (100%)",
    "toolbar.fullscreen": "Tela cheia (F)",
    "toolbar.background": "Alternar fundo",
    "toolbar.checker": "Checkerboard de transparência",
    "toolbar.info": "Informações (I)",
    "toolbar.settings": "Configurações",
    "toolbar.about": "Sobre",
    "toolbar.noFile": "Nenhum arquivo aberto",

    "empty.title": "Arraste uma imagem aqui",
    "empty.subtitle": "AVIF, WebP e GIF animados · PPM/PNM · sequências PNG/JPG",
    "empty.open": "Abrir arquivo",
    "empty.hint": "Dica: use ← → para navegar pela pasta",

    "error.generic": "Não foi possível abrir este arquivo.",
    "error.decode": "Não foi possível decodificar este arquivo (corrompido ou formato não suportado pelo webview).",
    "error.openAnother": "Abrir outro",
    "banner.text": "Este {format} tem {size}. Em AVIF ficaria ~{est}.",
    "banner.cta": "Converter com AVIFnim",

    "info.title": "Informações",
    "info.name": "Nome",
    "info.format": "Formato",
    "info.dimensions": "Dimensões",
    "info.size": "Tamanho",
    "info.sequence": "Sequência",
    "info.path": "Caminho",
    "info.none": "—",
    "info.seqNone": "Não é sequência",
    "seq.bar": "{n} frames · {fps} fps · loop",

    "common.close": "Fechar",
    "common.type": "Tipo",

    "settings.title": "Configurações",
    "settings.assoc.title": "Associações de arquivo",
    "settings.assoc.desc": "Opcional e desmarcado por padrão. O AVIFnim Viewer nunca assume seus formatos sozinho.",
    "settings.assoc.default": "Definir como padrão…",
    "settings.assoc.hint": "Abre as configurações do sistema; o app não altera o padrão silenciosamente.",
    "settings.appearance": "Aparência",
    "settings.theme": "Tema",
    "settings.lang": "Idioma",
    "settings.background": "Fundo padrão",
    "settings.checker": "Mostrar checkerboard de transparência",
    "settings.tips": "Mostrar dicas",
    "settings.resetView": "Resetar zoom/pan ao trocar de arquivo",
    "settings.performance": "Desempenho",
    "settings.prefetchN": "Prefetch (arquivos para cada lado)",
    "settings.prefetchMb": "Teto de memória do prefetch",
    "settings.sequence": "Sequências PNG/JPG (G7)",
    "settings.seqFps": "Velocidade de reprodução",
    "settings.seqHint": "Ao abrir um arquivo com número no nome, a sequência da pasta é detectada e tocada em loop.",
    "settings.reset": "Reset",
    "settings.resetPromo": "Mostrar o modal do AVIFnim novamente",
    "settings.resetAll": "Restaurar padrões",
    "settings.saved": "Salvo automaticamente",
    "theme.auto": "Seguir o sistema",
    "theme.light": "Claro",
    "theme.dark": "Escuro",
    "bg.dark": "Escuro",
    "bg.light": "Claro",
    "bg.gray": "Cinza",

    "about.title": "Sobre o AVIFnim Viewer",
    "about.tagline": "Visualize AVIF, WebP e GIF animados no seu PC.",
    "about.license": "Licença",
    "about.free": "gratuito e open source",
    "about.promo.title": "Conheça o AVIFnim",
    "about.promo.text": "Converta sequências, vídeo e gravação de tela para AVIF animado. O conversor que combina com este visualizador.",
    "about.promo.cta": "Conheça o AVIFnim",
    "about.author": "Autor",
    "about.email": "E-mail",
    "about.site": "Site",
    "about.repo": "Repositório",
    "about.donate": "Colabore com o autor",
    "about.donate.pending": "QR codes do PayPal e do PIX serão adicionados em breve.",
    "about.limits": "Limitações conhecidas",
    "about.limits.loop": "Arquivos com loop count 1x tocam uma vez e param (comportamento do formato).",
    "about.limits.fps": "O FPS efetivo depende do webview do sistema.",
    "about.limits.pnm": "PPM/PNM é sempre exibido como still e assume sRGB (sem perfil de cor).",
    "about.viewRepo": "Ver no GitHub",

    "promo.title": "Este é o AVIFnim Viewer",
    "promo.text": "Gostou de ver AVIF animado? O AVIFnim é o conversor que cria AVIF animado a partir de sequências, vídeo e gravação de tela.",
    "promo.note": "Este aviso aparece uma única vez.",
    "promo.never": "Não mostrar novamente",
    "promo.cta": "Conhecer o AVIFnim",

    "toast.assoc": "Associação atualizada: {fmt}",
    "toast.saveFail": "Falha ao salvar as configurações",
    "toast.linkPending": "O link oficial do AVIFnim será adicionado em breve.",
    "toast.reset": "Configurações restauradas",
    "toast.promoReset": "O modal do AVIFnim voltará a aparecer",
    "toast.copied": "Copiado",
  },
  en: {
    "toolbar.open": "Open (Ctrl+O)",
    "toolbar.prev": "Previous (←)",
    "toolbar.next": "Next (→)",
    "toolbar.zoomOut": "Zoom out (−)",
    "toolbar.zoomIn": "Zoom in (+)",
    "toolbar.fit": "Fit",
    "toolbar.actual": "Actual size (100%)",
    "toolbar.fullscreen": "Fullscreen (F)",
    "toolbar.background": "Cycle background",
    "toolbar.checker": "Transparency checkerboard",
    "toolbar.info": "Info (I)",
    "toolbar.settings": "Settings",
    "toolbar.about": "About",
    "toolbar.noFile": "No file open",

    "empty.title": "Drag an image here",
    "empty.subtitle": "Animated AVIF, WebP and GIF · PPM/PNM · PNG/JPG sequences",
    "empty.open": "Open file",
    "empty.hint": "Tip: use ← → to browse the folder",

    "error.generic": "Could not open this file.",
    "error.decode": "Could not decode this file (corrupted or a format the system webview does not support).",
    "error.openAnother": "Open another",
    "banner.text": "This {format} is {size}. As AVIF it would be ~{est}.",
    "banner.cta": "Convert with AVIFnim",

    "info.title": "Information",
    "info.name": "Name",
    "info.format": "Format",
    "info.dimensions": "Dimensions",
    "info.size": "Size",
    "info.sequence": "Sequence",
    "info.path": "Path",
    "info.none": "—",
    "info.seqNone": "Not a sequence",
    "seq.bar": "{n} frames · {fps} fps · loop",

    "common.close": "Close",
    "common.type": "Type",

    "settings.title": "Settings",
    "settings.assoc.title": "File associations",
    "settings.assoc.desc": "Optional and unchecked by default. AVIFnim Viewer never takes over your formats on its own.",
    "settings.assoc.default": "Set as default…",
    "settings.assoc.hint": "Opens the system settings; the app never silently changes defaults.",
    "settings.appearance": "Appearance",
    "settings.theme": "Theme",
    "settings.lang": "Language",
    "settings.background": "Default background",
    "settings.checker": "Show transparency checkerboard",
    "settings.tips": "Show tips",
    "settings.resetView": "Reset zoom/pan when switching files",
    "settings.performance": "Performance",
    "settings.prefetchN": "Prefetch (files on each side)",
    "settings.prefetchMb": "Prefetch memory cap",
    "settings.sequence": "PNG/JPG sequences (G7)",
    "settings.seqFps": "Playback speed",
    "settings.seqHint": "When you open a file with a number in its name, the folder sequence is detected and played in a loop.",
    "settings.reset": "Reset",
    "settings.resetPromo": "Show the AVIFnim modal again",
    "settings.resetAll": "Restore defaults",
    "settings.saved": "Saved automatically",
    "theme.auto": "Follow system",
    "theme.light": "Light",
    "theme.dark": "Dark",
    "bg.dark": "Dark",
    "bg.light": "Light",
    "bg.gray": "Gray",

    "about.title": "About AVIFnim Viewer",
    "about.tagline": "View animated AVIF, WebP and GIF on your PC.",
    "about.license": "License",
    "about.free": "free and open source",
    "about.promo.title": "Meet AVIFnim",
    "about.promo.text": "Convert sequences, video and screen recordings to animated AVIF. The converter that goes with this viewer.",
    "about.promo.cta": "Meet AVIFnim",
    "about.author": "Author",
    "about.email": "Email",
    "about.site": "Website",
    "about.repo": "Repository",
    "about.donate": "Support the author",
    "about.donate.pending": "PayPal and PIX QR codes will be added soon.",
    "about.limits": "Known limitations",
    "about.limits.loop": "Files with loop count 1x play once and stop (format behavior).",
    "about.limits.fps": "Effective FPS depends on the system webview.",
    "about.limits.pnm": "PPM/PNM is always shown as a still and assumes sRGB (no color profile).",
    "about.viewRepo": "View on GitHub",

    "promo.title": "This is AVIFnim Viewer",
    "promo.text": "Enjoyed seeing animated AVIF? AVIFnim is the converter that creates animated AVIF from sequences, video and screen recordings.",
    "promo.note": "This notice appears only once.",
    "promo.never": "Don't show again",
    "promo.cta": "Meet AVIFnim",

    "toast.assoc": "Association updated: {fmt}",
    "toast.saveFail": "Failed to save settings",
    "toast.linkPending": "The official AVIFnim link will be added soon.",
    "toast.reset": "Settings restored",
    "toast.promoReset": "The AVIFnim modal will appear again",
    "toast.copied": "Copied",
  },
};

const state = {
  settings: JSON.parse(JSON.stringify(DEFAULT_SETTINGS)),
  appInfo: null,
  avifnimInstalled: false,
  siblings: [],
  index: -1,
  current: null,
  fitMode: true,
  zoom: 1,
  panX: 0,
  panY: 0,
  seq: null,
  seqTimer: null,
  seqBuffer: new Map(),
  seqLoads: 0,
  bannerDismissedSession: false,
  dragging: false,
  dragStart: null,
  modalOpen: null,
  toastTimer: null,
  bench: false,
  benchFrames: 0,
  benchStart: 0,
  imageErrorShown: false,
};

/* ---------------- helpers ---------------- */

const $ = (id) => document.getElementById(id);

function t(key) {
  const dict = I18N[state.settings.lang] || I18N.pt;
  return Object.prototype.hasOwnProperty.call(dict, key) ? dict[key] : key;
}

function tf(key, vars) {
  let out = t(key);
  for (const key2 of Object.keys(vars)) {
    out = out.split("{" + key2 + "}").join(String(vars[key2]));
  }
  return out;
}

function extOf(path) {
  const m = /\.([^.\\/]+)$/.exec(String(path || ""));
  return m ? m[1].toLowerCase() : "";
}

function baseName(path) {
  return String(path || "").split(/[\\/]/).pop() || "";
}

function samePath(a, b) {
  const norm = (p) => String(p || "").replace(/\//g, "\\").toLowerCase();
  return norm(a) === norm(b);
}

function humanSize(bytes) {
  const n = Number(bytes) || 0;
  if (n < 1024) return n + " B";
  if (n < 1024 * 1024) return (n / 1024).toFixed(1).replace(".", ",") + " KB";
  if (n < 1024 * 1024 * 1024) return (n / (1024 * 1024)).toFixed(1).replace(".", ",") + " MB";
  return (n / (1024 * 1024 * 1024)).toFixed(2).replace(".", ",") + " GB";
}

function escapeReg(s) {
  return String(s).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function toast(message) {
  const el = $("toast");
  el.textContent = message;
  el.classList.remove("hidden");
  if (state.toastTimer) clearTimeout(state.toastTimer);
  state.toastTimer = setTimeout(() => el.classList.add("hidden"), 3200);
}

function setBusy(value) {
  $("stage").classList.toggle("busy", !!value);
}

/* ---------------- i18n / tema / fundo ---------------- */

function applyI18n() {
  document.documentElement.lang = state.settings.lang === "en" ? "en-US" : "pt-BR";
  document.querySelectorAll("[data-i18n]").forEach((el) => {
    el.textContent = t(el.dataset.i18n);
  });
  document.querySelectorAll("[data-i18n-title]").forEach((el) => {
    el.title = t(el.dataset.i18nTitle);
  });
  if (state.seq) updateSeqBar();
  if (state.current) {
    updateInfo();
    updateBanner();
  }
}

function applyTheme() {
  const pref = state.settings.theme;
  const systemDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
  const dark = pref === "dark" || (pref === "auto" && systemDark);
  document.documentElement.dataset.theme = dark ? "dark" : "light";
  invoke("set_window_theme", { dark }).catch(() => {});
}

function applyBackground() {
  const stage = $("stage");
  ["dark", "light", "gray"].forEach((bg) => stage.classList.toggle("bg-" + bg, state.settings.background === bg));
  stage.classList.toggle("checker", !!state.settings.checkerboard);
}

/* ---------------- bootstrap ---------------- */

document.addEventListener("DOMContentLoaded", init);

async function init() {
  bindUi();
  applyTheme();
  applyBackground();
  await loadSettings();
  state.bench = await invoke("bench_mode").catch(() => false);
  applyI18n();
  applyTheme();
  applyBackground();
  fillSettingsForm();
  applyView();
  await loadAppInfo();
  await checkAvifnim();
  setupDropEvents();
  setupKeyboard();

  try {
    const initial = await invoke("initial_arg");
    if (initial) await openPath(initial);
  } catch (err) {
    console.warn("initial_arg", err);
  }
  updateNavButtons();
}

async function loadSettings() {
  try {
    const saved = await invoke("get_settings");
    if (saved && typeof saved === "object") {
      state.settings = Object.assign({}, DEFAULT_SETTINGS, saved, {
        associations: Object.assign({}, DEFAULT_SETTINGS.associations, saved.associations || {}),
      });
    }
  } catch (err) {
    console.warn("get_settings", err);
  }
}

async function loadAppInfo() {
  try {
    state.appInfo = await invoke("app_info");
    const version = $("about-version");
    if (version && state.appInfo) version.textContent = "v" + state.appInfo.version;
    const site = $("about-site");
    if (site && state.appInfo) site.textContent = state.appInfo.siteUrl || "—";
  } catch (err) {
    console.warn("app_info", err);
  }
}

async function checkAvifnim() {
  try {
    state.avifnimInstalled = !!(await invoke("avifnim_installed"));
  } catch (err) {
    state.avifnimInstalled = false;
  }
}

/* ---------------- abrir / exibir ---------------- */

async function openPath(path) {
  if (!path) return;
  hideError();
  stopSequence();
  try {
    const list = await invoke("list_siblings", { path });
    if (Array.isArray(list) && list.length) {
      state.siblings = list;
      state.index = list.findIndex((entry) => samePath(entry.path, path));
      if (state.index < 0) state.index = 0;
    } else {
      state.siblings = [{ path, name: baseName(path) }];
      state.index = 0;
    }
  } catch (err) {
    state.siblings = [{ path, name: baseName(path) }];
    state.index = 0;
  }
  await showCurrent();
  updateNavButtons();
  $("stage").focus();
}

async function showCurrent() {
  const entry = state.siblings[state.index];
  if (!entry) return;
  const startedAt = performance.now();
  const hadImage = !!state.current;
  setBusy(true);
  try {
    const data = await invoke("load_image", { path: entry.path });
    state.current = data;
    state.imageErrorShown = false;
    if (!hadImage || state.settings.reset_view_on_navigate !== false) {
      resetView();
    }
    const img = $("viewer-img");
    img.classList.remove("hidden");
    img.src = data.dataUrl;
    img.alt = data.name;
    $("dropzone").classList.add("hidden");
    $("viewport").classList.remove("hidden");
    updateChrome();
    updateInfo();
    updateBanner();
    startSequenceIfDetected();
    prefetchNeighbors();
    invoke("record_view")
      .then((count) => {
        state.settings.view_count = Number(count) || state.settings.view_count;
        maybeShowPromo(count);
      })
      .catch(() => {});
    setTimeout(updateInfo, 60);
    if (state.bench) {
      await whenImageReady(img);
      requestAnimationFrame(() => {
        requestAnimationFrame(() => {
          invoke("bench_log", {
            line: JSON.stringify({
              event: "first_paint",
              file: data.path,
              format: data.format,
              size: data.size,
              width: img.naturalWidth,
              height: img.naturalHeight,
              ms: Math.round(performance.now() - startedAt),
            }),
          }).catch(() => {});
        });
      });
    }
  } catch (err) {
    showError(err);
  } finally {
    setBusy(false);
  }
}

function whenImageReady(img, timeoutMs = 5000) {
  if (img.complete && img.naturalWidth > 0) return Promise.resolve();
  return new Promise((resolve) => {
    let done = false;
    const finish = () => {
      if (done) return;
      done = true;
      resolve();
    };
    img.addEventListener("load", finish, { once: true });
    img.addEventListener("error", finish, { once: true });
    setTimeout(finish, timeoutMs);
  });
}

function updateChrome() {
  const cur = state.current;
  const title = $("file-title");
  const badge = $("format-badge");
  if (!cur) {
    title.textContent = t("toolbar.noFile");
    badge.classList.add("hidden");
    document.title = "AVIFnim Viewer";
    invoke("set_window_title", { title: "AVIFnim Viewer" }).catch(() => {});
    return;
  }
  title.textContent = cur.name;
  badge.textContent = cur.format;
  badge.className = "badge";
  const ext = extOf(cur.path);
  if (ext === "avif") badge.classList.add("avif");
  else if (ext === "webp") badge.classList.add("webp");
  else if (ext === "gif") badge.classList.add("gif");
  else if (["ppm", "pgm", "pbm", "pnm"].includes(ext)) badge.classList.add("pnm");
  document.title = cur.name + " — AVIFnim Viewer";
  invoke("set_window_title", { title: cur.name + " — AVIFnim Viewer" }).catch(() => {});
}

function updateNavButtons() {
  const has = state.siblings.length > 1;
  $("btn-prev").disabled = !has;
  $("btn-next").disabled = !has;
}

async function navigate(delta) {
  if (state.siblings.length < 2) return;
  stopSequence();
  const n = state.siblings.length;
  state.index = (state.index + delta + n) % n;
  await showCurrent();
  updateNavButtons();
}

async function openDialog() {
  try {
    const path = await invoke("open_file_dialog");
    if (path) await openPath(path);
  } catch (err) {
    showError(err);
  }
}

/* ---------------- sequências (G7) ---------------- */

function startSequenceIfDetected() {
  stopSequence();
  const cur = state.current;
  if (!cur) return;
  const ext = extOf(cur.path);
  if (!SEQ_EXTS.includes(ext)) return;
  const match = cur.name.match(/^(.*?)(\d+)(\.[^.]+)$/);
  if (!match) return;
  const [, prefix, , suffix] = match;
  const re = new RegExp("^" + escapeReg(prefix) + "(\\d+)" + escapeReg(suffix) + "$", "i");
  const frames = state.siblings.filter((entry) => re.test(entry.name));
  if (frames.length < 2) return;
  const start = frames.findIndex((entry) => samePath(entry.path, cur.path));
  if (start < 0) return;
  startSequence(frames.map((entry) => entry.path), start);
}

function startSequence(frames, startIndex) {
  state.seq = {
    frames,
    idx: ((startIndex % frames.length) + frames.length) % frames.length,
    fps: state.settings.seq_fps === 30 ? 30 : 60,
  };
  state.seqBuffer.clear();
  $("seq-bar").classList.remove("hidden");
  updateSeqBar();
  fillSeqBuffer();
  seqTick();
  if (state.bench) {
    state.benchFrames = 0;
    state.benchStart = performance.now();
    const frameCount = frames.length;
    setTimeout(() => {
      if (!state.seq) return;
      const elapsed = performance.now() - state.benchStart;
      const fps = Math.round((state.benchFrames / (elapsed / 1000)) * 10) / 10;
      invoke("bench_log", {
        line: JSON.stringify({
          event: "sequence_fps",
          frames: frameCount,
          played: state.benchFrames,
          seconds: Math.round(elapsed / 100) / 10,
          fps,
        }),
      }).catch(() => {});
    }, 2000);
  }
}

function updateSeqBar() {
  if (!state.seq) return;
  $("seq-info").textContent = tf("seq.bar", { n: state.seq.frames.length, fps: state.seq.fps });
}

function stopSequence() {
  if (state.seqTimer) clearTimeout(state.seqTimer);
  state.seqTimer = null;
  state.seq = null;
  state.seqBuffer.clear();
  state.seqLoads = 0;
  const bar = $("seq-bar");
  if (bar) bar.classList.add("hidden");
}

function seqTick() {
  const seq = state.seq;
  if (!seq) return;
  if (state.bench) state.benchFrames += 1;
  const path = seq.frames[seq.idx];
  const cached = state.seqBuffer.get(path);
  const img = $("viewer-img");
  if (cached) {
    img.src = cached;
  } else {
    loadSeqFrame(path).then((url) => {
      if (url && state.seq === seq) img.src = url;
    });
  }
  seq.idx = (seq.idx + 1) % seq.frames.length;
  fillSeqBuffer();
  const delay = Math.max(12, Math.round(1000 / seq.fps));
  state.seqTimer = setTimeout(seqTick, delay);
}

function fillSeqBuffer() {
  const seq = state.seq;
  if (!seq) return;
  while (state.seqLoads < 3) {
    let missing = null;
    for (let k = 0; k < PRELOAD_AHEAD; k++) {
      const path = seq.frames[(seq.idx + k) % seq.frames.length];
      if (!state.seqBuffer.has(path)) {
        missing = path;
        break;
      }
    }
    if (!missing) break;
    state.seqLoads += 1;
    loadSeqFrame(missing).finally(() => {
      state.seqLoads = Math.max(0, state.seqLoads - 1);
    });
  }
  if (state.seqBuffer.size > SEQ_BUFFER_MAX) {
    const oldest = state.seqBuffer.keys().next().value;
    state.seqBuffer.delete(oldest);
  }
}

function loadSeqFrame(path) {
  const cached = state.seqBuffer.get(path);
  if (cached) return Promise.resolve(cached);
  return invoke("load_image", { path })
    .then((data) => {
      state.seqBuffer.set(path, data.dataUrl);
      return data.dataUrl;
    })
    .catch(() => null);
}

/* ---------------- prefetch ---------------- */

function prefetchNeighbors() {
  const paths = [];
  const n = Math.max(0, Math.min(10, Number(state.settings.prefetch_n) || 0));
  if (n > 0) {
    for (let k = 1; k <= n; k++) {
      const prev = state.siblings[state.index - k];
      const next = state.siblings[state.index + k];
      if (prev) paths.push(prev.path);
      if (next) paths.push(next.path);
    }
  }
  if (state.seq) {
    for (let k = 1; k <= 8; k++) {
      paths.push(state.seq.frames[(state.seq.idx + k) % state.seq.frames.length]);
    }
  }
  if (paths.length) invoke("prefetch", { paths }).catch(() => {});
}

/* ---------------- zoom / pan ---------------- */

function applyView() {
  const img = $("viewer-img");
  const viewport = $("viewport");
  const actual = !state.fitMode;
  viewport.classList.toggle("actual", actual);
  if (actual) {
    img.style.transform = "translate(" + state.panX + "px, " + state.panY + "px) scale(" + state.zoom + ")";
  } else {
    img.style.transform = "";
  }
  viewport.classList.toggle("can-pan", actual && state.zoom > 1);
  $("zoom-label").textContent = state.fitMode ? t("toolbar.fit") : Math.round(state.zoom * 100) + "%";
}

function resetView() {
  state.fitMode = true;
  state.zoom = 1;
  state.panX = 0;
  state.panY = 0;
  applyView();
}

function setFit() {
  resetView();
}

function setActual() {
  state.fitMode = false;
  state.zoom = 1;
  state.panX = 0;
  state.panY = 0;
  applyView();
}

function zoomBy(factor) {
  if (state.fitMode) {
    state.fitMode = false;
    state.zoom = 1;
  }
  state.zoom = Math.min(12, Math.max(0.1, state.zoom * factor));
  applyView();
}

/* ---------------- info ---------------- */

function updateInfo() {
  const panel = $("info-panel");
  const cur = state.current;
  if (!cur) return;
  $("info-name").textContent = cur.name || "—";
  $("info-format").textContent = cur.format || "—";
  const img = $("viewer-img");
  const w = img.naturalWidth || cur.width || 0;
  const h = img.naturalHeight || cur.height || 0;
  $("info-dims").textContent = w && h ? w + " × " + h + " px" : "—";
  $("info-size").textContent = humanSize(cur.size);
  let seqText = t("info.seqNone");
  if (state.seq) seqText = tf("seq.bar", { n: state.seq.frames.length, fps: state.seq.fps });
  else if (cur.animated) seqText = cur.format;
  $("info-seq").textContent = seqText;
  $("info-path").textContent = cur.path || "—";
  $("info-path").title = cur.path || "";
}

/* ---------------- banner / promo ---------------- */

function updateBanner() {
  const banner = $("banner");
  const cur = state.current;
  if (!cur || state.bannerDismissedSession) {
    banner.classList.add("hidden");
    return;
  }
  const ext = extOf(cur.path);
  if (!["gif", "webp"].includes(ext)) {
    banner.classList.add("hidden");
    return;
  }
  const estimated = Math.max(1024, Math.round((cur.size || 0) / 7));
  $("banner-text").textContent = tf("banner.text", {
    format: cur.format,
    size: humanSize(cur.size),
    est: humanSize(estimated),
  });
  banner.classList.remove("hidden");
}

function maybeShowPromo(viewCount) {
  if (!state.settings) return;
  if (state.avifnimInstalled) return;
  if (state.settings.promo_seen) return;
  if (Number(viewCount) < 5) return;
  state.settings.promo_seen = true;
  persistSettings({ promo_seen: true });
  openModal("promo-modal");
}

function openAvifnim() {
  const url = state.appInfo && state.appInfo.avifnimUrl;
  if (!url) {
    toast(t("toast.linkPending"));
    return;
  }
  invoke("open_url", { url }).catch((err) => toast(String(err)));
}

/* ---------------- modais ---------------- */

function openModal(id) {
  const modal = $(id);
  if (!modal) return;
  state.modalOpen = id;
  modal.classList.remove("hidden");
  if (id === "settings-modal") {
    fillSettingsForm();
    loadAssociationStates();
  }
  if (id === "about-modal") fillAbout();
}

function closeModal(id) {
  const target = id || state.modalOpen;
  if (!target) return;
  const modal = $(target);
  if (modal) modal.classList.add("hidden");
  if (state.modalOpen === target) state.modalOpen = null;
}

function fillAbout() {
  const info = state.appInfo;
  if (!info) return;
  $("about-version").textContent = "v" + info.version;
  $("about-site").textContent = info.siteUrl || "—";
  const cta = $("about-avifnim-cta");
  cta.disabled = !info.avifnimUrl;
  cta.title = info.avifnimUrl ? "" : t("toast.linkPending");
}

/* ---------------- configurações ---------------- */

function fillSettingsForm() {
  const s = state.settings;
  $("set-theme").value = s.theme;
  $("set-lang").value = s.lang;
  $("set-background").value = s.background;
  $("set-checker").checked = !!s.checkerboard;
  $("set-tips").checked = !!s.show_tips;
  $("set-reset-view").checked = s.reset_view_on_navigate !== false;
  $("set-prefetch-n").value = String(s.prefetch_n);
  $("set-prefetch-mb").value = String(s.prefetch_mb);
  $("set-seq-fps").value = String(s.seq_fps);
  $("assoc-avif").checked = !!s.associations.avif;
  $("assoc-webp").checked = !!s.associations.webp;
  $("assoc-gif").checked = !!s.associations.gif;
  $("assoc-pnm").checked = !!s.associations.pnm;
}

async function loadAssociationStates() {
  try {
    const states = await invoke("get_association_states");
    if (states && typeof states === "object") {
      if (typeof states.avif === "boolean") $("assoc-avif").checked = states.avif;
      if (typeof states.webp === "boolean") $("assoc-webp").checked = states.webp;
      if (typeof states.gif === "boolean") $("assoc-gif").checked = states.gif;
      if (typeof states.pnm === "boolean") $("assoc-pnm").checked = states.pnm;
    }
  } catch (err) {
    console.warn("get_association_states", err);
  }
}

async function persistSettings(patch) {
  state.settings = Object.assign({}, state.settings, patch || {});
  applyTheme();
  applyBackground();
  applyI18n();
  try {
    const saved = await invoke("save_settings", { settings: state.settings });
    if (saved && typeof saved === "object") state.settings = saved;
    fillSettingsForm();
    applyTheme();
    applyBackground();
    $("settings-saved").textContent = t("settings.saved");
  } catch (err) {
    toast(t("toast.saveFail"));
    console.warn(err);
  }
}

async function toggleAssociation(group, checkbox) {
  const enabled = !!checkbox.checked;
  try {
    const applied = await invoke("set_association", { group, enabled });
    state.settings.associations[group] = !!applied;
    toast(tf("toast.assoc", { fmt: group.toUpperCase() }));
    persistSettings({});
  } catch (err) {
    checkbox.checked = !enabled;
    toast(String(err));
  }
}

async function resetAllSettings() {
  state.settings = JSON.parse(JSON.stringify(DEFAULT_SETTINGS));
  try {
    await invoke("clear_cache");
  } catch (err) {
    console.warn(err);
  }
  await persistSettings({});
  toast(t("toast.reset"));
}

/* ---------------- erros ---------------- */

function showError(err) {
  const raw = typeof err === "string" ? err : (err && err.message) || String(err || "");
  const text = raw.trim().slice(0, 400) || t("error.generic");
  $("error-text").textContent = text;
  $("error-bar").classList.remove("hidden");
  if (!state.current) {
    $("dropzone").classList.remove("hidden");
  }
}

function hideError() {
  $("error-bar").classList.add("hidden");
}

/* ---------------- eventos ---------------- */

function setupDropEvents() {
  const stage = $("stage");
  ["dragenter", "dragover"].forEach((eventName) => {
    stage.addEventListener(eventName, (event) => {
      event.preventDefault();
      if (event.dataTransfer) event.dataTransfer.dropEffect = "copy";
      $("dropzone").classList.add("dragover");
    });
  });
  ["dragleave", "drop"].forEach((eventName) => {
    stage.addEventListener(eventName, (event) => {
      event.preventDefault();
      $("dropzone").classList.remove("dragover");
    });
  });
  stage.addEventListener("drop", (event) => {
    const file = event.dataTransfer && event.dataTransfer.files && event.dataTransfer.files[0];
    if (file && file.path) openPath(file.path);
  });

  try {
    const webview = getCurrentWebview && getCurrentWebview();
    if (webview && webview.onDragDropEvent) {
      webview.onDragDropEvent((event) => {
        const payload = event && event.payload;
        if (!payload) return;
        if (payload.type === "drop" && payload.paths && payload.paths.length) {
          openPath(payload.paths[0]);
        } else if (payload.type === "enter" || payload.type === "over") {
          $("dropzone").classList.add("dragover");
        } else if (payload.type === "leave") {
          $("dropzone").classList.remove("dragover");
        }
      });
    }
  } catch (err) {
    console.warn("drag-drop", err);
  }
}

function setupKeyboard() {
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      if (state.modalOpen) {
        closeModal(state.modalOpen);
        event.preventDefault();
      } else if (document.fullscreenElement) {
        document.exitFullscreen().catch(() => {});
        event.preventDefault();
      }
      return;
    }
    const mod = event.ctrlKey || event.metaKey;
    if (mod && event.key.toLowerCase() === "o") {
      event.preventDefault();
      openDialog();
      return;
    }
    if (state.modalOpen) return;
    if (event.ctrlKey || event.metaKey || event.altKey) return;

    switch (event.key) {
      case "ArrowLeft":
        event.preventDefault();
        navigate(-1);
        break;
      case "ArrowRight":
        event.preventDefault();
        navigate(1);
        break;
      case "+":
      case "=":
        event.preventDefault();
        zoomBy(1.25);
        break;
      case "-":
      case "_":
        event.preventDefault();
        zoomBy(0.8);
        break;
      case "0":
        event.preventDefault();
        setFit();
        break;
      case "f":
      case "F":
        event.preventDefault();
        toggleFullscreen();
        break;
      case "i":
      case "I":
        event.preventDefault();
        toggleInfo();
        break;
      case "s":
      case "S":
        event.preventDefault();
        openModal("settings-modal");
        break;
      case "F1":
        event.preventDefault();
        openModal("about-modal");
        break;
      default:
        break;
    }
  });
}

function toggleFullscreen() {
  if (document.fullscreenElement) {
    document.exitFullscreen().catch(() => {});
  } else {
    document.documentElement.requestFullscreen().catch(() => {});
  }
}

function toggleInfo() {
  const panel = $("info-panel");
  if (panel.classList.contains("hidden")) {
    updateInfo();
    panel.classList.remove("hidden");
  } else {
    panel.classList.add("hidden");
  }
}

function setupPointerPan() {
  const viewport = $("viewport");
  viewport.addEventListener("pointerdown", (event) => {
    if (state.fitMode || state.zoom <= 1) return;
    state.dragging = true;
    state.dragStart = { x: event.clientX - state.panX, y: event.clientY - state.panY };
    viewport.classList.add("panning");
    viewport.setPointerCapture(event.pointerId);
  });
  viewport.addEventListener("pointermove", (event) => {
    if (!state.dragging || !state.dragStart) return;
    state.panX = event.clientX - state.dragStart.x;
    state.panY = event.clientY - state.dragStart.y;
    applyView();
  });
  const stop = () => {
    state.dragging = false;
    state.dragStart = null;
    viewport.classList.remove("panning");
  };
  viewport.addEventListener("pointerup", stop);
  viewport.addEventListener("pointercancel", stop);
  viewport.addEventListener(
    "wheel",
    (event) => {
      if (!state.current) return;
      event.preventDefault();
      zoomBy(event.deltaY < 0 ? 1.12 : 1 / 1.12);
    },
    { passive: false },
  );
}

function bindUi() {
  $("btn-open").addEventListener("click", openDialog);
  $("btn-open-empty").addEventListener("click", openDialog);
  $("dropzone").addEventListener("click", (event) => {
    if (event.target.id !== "btn-open-empty") openDialog();
  });
  $("btn-prev").addEventListener("click", () => navigate(-1));
  $("btn-next").addEventListener("click", () => navigate(1));
  $("btn-zoom-out").addEventListener("click", () => zoomBy(0.8));
  $("btn-zoom-in").addEventListener("click", () => zoomBy(1.25));
  $("btn-fit").addEventListener("click", setFit);
  $("btn-100").addEventListener("click", setActual);
  $("btn-fullscreen").addEventListener("click", toggleFullscreen);
  $("btn-info").addEventListener("click", toggleInfo);
  $("btn-info-close").addEventListener("click", () => $("info-panel").classList.add("hidden"));
  $("btn-bg").addEventListener("click", () => {
    const i = BG_ORDER.indexOf(state.settings.background);
    persistSettings({ background: BG_ORDER[(i + 1) % BG_ORDER.length] });
  });
  $("btn-checker").addEventListener("click", () => persistSettings({ checkerboard: !state.settings.checkerboard }));
  $("btn-settings").addEventListener("click", () => openModal("settings-modal"));
  $("btn-about").addEventListener("click", () => openModal("about-modal"));
  $("btn-settings-close").addEventListener("click", () => closeModal("settings-modal"));
  $("btn-about-close").addEventListener("click", () => closeModal("about-modal"));
  $("promo-dismiss").addEventListener("click", () => closeModal("promo-modal"));
  $("promo-cta").addEventListener("click", openAvifnim);
  $("about-avifnim-cta").addEventListener("click", openAvifnim);
  $("promo-art").addEventListener("click", openAvifnim);

  $("banner-close").addEventListener("click", () => {
    state.bannerDismissedSession = true;
    $("banner").classList.add("hidden");
  });
  $("banner-cta").addEventListener("click", openAvifnim);

  $("btn-error-close").addEventListener("click", hideError);
  $("btn-error-open").addEventListener("click", openDialog);

  document.querySelectorAll("[data-close]").forEach((button) => {
    button.addEventListener("click", () => closeModal(button.dataset.close));
  });

  document.querySelectorAll(".modal").forEach((modal) => {
    modal.addEventListener("mousedown", (event) => {
      if (event.target === modal) closeModal(modal.id);
    });
  });

  $("set-theme").addEventListener("change", (event) => persistSettings({ theme: event.target.value }));
  $("set-lang").addEventListener("change", (event) => persistSettings({ lang: event.target.value }));
  $("set-background").addEventListener("change", (event) => persistSettings({ background: event.target.value }));
  $("set-checker").addEventListener("change", (event) => persistSettings({ checkerboard: event.target.checked }));
  $("set-tips").addEventListener("change", (event) => persistSettings({ show_tips: event.target.checked }));
  $("set-reset-view").addEventListener("change", (event) =>
    persistSettings({ reset_view_on_navigate: event.target.checked }),
  );
  $("set-prefetch-n").addEventListener("change", (event) => persistSettings({ prefetch_n: Number(event.target.value) }));
  $("set-prefetch-mb").addEventListener("change", (event) => persistSettings({ prefetch_mb: Number(event.target.value) }));
  $("set-seq-fps").addEventListener("change", (event) => {
    const fps = Number(event.target.value) === 30 ? 30 : 60;
    persistSettings({ seq_fps: fps }).then(() => {
      if (state.seq) {
        state.seq.fps = fps;
        updateSeqBar();
      }
    });
  });

  $("assoc-avif").addEventListener("change", (event) => toggleAssociation("avif", event.target));
  $("assoc-webp").addEventListener("change", (event) => toggleAssociation("webp", event.target));
  $("assoc-gif").addEventListener("change", (event) => toggleAssociation("gif", event.target));
  $("assoc-pnm").addEventListener("change", (event) => toggleAssociation("pnm", event.target));
  $("btn-default-apps").addEventListener("click", () => {
    invoke("open_default_apps_settings").catch((err) => toast(String(err)));
  });
  $("btn-reset-promo").addEventListener("click", () => {
    persistSettings({ promo_seen: false });
    toast(t("toast.promoReset"));
  });
  $("btn-reset-all").addEventListener("click", resetAllSettings);

  $("about-repo").addEventListener("click", (event) => {
    event.preventDefault();
    const url = state.appInfo && state.appInfo.repoUrl;
    if (url) invoke("open_url", { url }).catch((err) => toast(String(err)));
  });
  $("about-repo-link").addEventListener("click", (event) => {
    event.preventDefault();
    const url = state.appInfo && state.appInfo.repoUrl;
    if (url) invoke("open_url", { url }).catch((err) => toast(String(err)));
  });
  $("about-email").addEventListener("click", (event) => {
    event.preventDefault();
    const email = (state.appInfo && state.appInfo.email) || "mayerdesigner@gmail.com";
    invoke("open_url", { url: "mailto:" + email }).catch(() => {});
  });

  setupPointerPan();

  // Arquivo "abriu" mas o webview nao conseguiu decodificar (corrompido/nao suportado).
  $("viewer-img").addEventListener("error", () => {
    if (!state.current || state.imageErrorShown) return;
    state.imageErrorShown = true;
    $("viewer-img").classList.add("hidden");
    showError(t("error.decode"));
  });

  window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
    if (state.settings.theme === "auto") applyTheme();
  });

  window.addEventListener("resize", () => applyView());

  listen("open-path", (event) => {
    if (event && event.payload) openPath(event.payload);
  }).catch((err) => console.warn("listen open-path", err));
}
