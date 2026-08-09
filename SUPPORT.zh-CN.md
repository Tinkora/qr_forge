# 支持

[English](./SUPPORT.md)

## 选择正确渠道

| 需求 | 渠道 |
| ------ | ------ |
| 安装、使用或设计问题 | [GitHub Discussions](https://github.com/Tinkora/qr_forge/discussions) |
| 可复现 bug | [GitHub Issues](https://github.com/Tinkora/qr_forge/issues/new/choose) |
| 功能建议 | 先使用 [GitHub Discussions](https://github.com/Tinkora/qr_forge/discussions)，再决定是否创建 issue |
| 疑似安全漏洞 | [私密漏洞报告](https://github.com/Tinkora/qr_forge/security/advisories/new) |

不要在公开 issue 或 discussion 中放入密码、私密网址、联系人记录、access token 或其他敏感二维码 payload。请使用能够复现问题的最小虚构值替代。

## 提问之前

1. 阅读 [README](./README.zh-CN.md)、[产品契约](./docs/CONTRACT.zh-CN.md)和[成熟度](./docs/MATURITY.zh-CN.md)。
2. 检查已有 issue 和 discussion。
3. 使用最新 GitHub release 或当前 Pages 部署复现。
4. 记录浏览器、操作系统、版本或 commit、输入模式和输出格式。

对于扫描器兼容问题，请说明扫描器，并尽可能提供不含敏感信息的示例输出。对于构建失败，请提供失败命令和相关错误输出，但应删除 token 和本地 secret。

## 支持边界

社区支持按维护者时间尽力提供，不保证响应时间。维护者支持最新 GitHub release 和当前项目部署，不负责自定义 fork、无关条形码硬件、浏览器扩展或第三方托管配置。

一般问题应使用 Discussions。Issue 应描述可以采取行动的行为。安全报告必须遵守 [SECURITY.zh-CN.md](./SECURITY.zh-CN.md)。
