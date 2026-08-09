# 产品契约

[English](./CONTRACT.md)

本文档定义 qr_forge 0.1.x 已实现的输入、输出和错误行为。产品描述不得承诺超出本契约的行为。如果文档存在缺陷，以源码和测试为准。

## 支持的接口

1. `crates/qr_forge_web/static` 中的静态浏览器应用
2. `qr_forge_core` 的公开 Rust module 和 re-export
3. `qr_forge_web` 通过 `wasm-bindgen` 导出的 JavaScript 函数

项目目前不发布托管生成 API、CLI、MCP 接口、npm package 或 crates.io package。

## 通用二维码契约

| 输入 | 接受值 | Rust 核心默认值 | 浏览器 UI 范围 |
| ------ | -------- | ----------------- | ----------------- |
| 数据 | `qrcode` 接受的非空 UTF-8 文本 | 无 | 非空 |
| 纠错 | `L`、`M`、`Q` 或 `H`；Rust/WASM 中不区分大小写 | `M` | `L`、`M`、`Q` 或 `H`；初始选择为 `H` |
| 模块尺寸 | 1 至 64 像素的整数 | `8` | 1 至 32 |
| 边距 | 0 至 16 个模块的整数 | `4` | 0 至 16 |
| 前景色 | 六位十六进制数字，可带一个 `#` 前缀 | `#000000` | 浏览器颜色输入 |
| 背景色 | 六位十六进制数字，可带一个 `#` 前缀 | `#FFFFFF` | 浏览器颜色输入 |

SVG 输出是由背景和模块 `<rect>` 元素组成的正方形 SVG 字符串。PNG 输出是 RGBA PNG 字节序列。每边输出像素尺寸为 `(矩阵模块数 + 2 * 边距) * 模块尺寸`。

边距由用户控制。边距为零是有效输入，但可能降低扫描器兼容性。使用者需要选择足够的颜色对比度，并独立测试关键输出。

## Payload 模式

### 网址或文本

输入值不会经过网址规范化或网络校验，空白字符具有实际意义。空输入会被拒绝。

### Wi-Fi

`wifi_qr_config` 及其 WASM wrapper 执行以下规则：

- SSID 非空且不超过 32 个 UTF-8 字节
- 密码不超过 64 个 UTF-8 字节
- 加密类型不区分大小写，接受 `WPA`、`WPA2`、`WEP`、`NOPASS` 或空字符串
- `WPA2` 输出为 `WPA`；`NOPASS` 和空加密类型输出为 `nopass`
- 反斜杠、分号、逗号、冒号和双引号会被转义
- SSID 和密码首尾空白保持不变

受保护网络输出 `WIFI:T:<type>;S:<ssid>;P:<password>;;`，开放网络输出 `WIFI:S:<ssid>;T:nopass;;`。

浏览器 UI 提供 WPA 或 WPA2、WEP 及开放网络选项。它不会连接网络，也不会向接入点验证凭据。

### vCard

`vcard_qr_config` 输出包含 `FN` 字段，以及可选 `TEL`、`EMAIL`、`ORG` 字段的 vCard 3.0 文本。姓名 trim 后必须非空。所有字段都会 trim；反斜杠、分号、逗号和换行符会被转义。函数不验证电话或邮件格式。

### 电话和邮件

SVG helper 会 trim 输入并添加 `tel:` 或 `mailto:` 前缀，不会规范化或校验值。浏览器 UI 要求输入非空后才允许生成。

## 条形码契约

| 类型 | 输入 | 编码 | 静区 |
| --- | --- | --- | --- |
| Code 128 | 1 至 128 字节，每个字节为 ASCII 32 至 126 | 偶数长度纯数字输入使用子集 C，否则使用子集 B | 两侧各 10 个模块 |
| EAN-13 | 恰好 12 位 ASCII 数字 | 编码器计算校验位；数据编码为 95 个模块 | 左侧 11 个模块，右侧 7 个模块 |

条形码高度必须为 20 至 2000 像素，模块宽度必须为 1 至 16 像素。SVG 和 PNG 输出只包含条和背景，不绘制人类可读文字。像素宽度包含静区。

Code 128 不支持控制字符、非 ASCII 文本、子集 A 或一个 payload 内的自适应子集切换。EAN-13 输入不得包含空格或第 13 位校验位。

## Logo 叠加契约

- Logo 输入必须能被发布构建解码为 PNG。
- 比例必须为有限数，并且包含在 `0.05` 至 `0.30` 范围内。
- Logo 被缩放为正方形，并居中覆盖二维码数据区。
- Logo 周围添加两像素白色 padding。
- 浏览器接受最大 2 MiB 的 PNG 文件，并自动选择 H 纠错。
- Logo 只包含在 PNG 输出中。Logo 生效时，浏览器会禁用 SVG 下载，避免提供一个悄悄省略 Logo 的输出。

Logo 会遮挡模块。成功生成不代表结果能被所有相机或扫描器识别。

## Rust API

主要公开 Rust 接口包括：

- `generate_qr`、`QrOptions`、`QrEcLevel` 和 `QrMatrix`
- `wifi_qr_config` 和 `vcard_qr_config`
- `generate_barcode`、`BarcodeType` 和 `BarcodeMatrix`
- `qr_to_svg`、`qr_to_png`、`barcode_to_svg` 和 `barcode_to_png`
- `render::qr_to_png_with_logo`
- `CoreError`

矩阵结构支持序列化，但在 1.0 之前不应被视为持久化存储格式。

## WebAssembly API

除特别说明外，下表颜色参数为字符串，尺寸为无符号整数。

| 函数 | 结果 |
| ------ | ------ |
| `wasm_generate_qr_svg(data, ec, module_size, margin, fg, bg)` | 包含 `svg` 和数值 `size` 的 object |
| `wasm_generate_qr_png(data, ec, module_size, margin, fg, bg)` | PNG `Uint8Array` |
| `wasm_generate_qr_png_with_logo(data, ec, module_size, margin, fg, bg, logo, ratio)` | PNG `Uint8Array` |
| `wasm_wifi_qr_svg(ssid, password, encryption, ec, module_size, margin, fg, bg)` | 包含 `svg` 和 `size` 的 object |
| `wasm_wifi_qr_png(ssid, password, encryption, ec, module_size, margin, fg, bg)` | PNG `Uint8Array` |
| `wasm_wifi_payload(ssid, password, encryption)` | Wi-Fi payload 字符串 |
| `wasm_vcard_qr_svg(name, phone, email, org, ec, module_size, margin, fg, bg)` | 包含 `svg` 和 `size` 的 object |
| `wasm_vcard_qr_png(name, phone, email, org, ec, module_size, margin, fg, bg)` | PNG `Uint8Array` |
| `wasm_vcard_payload(name, phone, email, org)` | vCard payload 字符串 |
| `wasm_phone_qr_svg(phone, ec, module_size, margin, fg, bg)` | 包含 `svg` 和 `size` 的 object |
| `wasm_email_qr_svg(email, ec, module_size, margin, fg, bg)` | 包含 `svg` 和 `size` 的 object |
| `wasm_generate_barcode_svg(data, type, height, module_width, fg, bg)` | 包含 `svg`、数值 `width` 和数值 `height` 的 object |
| `wasm_generate_barcode_png(data, type, height, module_width, fg, bg)` | PNG `Uint8Array` |
| `get_version()` | Package 版本字符串 |

Barcode type 不区分大小写，接受 `code128`、`code_128`、`code-128`、`ean13`、`ean_13` 或 `ean-13`。未知 barcode type 目前抛出普通 JavaScript 字符串；由 `CoreError` 产生的错误会抛出包含 `code` 和 `message` 的 object。

## 机器可读错误码

| 错误码 | 条件 |
| -------- | ------ |
| `EMPTY_DATA` | 必填数据为空 |
| `INVALID_DATA` | 二维码编码器拒绝数据 |
| `INVALID_EC_LEVEL` | 纠错不是 L、M、Q 或 H |
| `INVALID_HEX_COLOR` | 颜色不是六位十六进制数字 |
| `INVALID_EAN13` | EAN-13 输入长度不是 12 字节 |
| `INVALID_EAN13_CHARS` | EAN-13 输入包含非数字 |
| `INVALID_CODE128` | Code 128 输入为空时使用 `EMPTY_DATA`，否则表示包含不支持的字节 |
| `CODE128_TOO_LONG` | Code 128 输入超过 128 字节 |
| `INVALID_WIFI_SSID` | SSID 为空或超过 32 字节 |
| `INVALID_WIFI_PASSWORD` | 密码超过 64 字节 |
| `INVALID_WIFI_ENCRYPTION` | 加密类型不受支持 |
| `MISSING_VCARD_NAME` | Trim 后的 vCard 姓名为空 |
| `SVG_RENDER_FAILED` | SVG 渲染失败 |
| `PNG_RENDER_FAILED` | PNG 渲染或编码失败 |
| `LOGO_OVERLAY_FAILED` | Logo 解码、尺寸调整或合成失败 |
| `INVALID_LOGO_RATIO` | Logo 比例不是有限数，或超出 0.05 至 0.30 |
| `INVALID_MODULE_SIZE` | 二维码模块尺寸超出 1 至 64 |
| `INVALID_MARGIN` | 二维码边距超过 16 |
| `INVALID_BARCODE_HEIGHT` | 条形码高度超出 20 至 2000 |
| `INVALID_BARCODE_MODULE_WIDTH` | 条形码模块宽度超出 1 至 16 |

错误消息是供人阅读的诊断文本，措辞可能被澄清。使用者应根据 `code` 分支，不应解析英文消息。

## 兼容性策略

0.1 版本仍处于 1.0 之前。Rust 类型、WASM signature、生成的 markup 和 UI 结构可能在 minor release 中变化，但必须记录在 [CHANGELOG.md](../CHANGELOG.md)。维护者应避免在 0.1.x 内改变现有错误码的含义，并为有意的契约变更提供迁移说明。
