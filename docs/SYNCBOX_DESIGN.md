# 📄 SyncBox 完整产品与技术设计文档

版本：v1.0  
日期：2025 年 4 月 5 日  
目标：打造一个基于局域网、零云依赖、高隐私的智能文件同步系统

---

## 一、产品概述

### 1.1 项目名称

SyncBox —— 局域网智能文件同步中枢

### 1.2 核心理念

> 你的数据，只属于你。

SyncBox 是一个完全本地化、无需公网、无需云服务的文件同步解决方案，支持手机（iOS/Android）与电脑之间通过 IPv6 局域网 自动同步照片、视频等文件，并可将数据同步到 外接硬盘、U 盘、NAS，实现家庭级数据归档与备份。

---

## 二、产品目标

| 目标          | 说明                                          |
| ------------- | --------------------------------------------- |
| ✅ 零云依赖   | 所有数据仅在本地网络传输与存储                |
| ✅ 高隐私     | 不上传任何数据到第三方                        |
| ✅ 极简操作   | 插入硬盘 → 扫码连接 → 自动同步                |
| ✅ 多目标同步 | 支持同步到本地、U 盘、移动硬盘、NAS           |
| ✅ 跨平台     | iOS、Android、Windows（未来支持 macOS/Linux） |
| ✅ 自动发现   | 基于 mDNS + IPv6 实现设备自动识别             |

---

## 三、系统架构

```
+----------------+       +-------------------------------------+       +----------------+
|   iOS App      | <---> |   SyncBox Desktop (Tauri + Rust)    | <---> | 外接存储设备     |
| (SwiftUI)      |  IPv6 |  - GUI 界面                         |       | (U盘/硬盘/NAS)  |
+----------------+       |  - 内建 Axum 服务                    |       +----------------+
                        |  - mDNS 广播                        |
+----------------+       |  - 存储管理引擎                     |
| Android App    | <---> |  - 设备认证与同步控制               |
| (Kotlin)       |       +-------------------------------------+
+----------------+
```

- 通信方式：HTTP over IPv6（局域网直连）
- 发现机制：mDNS（`_syncbox._tcp.local`）
- 数据流向：手机 → 电脑 →（可选）U 盘/NAS
- 后端框架：Axum（Rust 异步 Web 框架）
- GUI 框架：Tauri（Rust + Web 前端）

---

## 四、核心功能模块

### 4.1 电脑端（SyncBox Desktop）

#### 技术栈

- 前端：Vue 3 / React + Tailwind CSS（Tauri 渲染层）
- 后端：Rust（Axum + Tokio）
- 打包：Tauri（生成 `.msi` 安装包）

#### 功能模块

| 模块          | 功能说明                                  |
| ------------- | ----------------------------------------- |
| 服务引擎      | 内建 Axum HTTP 服务，监听 `[::]:8080`     |
| GUI 界面      | 提供图形化配置、设备管理、日志查看        |
| mDNS 广播     | 广播 `_syncbox._tcp.local` 服务供手机发现 |
| 设备认证      | 管理手机设备绑定，支持扫码或手动确认      |
| 存储管理      | 支持本地路径、U 盘、NAS 作为同步目标      |
| 文件分发      | 上传后自动复制到多个目标（本地 + 外接）   |
| 系统托盘      | 最小化到托盘，右键可启停服务              |

---

### 4.2 后端服务（Rust + Axum）

#### 为什么选择 Axum？

- 完全异步，基于 Tokio
- 类型安全，编译期检查路由
- 与 Tower middleware 兼容（日志、认证、限流）
- 支持 WebSocket（未来扩展）
- 社区活跃，适合长期维护

#### 核心路由设计

```rust
// GET / -> 返回服务信息（用于发现）
// GET /api/devices -> 返回已绑定设备列表
// POST /api/upload -> 接收文件上传（multipart）
// POST /api/bind -> 处理设备绑定请求
// GET /api/logs -> 流式返回日志（SSE）
```

#### 中间件（Middleware）

- TraceLayer：记录请求日志
- AuthLayer：Bearer Token 认证
- SizeLimit：限制上传文件大小
- Compression：gzip 压缩响应

#### 依赖（Cargo.toml）

```toml
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio-util = { version = "0.7", features = ["codec"] }
```

---

### 4.3 移动端（iOS & Android）

#### 共同功能

- mDNS 扫描发现 SyncBox 服务
- 扫码或手动输入 IPv6 地址连接
- 相册权限获取与新文件检测
- 后台上传（iOS: BGURLSession, Android: WorkManager）
- 上传进度与历史记录

#### iOS（SwiftUI）

- 使用 `NWBrowser` 扫描 mDNS
- `Photos` 框架读取图片/视频
- `BGTaskScheduler` 实现后台同步

#### Android（Kotlin + Jetpack Compose）

- 使用 `NsdManager` 发现服务
- `MediaStore` 查询媒体文件
- `WorkManager` 管理上传任务
- 支持 HEIC 转 JPEG（可选）

---

## 五、外接存储与 NAS 支持

### 5.1 支持的存储类型

| 类型                | 说明                                 |
| ------------------- | ------------------------------------ |
| 本地文件夹          | 如 `D:\\Sync\\Photos`              |
| U 盘 / 移动硬盘     | 自动识别，支持安全弹出               |
| 网络 NAS            | 支持 SMB/CIFS 协议（群晖、威联通等） |

### 5.2 实现机制

#### 1. 外接设备识别

- Windows：使用 `windows-rs` 调用 `SetupAPI` 监听设备插拔
- Linux：`inotify` + `udev` 事件监听
- macOS：`DiskArbitration` 框架

```rust
#[cfg(target_os = "windows")]
use windows::Win32::Devices::Properties::DEVPKEY_Device_InstanceId;
```

#### 2. NAS 支持（SMB）

- 使用 `smbclient` crate 或 `tokio-smb`
- 支持用户名/密码认证
- 自动挂载为本地路径（如 `\\\\192.168.1.100\\Family`）
- 断线重连 + 指数退避重试

#### 3. 文件分发策略

```rust
async fn distribute_file(source: &Path, targets: &[StorageTarget]) {
    for target in targets {
        let dest = target.path.join(generate_subpath());
        match target.protocol {
            Local => fs::copy(source, &dest).await,
            SMB => smb_client.copy(source, &dest).await,
        }
    }
}
```

---

## 六、网络与通信设计

### 6.1 发现机制：mDNS

- 服务名：`_syncbox._tcp.local`
- 端口：`8080`
- TXT 记录：
  ```txt
  name=John's PC
  version=1.0
  path=/Photos
  ipv6=[fe80::1%en0]
  ```

### 6.2 通信协议

- 传输层：TCP over IPv6（优先）或 IPv4
- 应用层：HTTP/1.1（未来支持 HTTP/3）
- 安全：可选 TLS 1.3（自签证书 + 客户端信任）
- 认证：`Authorization: Bearer <token>`

### 6.3 IPv6 支持

- 自动获取本机 link-local 或 global IPv6 地址
- 支持 `%scope_id`（如 `[fe80::1%en0]`）
- 手机端可直接访问 `http://[fe80::1%wifi0]:8080`

---

## 七、数据模型

### 7.1 存储目标（StorageTarget）

```rust
enum StorageTarget {
    Local { path: PathBuf },
    Usb { serial: String, label: String, mount_point: PathBuf },
    Nas { address: String, share: String, username: String, encrypted_password: Vec<u8> },
}
```

### 7.2 已上传文件记录

```json
{
  "file_id": "uuid",
  "device_id": "ios-abc123",
  "filename": "IMG_20250405.jpg",
  "sha256": "a1b2c3...",
  "size": 3456789,
  "uploaded_at": "2025-04-05T10:25:10Z",
  "targets": ["local", "usb-travel", "nas-family"]
}
```

---

## 八、安全与隐私

| 项目     | 措施                                                     |
| -------- | -------------------------------------------------------- |
| 数据传输 | 局域网内，不经过公网                                     |
| 认证机制 | Bearer Token（UUID），可撤销                             |
| 密码存储 | NAS 密码使用操作系统密钥链（Windows Credential Manager） |
| 文件权限 | 仅访问用户指定目录                                       |
| 日志脱敏 | 不记录文件内容、用户身份                                 |
| 更新机制 | 官方签名发布，防止篡改                                   |

---

## 九、部署与分发

### 9.1 电脑端（Windows）

- 打包为 `.msi` 安装包（Tauri + WiX）
- 安装后自动创建开始菜单项和桌面快捷方式
- 可选：开机自启、系统托盘常驻

### 9.2 移动端

- iOS：App Store 上架，需说明“本地网络权限”
- Android：Google Play + APK 直装

### 9.3 配置文件

`config.toml` 示例：

```toml
port = 8080
sync_path = "D:\\Sync\\Photos"
nas_targets = [
    { address = "\\\\192.168.1.100\\Family", username = "backup", encrypted_password = "..." }
]
enable_mdns = true
log_level = "info"
```

---

## 十、用户流程示例

场景：旅行后自动备份到移动硬盘

1. 用户回家，插入“2TB 旅行备份盘”
2. SyncBox 桌面端弹出提示：“检测到新设备：Travel Disk”
3. 用户点击“添加为同步目标”
4. 手机打开 SyncBox App，自动连接电脑
5. 所有未上传照片开始同步
6. 上传完成后，自动复制到 U 盘 和 本地路径
7. 用户拔出 U 盘 前，点击“安全移除”

---

## 十一、未来扩展

| 版本 | 功能                           |
| ---- | ------------------------------ |
| v1.1 | 支持双向同步（电脑 → 手机）    |
| v1.2 | 支持端到端加密（E2EE）         |
| v1.3 | Web 管理界面（浏览器查看文件） |
| v2.0 | 支持 P2P 传输（QUIC + libp2p） |
| v2.1 | 支持 AI 智能分类（人物、地点、事件） |

---

## 十二、非功能性需求

| 项目     | 要求                               |
| -------- | ---------------------------------- |
| 性能     | 支持 100MB/s 上传速度              |
| 资源占用 | 内存 < 150MB，CPU < 5% idle        |
| 并发     | 支持最多 5 台设备同时连接          |
| 兼容性   | Windows 10/11, iOS 14+, Android 8+ |
| 可维护性 | 结构清晰，日志完整，配置可编辑     |

---

## 十三、总结

SyncBox 是一个面向未来的本地数据同步解决方案，结合：

- Rust + Axum：构建高性能、安全的后端服务
- Tauri：打造轻量、现代的桌面 GUI
- IPv6 + mDNS：实现零配置设备发现
- 外接存储 + NAS：满足真实用户备份需求

它不仅是一个工具，更是一个家庭数据中枢，帮助用户在数字时代真正掌控自己的数据。

---

## 附录

### 1. 推荐技术栈

| 模块     | 技术                         |
| -------- | ---------------------------- |
| Web 框架 | Axum                         |
| GUI 框架 | Tauri                        |
| mDNS     | async-mdns                   |
| SMB      | smbclient                    |
| 配置     | serde-toml                   |
| 日志     | tracing + tracing-subscriber |
| 打包     | tauri-cli                    |

### 2. 项目结构建议

```
syncbox/
├── server/             # Axum 后端
├── desktop/            # Tauri GUI + Rust 逻辑
├── ios-client/         # SwiftUI
├── android-client/     # Kotlin
└── shared/             # 共享类型定义（如 API 模型）
```
