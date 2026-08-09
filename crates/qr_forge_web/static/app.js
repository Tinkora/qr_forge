import init, {
  wasm_generate_barcode_png,
  wasm_generate_barcode_svg,
  wasm_generate_qr_png,
  wasm_generate_qr_png_with_logo,
  wasm_generate_qr_svg,
  wasm_vcard_payload,
  wasm_wifi_payload,
} from "./pkg/qr_forge_web.js";

const MAX_LOGO_BYTES = 2 * 1024 * 1024;
const AUTO_GENERATE_DELAY = 240;

const messages = {
  en: {
    skipToWorkspace: "Skip to workspace",
    localOnly: "Local only",
    switchLanguage: "Switch to Chinese",
    generatorEyebrow: "Generator",
    controlsTitle: "Create code",
    modeLegend: "Type",
    modeText: "URL / Text",
    modeWifi: "Wi-Fi",
    modeVcard: "vCard",
    modePhone: "Phone",
    modeEmail: "Email",
    modeBarcode: "Barcode",
    contentLabel: "Content",
    contentPlaceholder: "Enter a URL or text",
    ssidLabel: "Network name",
    ssidPlaceholder: "Office Wi-Fi",
    passwordLabel: "Password",
    passwordPlaceholder: "Network password",
    showPassword: "Show password",
    hidePassword: "Hide password",
    encryptionLabel: "Security",
    openNetwork: "Open network",
    nameLabel: "Name *",
    namePlaceholder: "Ada Lovelace",
    phoneLabel: "Phone",
    emailLabel: "Email",
    organizationLabel: "Organization",
    organizationPlaceholder: "Tinkora",
    phoneNumberLabel: "Phone number",
    emailAddressLabel: "Email address",
    barcodeTypeLabel: "Barcode type",
    eanOption: "EAN-13 (12 digits)",
    barcodeContentLabel: "Barcode data",
    barcodeContentPlaceholder: "Printable ASCII or 12 digits",
    heightLabel: "Height (px)",
    moduleWidthLabel: "Module (px)",
    appearanceLegend: "Appearance",
    errorCorrectionLabel: "Correction",
    moduleSizeLabel: "Module",
    marginLabel: "Margin",
    foregroundLabel: "Foreground",
    backgroundLabel: "Background",
    logoLegend: "Logo",
    choosePng: "Choose PNG",
    noFile: "No file",
    removeLogo: "Remove",
    logoRatioLabel: "Size",
    generateButton: "Generate",
    previewEyebrow: "Output",
    previewTitle: "Preview",
    noPreview: "No preview",
    downloadSvg: "SVG",
    downloadPng: "PNG",
    initializing: "Loading WebAssembly...",
    generating: "Generating...",
    qrReady: "QR ready · {width} × {height} px",
    barcodeReady: "Barcode ready · {width} × {height} px",
    logoLoaded: "{name} loaded · correction set to H",
    logoRemoved: "Logo removed",
    downloaded: "{format} downloaded",
    missingContent: "Enter content to generate a QR code",
    missingWifi: "Enter a network name",
    missingName: "Enter a contact name",
    missingPhone: "Enter a phone number",
    missingEmail: "Enter an email address",
    invalidEmail: "Enter a valid email address",
    missingBarcode: "Enter barcode data",
    invalidNumber: "{field} is outside the allowed range",
    invalidLogoType: "Choose a PNG file",
    invalidLogoSize: "Logo files must be 2 MB or smaller",
    unexpectedError: "Unable to generate: {message}",
    invalidData: "The input cannot be encoded",
    invalidEan: "EAN-13 requires exactly 12 digits",
    invalidCode128: "Code 128 supports printable ASCII only",
    code128TooLong: "Code 128 input cannot exceed 128 bytes",
    invalidWifiSsid: "Network names must contain 1 to 32 bytes",
    invalidWifiPassword: "Wi-Fi passwords cannot exceed 64 bytes",
    invalidWifiEncryption: "Choose WPA, WEP, or an open network",
    invalidVcardName: "Enter a contact name",
    invalidLogoRatio: "Logo size must be between 5% and 30%",
    generatedQrPreview: "Generated QR code preview",
    generatedBarcodePreview: "Generated barcode preview",
  },
  zh: {
    skipToWorkspace: "跳到工作区",
    localOnly: "仅本地处理",
    switchLanguage: "切换到英文",
    generatorEyebrow: "生成器",
    controlsTitle: "创建编码",
    modeLegend: "类型",
    modeText: "网址 / 文本",
    modeWifi: "Wi-Fi",
    modeVcard: "联系人",
    modePhone: "电话",
    modeEmail: "邮件",
    modeBarcode: "条形码",
    contentLabel: "内容",
    contentPlaceholder: "输入网址或文本",
    ssidLabel: "网络名称",
    ssidPlaceholder: "办公室 Wi-Fi",
    passwordLabel: "密码",
    passwordPlaceholder: "网络密码",
    showPassword: "显示密码",
    hidePassword: "隐藏密码",
    encryptionLabel: "安全类型",
    openNetwork: "开放网络",
    nameLabel: "姓名 *",
    namePlaceholder: "姓名",
    phoneLabel: "电话",
    emailLabel: "邮箱",
    organizationLabel: "组织",
    organizationPlaceholder: "Tinkora",
    phoneNumberLabel: "电话号码",
    emailAddressLabel: "邮箱地址",
    barcodeTypeLabel: "条形码类型",
    eanOption: "EAN-13（12 位数字）",
    barcodeContentLabel: "条形码数据",
    barcodeContentPlaceholder: "可打印 ASCII 或 12 位数字",
    heightLabel: "高度（像素）",
    moduleWidthLabel: "模块（像素）",
    appearanceLegend: "外观",
    errorCorrectionLabel: "纠错",
    moduleSizeLabel: "模块",
    marginLabel: "边距",
    foregroundLabel: "前景色",
    backgroundLabel: "背景色",
    logoLegend: "Logo",
    choosePng: "选择 PNG",
    noFile: "未选择文件",
    removeLogo: "移除",
    logoRatioLabel: "尺寸",
    generateButton: "生成",
    previewEyebrow: "输出",
    previewTitle: "预览",
    noPreview: "暂无预览",
    downloadSvg: "SVG",
    downloadPng: "PNG",
    initializing: "正在加载 WebAssembly...",
    generating: "正在生成...",
    qrReady: "二维码已生成 · {width} × {height} 像素",
    barcodeReady: "条形码已生成 · {width} × {height} 像素",
    logoLoaded: "已加载 {name} · 纠错级别设为 H",
    logoRemoved: "已移除 Logo",
    downloaded: "已下载 {format}",
    missingContent: "请输入要生成二维码的内容",
    missingWifi: "请输入网络名称",
    missingName: "请输入联系人姓名",
    missingPhone: "请输入电话号码",
    missingEmail: "请输入邮箱地址",
    invalidEmail: "请输入有效的邮箱地址",
    missingBarcode: "请输入条形码数据",
    invalidNumber: "{field} 超出允许范围",
    invalidLogoType: "请选择 PNG 文件",
    invalidLogoSize: "Logo 文件不得超过 2 MB",
    unexpectedError: "生成失败：{message}",
    invalidData: "当前输入无法编码",
    invalidEan: "EAN-13 必须为 12 位数字",
    invalidCode128: "Code 128 仅支持可打印 ASCII 字符",
    code128TooLong: "Code 128 输入不得超过 128 字节",
    invalidWifiSsid: "网络名称长度必须为 1 到 32 字节",
    invalidWifiPassword: "Wi-Fi 密码不得超过 64 字节",
    invalidWifiEncryption: "请选择 WPA、WEP 或开放网络",
    invalidVcardName: "请输入联系人姓名",
    invalidLogoRatio: "Logo 尺寸必须在 5% 到 30% 之间",
    generatedQrPreview: "已生成的二维码预览",
    generatedBarcodePreview: "已生成的条形码预览",
  },
};

const errorMessageKeys = {
  EMPTY_DATA: "missingContent",
  INVALID_DATA: "invalidData",
  INVALID_EAN13: "invalidEan",
  INVALID_EAN13_CHARS: "invalidEan",
  INVALID_CODE128: "invalidCode128",
  CODE128_TOO_LONG: "code128TooLong",
  INVALID_WIFI_SSID: "invalidWifiSsid",
  INVALID_WIFI_PASSWORD: "invalidWifiPassword",
  INVALID_WIFI_ENCRYPTION: "invalidWifiEncryption",
  MISSING_VCARD_NAME: "invalidVcardName",
  INVALID_LOGO_RATIO: "invalidLogoRatio",
};

const $ = (id) => document.getElementById(id);
const form = $("generator-form");
const tabBar = $("tab-bar");
const tabs = [...tabBar.querySelectorAll("[role='tab']")];
const statusElement = $("status");
const previewStage = $("preview-stage");
const emptyState = $("empty-state");
const qrContainer = $("qr-container");
const qrDisplay = $("qr-display");
const barcodePreview = $("barcode-preview");
const barcodeDisplay = $("barcode-display");
const barcodeTextDisplay = $("barcode-text-display");
const dimensions = $("dimensions");
const generateButton = $("btn-generate");
const downloadSvgButton = $("btn-download-svg");
const downloadPngButton = $("btn-download-png");
const languageToggle = $("language-toggle");
const languageToggleLabel = $("language-toggle-label");
const passwordToggle = $("password-toggle");
const passwordToggleIcon = $("password-toggle-icon");
const logoInput = $("logo-file");
const logoFileName = $("logo-file-name");
const clearLogoButton = $("clear-logo");

let language = "en";
let currentMode = "qr-url";
let initialized = false;
let logoData = null;
let logoName = null;
let autoGenerateTimer = null;
let previewObjectUrl = null;
let lastOutput = null;
let statusState = { key: "initializing", type: "neutral", variables: {} };

class UiError extends Error {
  constructor(key, variables = {}) {
    super(key);
    this.key = key;
    this.variables = variables;
  }
}

function translate(key, variables = {}) {
  const template = messages[language][key] ?? messages.en[key] ?? key;
  return Object.entries(variables).reduce(
    (text, [name, value]) => text.replaceAll(`{${name}}`, String(value)),
    template,
  );
}

function renderStatus() {
  statusElement.textContent = translate(statusState.key, statusState.variables);
  statusElement.className = "status-message";
  if (statusState.type !== "neutral") {
    statusElement.classList.add(`is-${statusState.type}`);
  }
}

function setStatus(key, type = "neutral", variables = {}) {
  statusState = { key, type, variables };
  renderStatus();
}

function updatePasswordToggleLabel() {
  const showing = $("wifi-password").type === "text";
  const label = translate(showing ? "hidePassword" : "showPassword");
  passwordToggle.setAttribute("aria-label", label);
  passwordToggle.title = label;
  passwordToggle.setAttribute("aria-pressed", String(showing));
  passwordToggleIcon.setAttribute("href", showing ? "#icon-eye-off" : "#icon-eye");
}

function updatePreviewAccessibleName() {
  const selector = lastOutput?.kind === "barcode" ? "#barcode-display svg" : "#qr-display svg, #qr-display img";
  const preview = document.querySelector(selector);
  if (!preview) return;
  const label = translate(lastOutput?.kind === "barcode" ? "generatedBarcodePreview" : "generatedQrPreview");
  preview.setAttribute("role", "img");
  preview.setAttribute("aria-label", label);
  if (preview instanceof HTMLImageElement) preview.alt = label;
}

function applyLanguage() {
  document.documentElement.lang = language === "en" ? "en" : "zh-CN";
  document.querySelectorAll("[data-i18n]").forEach((element) => {
    element.textContent = translate(element.dataset.i18n);
  });
  document.querySelectorAll("[data-i18n-placeholder]").forEach((element) => {
    element.placeholder = translate(element.dataset.i18nPlaceholder);
  });
  document.querySelectorAll("[data-i18n-aria-label]").forEach((element) => {
    element.setAttribute("aria-label", translate(element.dataset.i18nAriaLabel));
  });
  languageToggleLabel.textContent = language === "en" ? "中文" : "EN";
  languageToggle.setAttribute("aria-label", translate("switchLanguage"));
  languageToggle.title = translate("switchLanguage");
  if (!logoName) logoFileName.textContent = translate("noFile");
  updatePasswordToggleLabel();
  updatePreviewAccessibleName();
  renderStatus();
}

function setBusy(busy) {
  previewStage.setAttribute("aria-busy", String(busy));
  generateButton.disabled = busy;
  generateButton.classList.toggle("is-busy", busy);
}

function clearPreview() {
  if (previewObjectUrl) {
    URL.revokeObjectURL(previewObjectUrl);
    previewObjectUrl = null;
  }
  qrDisplay.replaceChildren();
  barcodeDisplay.replaceChildren();
  barcodeTextDisplay.textContent = "";
  qrContainer.hidden = true;
  barcodePreview.hidden = true;
  emptyState.hidden = false;
  dimensions.textContent = "—";
  downloadSvgButton.disabled = true;
  downloadPngButton.disabled = true;
  lastOutput = null;
}

function renderSvg(target, svgText, accessibleNameKey) {
  target.innerHTML = svgText;
  const svg = target.querySelector("svg");
  if (svg) {
    svg.setAttribute("role", "img");
    svg.setAttribute("aria-label", translate(accessibleNameKey));
  }
}

function renderPng(target, pngBytes) {
  if (previewObjectUrl) URL.revokeObjectURL(previewObjectUrl);
  previewObjectUrl = URL.createObjectURL(new Blob([pngBytes], { type: "image/png" }));
  const image = document.createElement("img");
  image.src = previewObjectUrl;
  image.alt = translate("generatedQrPreview");
  image.setAttribute("role", "img");
  target.replaceChildren(image);
}

function readInteger(id, minimum, maximum, fieldKey) {
  const value = Number.parseInt($(id).value, 10);
  if (!Number.isInteger(value) || value < minimum || value > maximum) {
    throw new UiError("invalidNumber", { field: translate(fieldKey) });
  }
  return value;
}

function readQrPayload() {
  switch (currentMode) {
    case "qr-url": {
      const value = $("qr-url-input").value;
      if (value.length === 0) throw new UiError("missingContent");
      return value;
    }
    case "qr-wifi": {
      const ssid = $("wifi-ssid").value;
      if (ssid.length === 0) throw new UiError("missingWifi");
      return wasm_wifi_payload(ssid, $("wifi-password").value, $("wifi-encryption").value);
    }
    case "qr-vcard": {
      const name = $("vcard-name").value;
      if (name.trim().length === 0) throw new UiError("missingName");
      const emailInput = $("vcard-email");
      if (emailInput.value && !emailInput.checkValidity()) throw new UiError("invalidEmail");
      return wasm_vcard_payload(name, $("vcard-phone").value, emailInput.value, $("vcard-org").value);
    }
    case "qr-phone": {
      const phone = $("phone-number").value.trim();
      if (!phone) throw new UiError("missingPhone");
      return `tel:${phone}`;
    }
    case "qr-email": {
      const emailInput = $("email-address");
      const email = emailInput.value.trim();
      if (!email) throw new UiError("missingEmail");
      if (!emailInput.checkValidity()) throw new UiError("invalidEmail");
      return `mailto:${email}`;
    }
    default:
      throw new UiError("missingContent");
  }
}

function readQrRequest() {
  return {
    kind: "qr",
    payload: readQrPayload(),
    ecLevel: $("ec-level").value,
    moduleSize: readInteger("module-size", 1, 32, "moduleSizeLabel"),
    margin: readInteger("margin", 0, 16, "marginLabel"),
    foreground: $("fg-color").value,
    background: $("bg-color").value,
    logoData,
    logoRatio: readInteger("logo-ratio", 5, 30, "logoRatioLabel") / 100,
  };
}

function readBarcodeRequest() {
  const data = $("barcode-input").value;
  if (data.length === 0) throw new UiError("missingBarcode");
  return {
    kind: "barcode",
    data,
    barcodeType: $("barcode-type").value,
    height: readInteger("barcode-height", 20, 2000, "heightLabel"),
    moduleWidth: readInteger("barcode-module-width", 1, 16, "moduleWidthLabel"),
    foreground: $("fg-color").value,
    background: $("bg-color").value,
  };
}

function reportError(error) {
  if (error instanceof UiError) {
    setStatus(error.key, "error", error.variables);
    return;
  }
  const key = errorMessageKeys[error?.code];
  if (key) {
    setStatus(key, "error");
    return;
  }
  const message = error?.message || String(error);
  setStatus("unexpectedError", "error", { message });
}

async function generate() {
  if (!initialized) return;
  window.clearTimeout(autoGenerateTimer);
  setBusy(true);
  setStatus("generating");
  await new Promise((resolve) => requestAnimationFrame(resolve));

  try {
    if (currentMode === "barcode") {
      const request = readBarcodeRequest();
      const result = wasm_generate_barcode_svg(
        request.data,
        request.barcodeType,
        request.height,
        request.moduleWidth,
        request.foreground,
        request.background,
      );
      renderSvg(barcodeDisplay, result.svg, "generatedBarcodePreview");
      barcodeTextDisplay.textContent = request.data;
      qrContainer.hidden = true;
      barcodePreview.hidden = false;
      emptyState.hidden = true;
      dimensions.textContent = `${result.width} × ${request.height} px`;
      lastOutput = { ...request, svg: result.svg, width: result.width };
      downloadSvgButton.disabled = false;
      downloadPngButton.disabled = false;
      setStatus("barcodeReady", "success", { width: result.width, height: request.height });
    } else {
      const request = readQrRequest();
      const result = wasm_generate_qr_svg(
        request.payload,
        request.ecLevel,
        request.moduleSize,
        request.margin,
        request.foreground,
        request.background,
      );
      let pngBytes = null;
      if (request.logoData) {
        pngBytes = wasm_generate_qr_png_with_logo(
          request.payload,
          request.ecLevel,
          request.moduleSize,
          request.margin,
          request.foreground,
          request.background,
          request.logoData,
          request.logoRatio,
        );
        renderPng(qrDisplay, pngBytes);
      } else {
        if (previewObjectUrl) {
          URL.revokeObjectURL(previewObjectUrl);
          previewObjectUrl = null;
        }
        renderSvg(qrDisplay, result.svg, "generatedQrPreview");
      }
      barcodePreview.hidden = true;
      qrContainer.hidden = false;
      emptyState.hidden = true;
      dimensions.textContent = `${result.size} × ${result.size} px`;
      lastOutput = { ...request, svg: request.logoData ? null : result.svg, pngBytes, size: result.size };
      downloadSvgButton.disabled = Boolean(request.logoData);
      downloadPngButton.disabled = false;
      setStatus("qrReady", "success", { width: result.size, height: result.size });
    }
  } catch (error) {
    clearPreview();
    reportError(error);
  } finally {
    setBusy(false);
  }
}

function hasRequiredInput() {
  switch (currentMode) {
    case "qr-url": return $("qr-url-input").value.length > 0;
    case "qr-wifi": return $("wifi-ssid").value.length > 0;
    case "qr-vcard": return $("vcard-name").value.trim().length > 0;
    case "qr-phone": return $("phone-number").value.trim().length > 0;
    case "qr-email": return $("email-address").value.trim().length > 0;
    case "barcode": return $("barcode-input").value.length > 0;
    default: return false;
  }
}

function scheduleGenerate(delay = AUTO_GENERATE_DELAY) {
  if (!initialized) return;
  window.clearTimeout(autoGenerateTimer);
  if (!hasRequiredInput()) {
    clearPreview();
    setStatus("noPreview");
    return;
  }
  autoGenerateTimer = window.setTimeout(generate, delay);
}

function updateBarcodeInputContract() {
  const input = $("barcode-input");
  const ean13 = $("barcode-type").value === "ean13";
  input.maxLength = ean13 ? 12 : 128;
  input.inputMode = ean13 ? "numeric" : "text";
  input.dataset.i18nPlaceholder = "barcodeContentPlaceholder";
  input.placeholder = translate("barcodeContentPlaceholder");
  if (ean13 && !/^\d{12}$/.test(input.value)) input.value = "590123412345";
}

function activateMode(mode, focusTab = false) {
  currentMode = mode;
  tabs.forEach((tab) => {
    const active = tab.dataset.mode === mode;
    tab.classList.toggle("is-active", active);
    tab.setAttribute("aria-selected", String(active));
    tab.tabIndex = active ? 0 : -1;
    if (active && focusTab) tab.focus();
  });
  document.querySelectorAll("[role='tabpanel']").forEach((panel) => {
    panel.hidden = panel.id !== `group-${mode}`;
  });
  const barcodeMode = mode === "barcode";
  $("qr-options").hidden = barcodeMode;
  $("logo-options").hidden = barcodeMode;
  $("mode-summary").textContent = barcodeMode ? "Barcode" : "QR";
  clearPreview();
  scheduleGenerate(0);
}

function downloadBlob(blob, filename) {
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  document.body.append(anchor);
  anchor.click();
  anchor.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 0);
}

function downloadSvg() {
  if (!lastOutput?.svg) return;
  const filename = lastOutput.kind === "barcode" ? "barcode.svg" : "qr-code.svg";
  downloadBlob(new Blob([lastOutput.svg], { type: "image/svg+xml" }), filename);
  setStatus("downloaded", "success", { format: "SVG" });
}

function createPngBytes() {
  if (!lastOutput) return null;
  if (lastOutput.pngBytes) return lastOutput.pngBytes;
  if (lastOutput.kind === "barcode") {
    return wasm_generate_barcode_png(
      lastOutput.data,
      lastOutput.barcodeType,
      lastOutput.height,
      lastOutput.moduleWidth,
      lastOutput.foreground,
      lastOutput.background,
    );
  }
  return wasm_generate_qr_png(
    lastOutput.payload,
    lastOutput.ecLevel,
    lastOutput.moduleSize,
    lastOutput.margin,
    lastOutput.foreground,
    lastOutput.background,
  );
}

function downloadPng() {
  try {
    const bytes = createPngBytes();
    if (!bytes) return;
    const filename = lastOutput.kind === "barcode" ? "barcode.png" : "qr-code.png";
    downloadBlob(new Blob([bytes], { type: "image/png" }), filename);
    setStatus("downloaded", "success", { format: "PNG" });
  } catch (error) {
    reportError(error);
  }
}

tabBar.addEventListener("click", (event) => {
  const tab = event.target.closest("[role='tab']");
  if (tab) activateMode(tab.dataset.mode);
});

tabBar.addEventListener("keydown", (event) => {
  const currentIndex = tabs.findIndex((tab) => tab === document.activeElement);
  if (currentIndex < 0) return;
  let nextIndex = currentIndex;
  if (["ArrowRight", "ArrowDown"].includes(event.key)) nextIndex = (currentIndex + 1) % tabs.length;
  if (["ArrowLeft", "ArrowUp"].includes(event.key)) nextIndex = (currentIndex - 1 + tabs.length) % tabs.length;
  if (event.key === "Home") nextIndex = 0;
  if (event.key === "End") nextIndex = tabs.length - 1;
  if (nextIndex !== currentIndex) {
    event.preventDefault();
    activateMode(tabs[nextIndex].dataset.mode, true);
  }
});

form.addEventListener("submit", (event) => {
  event.preventDefault();
  generate();
});

form.addEventListener("input", (event) => {
  if (event.target === logoInput) return;
  if (event.target.id === "fg-color") $("fg-value").textContent = event.target.value.toUpperCase();
  if (event.target.id === "bg-color") $("bg-value").textContent = event.target.value.toUpperCase();
  if (event.target.id === "logo-ratio") $("logo-ratio-label").textContent = `${event.target.value}%`;
  scheduleGenerate();
});

form.addEventListener("change", (event) => {
  if (event.target === logoInput) return;
  if (event.target.id === "barcode-type") updateBarcodeInputContract();
  scheduleGenerate(0);
});

languageToggle.addEventListener("click", () => {
  language = language === "en" ? "zh" : "en";
  applyLanguage();
});

passwordToggle.addEventListener("click", () => {
  const input = $("wifi-password");
  input.type = input.type === "password" ? "text" : "password";
  updatePasswordToggleLabel();
});

logoInput.addEventListener("change", async () => {
  const [file] = logoInput.files;
  if (!file) return;
  const isPng = file.type === "image/png" && file.name.toLowerCase().endsWith(".png");
  if (!isPng) {
    logoInput.value = "";
    reportError(new UiError("invalidLogoType"));
    return;
  }
  if (file.size > MAX_LOGO_BYTES) {
    logoInput.value = "";
    reportError(new UiError("invalidLogoSize"));
    return;
  }
  logoData = new Uint8Array(await file.arrayBuffer());
  logoName = file.name;
  logoFileName.textContent = file.name;
  clearLogoButton.hidden = false;
  $("ec-level").value = "H";
  setStatus("logoLoaded", "success", { name: file.name });
  scheduleGenerate(0);
});

clearLogoButton.addEventListener("click", () => {
  logoData = null;
  logoName = null;
  logoInput.value = "";
  logoFileName.textContent = translate("noFile");
  clearLogoButton.hidden = true;
  setStatus("logoRemoved", "success");
  scheduleGenerate(0);
});

downloadSvgButton.addEventListener("click", downloadSvg);
downloadPngButton.addEventListener("click", downloadPng);

applyLanguage();
setStatus("initializing");

try {
  await init();
  initialized = true;
  updateBarcodeInputContract();
  await generate();
} catch (error) {
  reportError(error);
}
