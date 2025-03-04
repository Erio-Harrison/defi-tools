import { PublicKey } from '@solana/web3.js';
import BN from 'bn.js';

// 程序ID：应该与您的部署ID匹配
export const PROGRAM_ID = new PublicKey('CdH2ymLMr7RyYcd1nyDZm59DRv6JgrtzuAxoH7STFvnm');

/**
 * 查找用户配置PDA
 * @param owner 用户钱包地址
 * @returns [PDA, bump]
 */
export function findUserProfilePDA(owner: PublicKey): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('user'), owner.toBuffer()],
    PROGRAM_ID
  );
}

/**
 * 查找策略配置PDA
 * @param userProfilePda 用户配置PDA
 * @param strategyCounter 策略计数器
 * @returns [PDA, bump]
 */
export function findStrategyPDA(userProfilePda: PublicKey, strategyCounter: number): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from('strategy'),
      userProfilePda.toBuffer(),
      new BN(strategyCounter).toArrayLike(Buffer, 'le', 8)
    ],
    PROGRAM_ID
  );
}

/**
 * 查找头寸追踪PDA
 * @param strategyId 策略ID
 * @param protocol 协议名称
 * @param asset 资产名称
 * @returns [PDA, bump]
 */
export function findPositionTrackerPDA(strategyId: number, protocol: string, asset: string): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from('position'),
      new BN(strategyId).toArrayLike(Buffer, 'le', 8),
      Buffer.from(protocol),
      Buffer.from(asset)
    ],
    PROGRAM_ID
  );
}
