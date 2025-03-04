import { Connection, Transaction, Signer, TransactionSignature } from '@solana/web3.js';
/**
 * 发送并确认交易，支持自动重试
 * @param connection Solana连接
 * @param transaction 交易对象
 * @param signers 签名者数组
 * @param maxRetries 最大重试次数
 * @returns 交易签名
 */
export declare function sendAndConfirmTransactionWithRetry(connection: Connection, transaction: Transaction, signers: Signer[], maxRetries?: number): Promise<TransactionSignature>;
/**
 * 等待交易确认
 * @param connection Solana连接
 * @param signature 交易签名
 * @param timeout 超时时间（毫秒）
 * @returns 成功状态
 */
export declare function waitForTransaction(connection: Connection, signature: string, timeout?: number): Promise<boolean>;
