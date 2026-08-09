import { readFile } from "node:fs/promises";

import { expect, test } from "@playwright/test";

const PNG_SIGNATURE = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
const LOGO_PNG = Buffer.from(
  "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=",
  "base64"
);

async function waitForQrReady(page) {
  await expect(page.locator("#status")).toContainText("QR ready");
  await expect(page.locator("#preview-stage")).toHaveAttribute("aria-busy", "false");
  await expect(page.getByRole("img", { name: "Generated QR code preview" })).toBeVisible();
}

async function downloadBytes(page, button, expectedFilename) {
  const downloadPromise = page.waitForEvent("download");
  await button.click();
  const download = await downloadPromise;
  expect(download.suggestedFilename()).toBe(expectedFilename);
  const path = await download.path();
  expect(path).not.toBeNull();
  return readFile(path);
}

test("initializes real WASM and switches between English and Chinese", async ({ page }) => {
  const wasmResponsePromise = page.waitForResponse((response) =>
    response.url().endsWith("/pkg/qr_forge_web_bg.wasm")
  );
  await page.goto("/");
  const wasmResponse = await wasmResponsePromise;

  expect(wasmResponse.status()).toBe(200);
  expect(wasmResponse.headers()["content-type"]).toContain("application/wasm");
  await waitForQrReady(page);
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect(page.getByRole("heading", { name: "Create code" })).toBeVisible();

  const version = await page.evaluate(async () => {
    const wasm = await import("/pkg/qr_forge_web.js");
    return wasm.get_version();
  });
  expect(version).toMatch(/^\d+\.\d+\.\d+/);

  await page.getByRole("button", { name: "Switch to Chinese" }).click();
  await expect(page.locator("html")).toHaveAttribute("lang", "zh-CN");
  await expect(page.getByRole("heading", { name: "创建编码" })).toBeVisible();
  await expect(page.locator("#status")).toContainText("二维码已生成");
  await expect(page.getByRole("img", { name: "已生成的二维码预览" })).toBeVisible();
});

test("supports arrow, Home, and End keyboard navigation across code types", async ({ page }) => {
  await page.goto("/");
  await waitForQrReady(page);

  const textTab = page.getByRole("tab", { name: "URL / Text" });
  const wifiTab = page.getByRole("tab", { name: "Wi-Fi" });
  const barcodeTab = page.getByRole("tab", { name: "Barcode" });

  await textTab.focus();
  await page.keyboard.press("ArrowRight");
  await expect(wifiTab).toBeFocused();
  await expect(wifiTab).toHaveAttribute("aria-selected", "true");
  await expect(page.locator("#group-qr-wifi")).toBeVisible();

  await page.keyboard.press("End");
  await expect(barcodeTab).toBeFocused();
  await expect(barcodeTab).toHaveAttribute("aria-selected", "true");
  await expect(page.locator("#group-barcode")).toBeVisible();

  await page.keyboard.press("Home");
  await expect(textTab).toBeFocused();
  await expect(textTab).toHaveAttribute("aria-selected", "true");
  await expect(page.locator("#group-qr-url")).toBeVisible();
});

test("preserves Wi-Fi boundary spaces and escapes canonical payload fields", async ({ page }) => {
  await page.goto("/");
  await waitForQrReady(page);

  const ssid = " Office;Net\\Lab ";
  const password = " pass:word,42\\x ";
  const payload = await page.evaluate(
    async ({ ssidValue, passwordValue }) => {
      const wasm = await import("/pkg/qr_forge_web.js");
      return wasm.wasm_wifi_payload(ssidValue, passwordValue, "WPA");
    },
    { ssidValue: ssid, passwordValue: password }
  );
  expect(payload).toBe(
    "WIFI:T:WPA;S: Office\\;Net\\\\Lab ;P: pass\\:word\\,42\\\\x ;;"
  );

  await page.getByRole("tab", { name: "Wi-Fi" }).click();
  await page.getByLabel("Network name").fill(ssid);
  await page.locator("#wifi-password").fill(password);
  await page.getByRole("button", { name: "Show password" }).click();
  await expect(page.locator("#wifi-password")).toHaveAttribute("type", "text");
  await expect(page.locator("#wifi-password")).toHaveValue(password);
  await page.getByRole("button", { name: "Generate" }).click();
  await expect(page.locator("#status")).toContainText("QR ready");
  await expect(page.getByRole("img", { name: "Generated QR code preview" })).toBeVisible();
});

test("escapes vCard fields in real WASM and generates the contact QR", async ({ page }) => {
  await page.goto("/");
  await waitForQrReady(page);

  const payload = await page.evaluate(async () => {
    const wasm = await import("/pkg/qr_forge_web.js");
    return wasm.wasm_vcard_payload(
      " Ada;Lovelace ",
      " +1,555;0100 ",
      " ada@example.com ",
      " Tinkora, Labs\nR&D\\HQ "
    );
  });
  expect(payload).toBe(
    [
      "BEGIN:VCARD",
      "VERSION:3.0",
      "FN:Ada\\;Lovelace",
      "TEL:+1\\,555\\;0100",
      "EMAIL:ada@example.com",
      "ORG:Tinkora\\, Labs\\nR&D\\\\HQ",
      "END:VCARD"
    ].join("\n")
  );

  await page.getByRole("tab", { name: "vCard" }).click();
  await page.locator("#vcard-name").fill("Ada;Lovelace");
  await page.locator("#vcard-phone").fill("+1,555;0100");
  await page.locator("#vcard-email").fill("ada@example.com");
  await page.locator("#vcard-org").fill("Tinkora, Labs\\HQ");
  await page.getByRole("button", { name: "Generate" }).click();
  await expect(page.locator("#status")).toContainText("QR ready");
  await expect(page.getByRole("img", { name: "Generated QR code preview" })).toBeVisible();
});

test("uses PNG preview and disables SVG export when a logo is present", async ({ page }) => {
  await page.goto("/");
  await waitForQrReady(page);

  await page.locator("#logo-file").setInputFiles({
    name: "logo.png",
    mimeType: "image/png",
    buffer: LOGO_PNG
  });

  await expect(page.locator("#preview-stage")).toHaveAttribute("aria-busy", "false");
  await expect(page.locator("#status")).toContainText("QR ready");
  await expect(page.locator("#logo-file-name")).toHaveText("logo.png");
  await expect(page.getByLabel("Correction")).toHaveValue("H");
  await expect(page.locator("#qr-display img")).toBeVisible();
  await expect(page.locator("#qr-display svg")).toHaveCount(0);
  await expect(page.locator("#btn-download-svg")).toBeDisabled();
  await expect(page.locator("#btn-download-png")).toBeEnabled();

  const png = await downloadBytes(page, page.locator("#btn-download-png"), "qr-code.png");
  expect(png.subarray(0, PNG_SIGNATURE.length)).toEqual(PNG_SIGNATURE);
  expect(png.length).toBeGreaterThan(100);
});

test("rejects invalid EAN-13 data and renders valid EAN-13 data", async ({ page }) => {
  await page.goto("/");
  await waitForQrReady(page);

  await page.getByRole("tab", { name: "Barcode" }).click();
  await page.getByLabel("Barcode type").selectOption("ean13");
  await page.getByLabel("Barcode data").fill("12345");
  await page.getByRole("button", { name: "Generate" }).click();
  await expect(page.locator("#status")).toHaveText("EAN-13 requires exactly 12 digits");
  await expect(page.locator("#btn-download-svg")).toBeDisabled();
  await expect(page.locator("#btn-download-png")).toBeDisabled();

  await page.getByLabel("Barcode data").fill("590123412345");
  await page.getByRole("button", { name: "Generate" }).click();
  await expect(page.locator("#status")).toContainText("Barcode ready");
  await expect(page.getByRole("img", { name: "Generated barcode preview" })).toBeVisible();
  await expect(page.locator("#barcode-text-display")).toHaveText("590123412345");
});

test("downloads Code 128 as SVG and PNG", async ({ page }) => {
  await page.goto("/");
  await waitForQrReady(page);

  await page.getByRole("tab", { name: "Barcode" }).click();
  await page.getByLabel("Barcode type").selectOption("code128");
  await page.getByLabel("Barcode data").fill("TINKORA-128 2026");
  await page.getByRole("button", { name: "Generate" }).click();
  await expect(page.locator("#status")).toContainText("Barcode ready");
  await expect(page.locator("#btn-download-svg")).toBeEnabled();
  await expect(page.locator("#btn-download-png")).toBeEnabled();

  const svg = await downloadBytes(page, page.locator("#btn-download-svg"), "barcode.svg");
  expect(svg.toString("utf8")).toContain("<svg");

  const png = await downloadBytes(page, page.locator("#btn-download-png"), "barcode.png");
  expect(png.subarray(0, PNG_SIGNATURE.length)).toEqual(PNG_SIGNATURE);
  expect(png.length).toBeGreaterThan(100);
});

test("loads without external requests, console problems, failed responses, or overflow", async ({
  baseURL,
  page
}) => {
  const problems = [];
  const externalRequests = [];
  const failedResponses = [];
  const expectedOrigin = new URL(baseURL).origin;

  page.on("console", (message) => {
    if (["error", "warning"].includes(message.type())) {
      problems.push(`${message.type()}: ${message.text()}`);
    }
  });
  page.on("pageerror", (error) => problems.push(`pageerror: ${error.message}`));
  page.on("request", (request) => {
    if (new URL(request.url()).origin !== expectedOrigin) {
      externalRequests.push(request.url());
    }
  });
  page.on("response", (response) => {
    if (response.status() >= 400) {
      failedResponses.push(`${response.status()} ${response.url()}`);
    }
  });

  await page.goto("/");
  await page.waitForLoadState("networkidle");
  await waitForQrReady(page);

  const layout = await page.evaluate(() => ({
    bodyScrollWidth: document.body.scrollWidth,
    clientWidth: document.documentElement.clientWidth,
    documentScrollWidth: document.documentElement.scrollWidth
  }));
  expect(layout.documentScrollWidth).toBe(layout.clientWidth);
  expect(layout.bodyScrollWidth).toBeLessThanOrEqual(layout.clientWidth);
  expect(externalRequests).toEqual([]);
  expect(failedResponses).toEqual([]);
  expect(problems).toEqual([]);
});
