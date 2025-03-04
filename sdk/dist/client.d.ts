import { Connection, PublicKey } from '@solana/web3.js';
import * as anchor from '@coral-xyz/anchor';
import { UserProfileData, StrategyConfigData, CreateStrategyParams, ExecuteStrategyParams, FundsOperationParams } from './types';
export declare class DefiToolsClient {
    private program;
    private connection;
    private wallet;
    /**
     * 创建DeFi工具集客户端
     * @param connection Solana连接
     * @param wallet Anchor钱包适配器
     */
    constructor(connection: Connection, wallet: anchor.Wallet);
    /**
     * 初始化用户配置
     * @param riskLevel 风险等级 (1-5)
     * @returns 交易签名
     */
    initializeUser(riskLevel: number): Promise<string>;
    /**
     * 创建投资策略
     * @param params 策略参数
     * @returns 交易签名
     */
    createStrategy(params: CreateStrategyParams): Promise<string>;
    /**
     * 执行策略
     * @param params 执行参数
     * @returns 交易签名
     */
    executeStrategy(params: ExecuteStrategyParams): Promise<string>;
    /**
     * 存入资金
     * @param params 存款参数
     * @returns 交易签名
     */
    depositFunds(params: FundsOperationParams): Promise<string>;
    /**
     * 提取资金
     * @param params 提款参数
     * @returns 交易签名
     */
    withdrawFunds(params: FundsOperationParams): Promise<string>;
    /**
     * 再平衡头寸
     * @param strategyId 策略ID
     * @returns 交易签名
     */
    rebalancePositions(strategyId: number): Promise<string>;
    /**
     * 获取用户配置数据
     * @param ownerAddress 用户地址
     * @returns 用户配置数据
     */
    getUserProfile(ownerAddress: PublicKey): Promise<UserProfileData>;
    /**
     * 获取策略配置数据
     * @param userProfilePda 用户配置PDA
     * @param strategyId 策略ID
     * @returns 策略配置数据
     */
    getStrategyConfig(userProfilePda: PublicKey, strategyId: number): Promise<StrategyConfigData>;
    /**
     * 获取用户所有策略
     * @param ownerAddress 用户地址
     * @returns 策略配置数据数组
     */
    getAllStrategies(ownerAddress: PublicKey): Promise<StrategyConfigData[]>;
    /**
     * 检查用户配置是否已初始化
     * @param ownerAddress 用户地址
     * @returns 是否已初始化
     */
    isUserInitialized(ownerAddress: PublicKey): Promise<boolean>;
}
