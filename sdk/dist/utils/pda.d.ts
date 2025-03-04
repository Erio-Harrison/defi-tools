import { PublicKey } from '@solana/web3.js';
export declare const PROGRAM_ID: PublicKey;
/**
 * 查找用户配置PDA
 * @param owner 用户钱包地址
 * @returns [PDA, bump]
 */
export declare function findUserProfilePDA(owner: PublicKey): [PublicKey, number];
/**
 * 查找策略配置PDA
 * @param userProfilePda 用户配置PDA
 * @param strategyCounter 策略计数器
 * @returns [PDA, bump]
 */
export declare function findStrategyPDA(userProfilePda: PublicKey, strategyCounter: number): [PublicKey, number];
/**
 * 查找头寸追踪PDA
 * @param strategyId 策略ID
 * @param protocol 协议名称
 * @param asset 资产名称
 * @returns [PDA, bump]
 */
export declare function findPositionTrackerPDA(strategyId: number, protocol: string, asset: string): [PublicKey, number];
