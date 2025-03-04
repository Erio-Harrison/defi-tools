import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';
export declare enum ProtocolType {
    Raydium = 0,
    Solend = 1,
    Orca = 2,
    Mango = 3
}
export declare enum AssetType {
    SOL = 0,
    USDC = 1,
    USDT = 2
}
export interface Allocation {
    protocol: ProtocolType | number;
    asset: AssetType | number;
    targetWeightBps: number;
}
export interface RebalanceCondition {
    timeIntervalSeconds: number;
    maxDeviationBps: number;
    autoRebalance: boolean;
}
export interface UserProfileData {
    owner: PublicKey;
    riskLevel: number;
    strategyCounter: number;
    vaultBump: number;
    lastActivity: BN;
    totalValueLamports: BN;
    isPaused: boolean;
}
export interface StrategyConfigData {
    owner: PublicKey;
    strategyId: number;
    allocations: Allocation[];
    rebalanceCondition: RebalanceCondition;
    createdAt: BN;
    lastExecutedAt: BN;
    maxSlippageBps: number;
}
export interface PositionTrackerData {
    strategyId: number;
    protocol: number[] | string;
    asset: number[] | string;
    positionValueLamports: BN;
    initialApyBps: number;
    currentApyBps: number;
    lastUpdatedAt: BN;
}
export interface CreateStrategyParams {
    allocations: Allocation[];
    rebalanceCondition: RebalanceCondition;
    maxSlippageBps: number;
}
export interface ExecuteStrategyParams {
    strategyId: number;
}
export interface FundsOperationParams {
    strategyId: number;
    amount: number;
}
