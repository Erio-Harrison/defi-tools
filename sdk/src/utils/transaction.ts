import { 
  Connection, 
  Transaction, 
  sendAndConfirmTransaction, 
  Signer,
  TransactionSignature
} from '@solana/web3.js';

/**
 * 发送并确认交易，支持自动重试
 * @param connection Solana连接
 * @param transaction 交易对象
 * @param signers 签名者数组
 * @param maxRetries 最大重试次数
 * @returns 交易签名
 */
export async function sendAndConfirmTransactionWithRetry(
  connection: Connection,
  transaction: Transaction,
  signers: Signer[],
  maxRetries: number = 3
): Promise<TransactionSignature> {
  let retries = 0;
  let lastError: Error | null = null;
  
  while (retries < maxRetries) {
    try {
      // 获取最新的区块哈希
      const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = signers[0].publicKey;

      // 发送交易
      const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        signers,
        {
          commitment: 'confirmed',
          maxRetries: 3
        }
      );
      return signature;
    } catch (error) {
      lastError = error as Error;
      console.error(`交易失败，正在重试 (${retries + 1}/${maxRetries})`, error);
      retries++;
      
      if (retries >= maxRetries) {
        break;
      }
      
      // 等待一段时间再重试
      await new Promise(resolve => setTimeout(resolve, 1000 * retries));
    }
  }
  
  throw lastError || new Error('交易发送失败，已达到最大重试次数');
}

/**
 * 等待交易确认
 * @param connection Solana连接
 * @param signature 交易签名
 * @param timeout 超时时间（毫秒）
 * @returns 成功状态
 */
export async function waitForTransaction(
  connection: Connection,
  signature: string,
  timeout: number = 60000
): Promise<boolean> {
  let timeoutId: NodeJS.Timeout | undefined;
  
  const promise = new Promise<boolean>((resolve, reject) => {
    const checkSignature = async () => {
      try {
        const signatureStatus = await connection.getSignatureStatus(signature);
        
        if (signatureStatus.value !== null) {
          if (signatureStatus.value.err) {
            reject(new Error(`交易 ${signature} 失败: ${JSON.stringify(signatureStatus.value.err)}`));
          } else {
            resolve(true);
          }
          return;
        }
        
        // 继续检查
        setTimeout(checkSignature, 1000);
      } catch (error) {
        reject(error);
      }
    };
    
    checkSignature();
    
    // 设置超时
    timeoutId = setTimeout(() => {
      reject(new Error(`等待交易确认超时: ${signature}`));
    }, timeout);
  });
  
  try {
    return await promise;
  } finally {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
  }
}
