# 文件处理工具包 (File Processor Bundle)

## 项目概述

`file-processor-bundle` 是一个高效的文件处理工具包，提供了一系列便捷的文件操作功能，包括文件合并、分割、格式转换等。
<img width="500" alt="image" src="https://github.com/user-attachments/assets/d9bfbba9-7fb9-4293-b93d-d98615a2f462" />
<img width="500" alt="image" src="https://github.com/user-attachments/assets/9f0248af-6beb-4701-b88a-6a0ada16f9c5" />

## 主要功能

- 文件合并：支持多文件合并为单个文件
- 文件分割：按指定大小或行数分割文件
- [todo]多文件内容替换: 支持按行或正则表达式替换文件内容
- [todo]多文件内容多值查找: 支持按正则表达式或多个值集查找文件内容
- [todo]Excel拆分: 支持将Excel文件按指定[列、行、sheet]拆分成多个文件
- [todo]Excel合并: 支持将Excel按[列、行、sheet]合并为一个文件
- [todo]Excel多值查找: 支持在多个Excel文件中查找指定值集
- [todo]Excel转JSON: 支持将Excel文件转换为JSON格式
- [todo]JSON转Excel: 支持将JSON文件转换为Excel格式

## 技术栈

- 前端：Vue 3 + TypeScript + Vite
- 后端：Rust (Tauri)

## 安装与使用

### 开发环境

```bash
# 克隆项目
git clone https://github.com/your-repo/file-processor-bundle.git

# 安装依赖
cd file-processor-bundle
yarn install

# 运行开发模式
yarn tauri dev
```

### 生产构建

```bash
yarn tauri build
```

## 配置说明

项目配置详见 `tauri.conf.json`，可自定义：
- 应用名称和图标
- 文件处理权限
- 窗口设置

## 贡献指南

欢迎提交Pull Request或Issue。请确保：
- 代码符合现有风格
- 包含必要的测试
- 更新相关文档

## 许可证

MIT License


