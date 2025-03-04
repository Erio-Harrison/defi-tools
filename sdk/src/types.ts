import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';

// 协议类型枚举
export enum ProtocolType {
  Raydium = 0,
  Solend = 1,
  Orca = 2,
  Mango = 3,
}

// 资产类型枚举
export enum AssetType {
  SOL = 0,
  USDC = 1,
  USDT = 2,
}

// 资产分配项
export interface Allocation {
  protocol: ProtocolType | number;
  asset: AssetType | number;
  targetWeightBps: number;
}

// 再平衡条件
export interface RebalanceCondition {
  timeIntervalSeconds: number;
  maxDeviationBps: number;
  autoRebalance: boolean;
}

// 用户配置数据
export interface UserProfileData {
  owner: PublicKey;
  riskLevel: number;
  strategyCounter: number;
  vaultBump: number;
  lastActivity: BN;
  totalValueLamports: BN;
  isPaused: boolean;
}

// 策略配置数据
export interface StrategyConfigData {
  owner: PublicKey;
  strategyId: number;
  allocations: Allocation[];
  rebalanceCondition: RebalanceCondition;
  createdAt: BN;
  lastExecutedAt: BN;
  maxSlippageBps: number;
}

// 头寸追踪数据
export interface PositionTrackerData {
  strategyId: number;
  protocol: number[] | string;
  asset: number[] | string;
  positionValueLamports: BN;
  initialApyBps: number;
  currentApyBps: number;
  lastUpdatedAt: BN;
}

// 策略创建参数
export interface CreateStrategyParams {
  allocations: Allocation[];
  rebalanceCondition: RebalanceCondition;
  maxSlippageBps: number;
}

// 策略执行参数
export interface ExecuteStrategyParams {
  strategyId: number;
}

// 资金存取参数
export interface FundsOperationParams {
  strategyId: number;
  amount: number;
}
