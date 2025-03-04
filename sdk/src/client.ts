// client.ts
import { 
    Connection, 
    PublicKey, 
    SystemProgram,
    Transaction 
  } from '@solana/web3.js';
  import * as anchor from '@coral-xyz/anchor';
  import { Program } from '@coral-xyz/anchor';
  import BN from 'bn.js';
  
  import { 
    findUserProfilePDA, 
    findStrategyPDA, 
    PROGRAM_ID 
  } from './utils/pda';
  import { sendAndConfirmTransactionWithRetry, waitForTransaction } from './utils/transaction';
  import { 
    UserProfileData, 
    StrategyConfigData, 
    PositionTrackerData,
    CreateStrategyParams,
    ExecuteStrategyParams,
    FundsOperationParams
  } from './types';
  
  // 导入IDL
  import idl from './idl/defi_tools.json';
  
  // 定义程序类型接口，用于访问账户
  interface DefiToolsProgram extends Program<any> {
    account: {
      userProfile: { 
        fetch(address: PublicKey): Promise<UserProfileData>; 
      };
      strategyConfig: {
        fetch(address: PublicKey): Promise<StrategyConfigData>;
      };
      positionTracker: {
        fetch(address: PublicKey): Promise<PositionTrackerData>;
      };
    };
  }
  
  export class DefiToolsClient {
    private program: DefiToolsProgram;
    private connection: Connection;
    private wallet: anchor.Wallet;
  
    /**
     * 创建DeFi工具集客户端
     * @param connection Solana连接
     * @param wallet Anchor钱包适配器
     */
    constructor(
      connection: Connection,
      wallet: anchor.Wallet
    ) {
      this.connection = connection;
      this.wallet = wallet;
      
      // 初始化程序并指定正确的类型
      const provider = new anchor.AnchorProvider(
        connection,
        wallet,
        { commitment: 'confirmed' }
      );
      this.program = new Program(idl as any, provider) as DefiToolsProgram;
    }
  
    /**
     * 初始化用户配置
     * @param riskLevel 风险等级 (1-5)
     * @returns 交易签名
     */
    async initializeUser(riskLevel: number): Promise<string> {
        if (riskLevel < 1 || riskLevel > 5) {
          throw new Error("风险等级必须在1-5之间");
        }
      
        try {
          const [userProfilePda] = findUserProfilePDA(this.wallet.publicKey);
      
          // 关键修改：强制类型断言
          const tx = await (this.program.methods
            .initializeUser(riskLevel) as any)
            .accounts({
              owner: this.wallet.publicKey,
              userProfile: userProfilePda,
              systemProgram: SystemProgram.programId
            })
            .transaction();
      
          const signature = await sendAndConfirmTransactionWithRetry(
            this.connection,
            tx,
            this.wallet.payer ? [this.wallet.payer as anchor.web3.Keypair] : []
          );
      
          await waitForTransaction(this.connection, signature);
          return signature;
        } catch (error) {
          console.error("初始化用户失败:", error);
          throw error;
        }
      }
  
    /**
     * 创建投资策略
     * @param params 策略参数
     * @returns 交易签名
     */
    async createStrategy(params: CreateStrategyParams): Promise<string> {
        try {
          const [userProfilePda] = findUserProfilePDA(this.wallet.publicKey);
          const userProfile = await this.getUserProfile(this.wallet.publicKey);
          const [strategyPda] = findStrategyPDA(userProfilePda, userProfile.strategyCounter);
      
          // 关键修复步骤：分步断言
          const method = (this.program.methods as any) // 第一步：强制断言 methods 为 any
            .createStrategy(
              params.allocations,
              params.rebalanceCondition,
              params.maxSlippageBps
            );
      
          const accounts = method.accounts({
            owner: this.wallet.publicKey,
            userProfile: userProfilePda,
            strategyConfig: strategyPda,
            systemProgram: SystemProgram.programId
          });
      
          const tx: Transaction = await accounts.transaction(); // 第二步：显式声明类型
      
          const signature = await sendAndConfirmTransactionWithRetry(
            this.connection,
            tx,
            [this.wallet.payer as anchor.web3.Keypair]
          );
      
          await waitForTransaction(this.connection, signature);
          return signature;
        } catch (error) {
          console.error("创建策略失败:", error);
          throw error;
        }
      }
      
/**
 * 执行策略
 * @param params 执行参数
 * @returns 交易签名
 */
async executeStrategy(params: ExecuteStrategyParams): Promise<string> {
    try {
      const [userProfilePda] = findUserProfilePDA(this.wallet.publicKey);
      const [strategyPda] = findStrategyPDA(userProfilePda, params.strategyId);
  
      // 强制断言链式调用
      const tx = await (this.program.methods as any)
        .executeStrategy(new BN(params.strategyId))
        .accounts({
          owner: this.wallet.publicKey,
          userProfile: userProfilePda,
          strategyConfig: strategyPda,
          systemProgram: SystemProgram.programId
        })
        .transaction();
  
      const signature = await sendAndConfirmTransactionWithRetry(
        this.connection,
        tx as Transaction, // 最终断言为 Transaction 类型
        [this.wallet.payer as anchor.web3.Keypair]
      );
  
      await waitForTransaction(this.connection, signature);
      return signature;
    } catch (error) {
      console.error("执行策略失败:", error);
      throw error;
    }
  }
  
  /**
   * 存入资金
   * @param params 存款参数
   * @returns 交易签名
   */
  async depositFunds(params: FundsOperationParams): Promise<string> {
    try {
      const [userProfilePda] = findUserProfilePDA(this.wallet.publicKey);
      const [strategyPda] = findStrategyPDA(userProfilePda, params.strategyId);
  
      // 双断言策略：强制方法链类型 + 最终交易类型
      const tx = await (this.program.methods as unknown as {
        depositFunds: (...args: any[]) => {
          accounts: (...args: any[]) => { transaction: () => Promise<Transaction> }
        }
      }).depositFunds(new BN(params.strategyId), new BN(params.amount))
        .accounts({
          owner: this.wallet.publicKey,
          userProfile: userProfilePda,
          strategyConfig: strategyPda,
          systemProgram: SystemProgram.programId
        })
        .transaction();
  
      const signature = await sendAndConfirmTransactionWithRetry(
        this.connection,
        tx,
        [this.wallet.payer as anchor.web3.Keypair]
      );
  
      await waitForTransaction(this.connection, signature);
      return signature;
    } catch (error) {
      console.error("存入资金失败:", error);
      throw error;
    }
  }

    /**
     * 提取资金
     * @param params 提款参数
     * @returns 交易签名
     */
    async withdrawFunds(params: FundsOperationParams): Promise<string> {
        try {
        const [userProfilePda] = findUserProfilePDA(this.wallet.publicKey);
        const [strategyPda] = findStrategyPDA(userProfilePda, params.strategyId);
    
        // 双断言策略：强制方法链类型 + 最终交易类型
        const tx = await (this.program.methods as unknown as {
            withdrawFunds: (...args: any[]) => {
            accounts: (...args: any[]) => { transaction: () => Promise<Transaction> };
            };
        }).withdrawFunds(new BN(params.strategyId), new BN(params.amount))
            .accounts({
            owner: this.wallet.publicKey,
            userProfile: userProfilePda,
            strategyConfig: strategyPda,
            systemProgram: SystemProgram.programId,
            })
            .transaction();
    
        const signature = await sendAndConfirmTransactionWithRetry(
            this.connection,
            tx,
            [this.wallet.payer as anchor.web3.Keypair]
        );
    
        await waitForTransaction(this.connection, signature);
        return signature;
        } catch (error) {
        console.error("提取资金失败:", error);
        throw error;
        }
    }
  
    /**
     * 再平衡头寸
     * @param strategyId 策略ID
     * @returns 交易签名
     */
    async rebalancePositions(strategyId: number): Promise<string> {
        try {
        const [userProfilePda] = findUserProfilePDA(this.wallet.publicKey);
        const [strategyPda] = findStrategyPDA(userProfilePda, strategyId);
    
        // 双断言策略：强制方法链类型 + 最终交易类型
        const tx = await (this.program.methods as unknown as {
            rebalancePositions: (...args: any[]) => {
            accounts: (...args: any[]) => { transaction: () => Promise<Transaction> };
            };
        }).rebalancePositions(new BN(strategyId))
            .accounts({
            owner: this.wallet.publicKey,
            userProfile: userProfilePda,
            strategyConfig: strategyPda,
            systemProgram: SystemProgram.programId,
            })
            .transaction();
    
        const signature = await sendAndConfirmTransactionWithRetry(
            this.connection,
            tx,
            [this.wallet.payer as anchor.web3.Keypair]
        );
    
        await waitForTransaction(this.connection, signature);
        return signature;
        } catch (error) {
        console.error("再平衡头寸失败:", error);
        throw error;
        }
    }
  
    /**
     * 获取用户配置数据
     * @param ownerAddress 用户地址
     * @returns 用户配置数据
     */
    async getUserProfile(ownerAddress: PublicKey): Promise<UserProfileData> {
        try {
        const [userProfilePda] = findUserProfilePDA(ownerAddress);
        const userProfile = await this.program.account.userProfile.fetch(userProfilePda);
        
        return {
            owner: userProfile.owner,
            riskLevel: userProfile.riskLevel,
            strategyCounter: userProfile.strategyCounter,
            vaultBump: userProfile.vaultBump,
            lastActivity: userProfile.lastActivity,
            totalValueLamports: userProfile.totalValueLamports,
            isPaused: userProfile.isPaused
        };
        } catch (error) {
        console.error("获取用户配置失败:", error);
        throw error;
        }
    }

    /**
     * 获取策略配置数据
     * @param userProfilePda 用户配置PDA
     * @param strategyId 策略ID
     * @returns 策略配置数据
     */
    async getStrategyConfig(userProfilePda: PublicKey, strategyId: number): Promise<StrategyConfigData> {
        try {
        const [strategyPda] = findStrategyPDA(userProfilePda, strategyId);
        const strategyConfig = await this.program.account.strategyConfig.fetch(strategyPda);
        
        return {
            owner: strategyConfig.owner,
            strategyId: strategyConfig.strategyId,
            allocations: strategyConfig.allocations.map((a: any) => ({
            protocol: a.protocol,
            asset: a.asset,
            targetWeightBps: a.targetWeightBps
            })),
            rebalanceCondition: {
            timeIntervalSeconds: strategyConfig.rebalanceCondition.timeIntervalSeconds,
            maxDeviationBps: strategyConfig.rebalanceCondition.maxDeviationBps,
            autoRebalance: strategyConfig.rebalanceCondition.autoRebalance
            },
            createdAt: strategyConfig.createdAt,
            lastExecutedAt: strategyConfig.lastExecutedAt,
            maxSlippageBps: strategyConfig.maxSlippageBps
        };
        } catch (error) {
        console.error("获取策略配置失败:", error);
        throw error;
        }
    }
    /**
     * 获取用户所有策略
     * @param ownerAddress 用户地址
     * @returns 策略配置数据数组
     */
    async getAllStrategies(ownerAddress: PublicKey): Promise<StrategyConfigData[]> {
      try {
        // 获取用户配置
        const userProfile = await this.getUserProfile(ownerAddress);
        const [userProfilePda] = findUserProfilePDA(ownerAddress);
        
        // 获取所有策略
        const strategies: StrategyConfigData[] = [];
        for (let i = 0; i < userProfile.strategyCounter; i++) {
          try {
            const strategy = await this.getStrategyConfig(userProfilePda, i);
            strategies.push(strategy);
          } catch (error) {
            console.warn(`获取策略 ${i} 失败:`, error);
            // 继续获取下一个策略
          }
        }
        
        return strategies;
      } catch (error) {
        console.error("获取所有策略失败:", error);
        throw error;
      }
    }
  
    /**
     * 检查用户配置是否已初始化
     * @param ownerAddress 用户地址
     * @returns 是否已初始化
     */
    async isUserInitialized(ownerAddress: PublicKey): Promise<boolean> {
      try {
        const [userProfilePda] = findUserProfilePDA(ownerAddress);
        await this.program.account.userProfile.fetch(userProfilePda);
        return true;
      } catch (error) {
        return false;
      }
    }
  }