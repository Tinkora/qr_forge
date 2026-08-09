# 发布检查清单

[English](./RELEASE_CHECKLIST.md)

每次发布 qr_forge 都必须使用本清单。完成的清单应记录在发布 PR 或跟踪 issue 中。每个已勾选项目必须指向命令输出、必要 GitHub check 或手工验证结果。

## 发布记录

- 版本：`vX.Y.Z`
- 目标 commit：
- 发布负责人：
- 评审者：
- 计划日期：
- 跟踪 issue 或 PR：

## 1. 范围与契约

- [ ] 所有发布行为均已实现；建议行为没有被描述为可用。
- [ ] 变更符合[产品范围](./PRODUCT_SCOPE.zh-CN.md)，或者已经关联批准的范围决策。
- [ ] [产品契约](./CONTRACT.zh-CN.md)与当前 Rust、WASM 和浏览器行为一致。
- [ ] [成熟度](./MATURITY.zh-CN.md)如实记录实验性能力和已知限制。
- [ ] 英文与简体中文文档含义对等。
- [ ] `CHANGELOG.md` 在发布版本下包含用户可见变更、修复、安全说明和迁移方式。
- [ ] 仓库中没有旧组织 URL、私有路径、凭据、生成 package 或内部迁移说明。
- [ ] 公开代码注释和 commit 使用英文；Markdown 不含 emoji。

## 2. 版本与仓库状态

- [ ] Working tree 干净，发布 commit 位于 `main`。
- [ ] `Cargo.toml`、两个 crate manifest 和 `Cargo.lock` 的目标版本及 Rust 1.85 基线一致。
- [ ] Repository、homepage、license、description 和 README 元数据均指向 `Tinkora/qr_forge`。
- [ ] `vX.Y.Z` tag 和 GitHub release 尚不存在。
- [ ] 所有必要 PR review 和 branch protection check 已满足。

## 3. Rust 与 WebAssembly 验证

使用仓库固定或声明的 toolchain 运行，并包含 Rust 1.85 的 MSRV 证据：

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
rustup target add wasm32-unknown-unknown
cargo check -p qr_forge_core --target wasm32-unknown-unknown
cargo check -p qr_forge_web --target wasm32-unknown-unknown
wasm-pack build --target web crates/qr_forge_web -- --locked

cd crates/qr_forge_web
npm ci
npx playwright install chromium
npm run test:wasm-smoke:local
```

- [ ] 格式检查通过。
- [ ] 所有 workspace 测试通过。
- [ ] 严格 Clippy 通过，且没有宽泛 lint suppression。
- [ ] 两个 crate 均可为 `wasm32-unknown-unknown` 编译。
- [ ] `wasm-pack` 生成 JavaScript 和 WebAssembly package，且没有缺少元数据的 warning。
- [ ] Playwright WASM smoke suite 在配置的四个 viewport 中全部通过。
- [ ] 必需的 CI、MSRV、依赖、许可证、安全和 CodeQL check 在准确的发布 commit 上通过。
- [ ] 二维码、Code 128 和 EAN-13 PNG 的独立解码器测试通过。

## 4. 浏览器验证

构建并启动准确的候选版本：

```bash
mkdir -p crates/qr_forge_web/static/pkg
cp crates/qr_forge_web/pkg/* crates/qr_forge_web/static/pkg/
python3 -m http.server 8080 --directory crates/qr_forge_web/static
```

- [ ] 应用加载时控制台没有 error 或 warning，WASM 请求成功返回。
- [ ] 没有意外外部运行时请求，输入的 payload 未被传输。
- [ ] 默认语言为英文；简体中文切换会更新可见文本和无障碍文本。
- [ ] 网址或文本、Wi-Fi、vCard、电话、邮件、Code 128 和 EAN-13 模式都能生成预期输出。
- [ ] 已检查 Wi-Fi 有意义空白和分隔符转义情况。
- [ ] EAN-13 错误长度和字符、Code 128 错误字符和长度、空二维码输入都会显示有用错误。
- [ ] SVG 和 PNG 下载包含最新生成的 payload。
- [ ] 有效 PNG Logo 显示在 PNG 预览中，选择 H 纠错，禁用 SVG 下载并保持 PNG 下载可用。
- [ ] 无效 Logo 类型、尺寸和比例会失败，不保留过时或误导输出。
- [ ] 密码显示、tab 方向键、Home 和 End、焦点顺序、label 和状态通知工作正常。
- [ ] 在 375、768、1024 和 1440 像素宽度下，没有水平溢出、裁切或不连贯重叠。
- [ ] 保持 reduced-motion 行为和可见焦点。
- [ ] 除自动解码外，关键示例输出也通过独立真实相机或扫描器测试。

## 5. 安全与供应链

- [ ] 已启用 GitHub 私密漏洞报告，且链接能够打开私密 advisory 表单。
- [ ] 没有未解决 security advisory 阻止发布。
- [ ] Dependency review、`cargo audit`、`cargo deny` 或仓库中的等效 workflow 通过。
- [ ] GitHub Actions 固定到完整 commit SHA，并使用最小权限。
- [ ] 构建和 Pages workflow 不使用 `curl | sh`、不可信 PR secret 或可变发布输入。
- [ ] 发布产物包含仓库策略要求的 provenance、checksum 或 attestation。
- [ ] 发布 commit 已通过 secret scanning，不含测试凭据或敏感二维码示例。

## 6. 发布

- [ ] 只有在所有门禁通过后，才把发布 PR merge 到 `main`。
- [ ] 从已验证 commit 创建签名或符合仓库策略的 annotated tag `vX.Y.Z`。
- [ ] 让发布 workflow 从 tag 构建产物；不要上传未经验证的本地产物。
- [ ] 使用来自 changelog 的说明及所有兼容性警告发布 GitHub release。
- [ ] 确认 Pages 部署关联发布 commit 或明确记录的后续 commit。
- [ ] 确认 release asset、checksum 或 attestation、源码链接和许可证文件可下载。

## 7. 发布后

- [ ] 在干净浏览器会话中打开公开 Pages URL，分别生成并下载一个二维码、Code 128 和 EAN-13 输出。
- [ ] 确认 README badge、文档链接、issue template、Discussions 和安全报告链接有效。
- [ ] 确认 GitHub 上的 release 和部署 workflow 为绿色。
- [ ] 公告只描述已发布契约中的能力。
- [ ] 为推迟工作创建后续 issue；不要在源码或公开声明中保留无人负责的发布 TODO。

## 停止条件

如果必要检查失败、扫描器输出不可复现、隐私边界被破坏、存在未解决的 critical 或 high severity 漏洞、双语文档含义存在重大差异，或无法从目标 commit 重新构建发布，则不得发布。

如果错误版本已经公开，应在 GitHub release 中明确标记，适当时停止或回滚 Pages 部署，从经过评审的 commit 发布修正 patch version，并在 changelog 或 security advisory 中记录事件。
