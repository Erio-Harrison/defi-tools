# DeFi Tools on Solana

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-4.0+-3178C6.svg?logo=typescript)](https://www.typescriptlang.org/)

跨协议DeFi资产管理工具，提供自动化策略执行、风险管理和组合再平衡功能。

## 功能特性

### 🚀 核心功能
- 多协议资产配置（Raydium/Solend/Orca/Mango）
- 智能风险等级评估（1-5级）
- 自动再平衡策略
- 跨协议头寸追踪

### 📈 高级特性
- 动态滑点控制（0.1%-5%）
- 条件触发执行（时间/价格偏差）
- 组合健康度监控
- 紧急暂停机制

## 技术栈

### 区块链层
| 技术 | 版本 | 用途 |
|------|------|------|
| Solana | 1.16+ | 底层区块链 |
| Anchor | 0.29+ | 智能合约框架 |
| SPL Token | 3.5+ | 代币标准 |

### 中间件
| 组件 | 版本 | 说明 |
|------|------|------|
| SDK | 0.1.0 | TypeScript客户端库 |
| CPI Helpers | 1.2 | 跨程序调用工具 |

### 前端
| 框架 | 版本 | 用途 |
|------|------|------|
| Next.js | 14+ | 应用框架 |
| Wallet Adapter | 0.9+ | 钱包集成 |
| React Query | 5+ | 数据同步 |

## 快速开始

### 环境准备
```bash
# 安装Rust工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装Solana工具集
sh -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"

# 安装Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

### 本地部署
```bash
git clone https://github.com/your-org/defi-tools.git
cd defi-tools

# 安装依赖
npm run bootstrap

# 启动本地验证器
solana-test-validator --reset

# 部署程序
anchor deploy --provider.cluster localnet

# 启动前端
cd frontend && npm run dev
```

## SDK使用指南

### 安装
```bash
npm install @defi-tools/sdk
# 或
yarn add @defi-tools/sdk
```

### 初始化客户端
```typescript
import { Connection } from "@solana/web3.js";
import { DefiToolsClient } from "@defi-tools/sdk";

const RPC_ENDPOINT = "https://api.mainnet-beta.solana.com";
const connection = new Connection(RPC_ENDPOINT);

// 使用钱包适配器初始化
const client = new DefiToolsClient(
  connection,
  walletAdapter // 兼容Wallet Standard的对象
);
```

### 核心操作示例
```typescript
// 初始化用户配置
const tx1 = await client.initializeUser(3);

// 创建投资策略
const strategyParams = {
  allocations: [
    { protocol: 'Raydium', asset: 'SOL', targetWeight: 60 },
    { protocol: 'Solend', asset: 'USDC', targetWeight: 40 }
  ],
  rebalanceCondition: {
    trigger: 'deviation', 
    threshold: 5 // 5%偏差触发
  }
};
const tx2 = await client.createStrategy(strategyParams);

// 执行策略
const tx3 = await client.executeStrategy({
  strategyId: 1,
  amount: 5 // SOL
});
```

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

---

**注意事项**：
- 主网部署前需严格测试滑点计算逻辑
- 建议使用v0.29+版本的Anchor框架
- 前端需处理钱包交易签名超时情况

---

如有问题或建议，请联系：u7541840$gmail.com