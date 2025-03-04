'use client';

import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { useState, useCallback } from 'react';
import * as anchor from '@coral-xyz/anchor';
import { 
  CreateStrategyParams, 
  DefiToolsClient, 
  ExecuteStrategyParams, 
  FundsOperationParams, 
  StrategyConfigData, 
  UserProfileData,
  findUserProfilePDA, 
  findStrategyPDA
} from 'defi-tools-sdk';

export function useDefiTools() {
  const { connection } = useConnection();
  const wallet = useWallet();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  // 创建客户端实例
  const getClient = useCallback(() => {
    if (!wallet.publicKey || !wallet.signTransaction || !wallet.signAllTransactions) {
      throw new Error('钱包未连接');
    }
  
    // Define a wallet adapter compatible with Anchor
    const anchorWallet = {
      publicKey: wallet.publicKey,
      signTransaction: wallet.signTransaction,
      signAllTransactions: wallet.signAllTransactions,
    };
  
    // Pass the wallet to DefiToolsClient
    return new DefiToolsClient(connection, anchorWallet as anchor.Wallet);
  }, [connection, wallet]);

  // 初始化用户
  const initializeUser = useCallback(async (riskLevel: number) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const txId = await client.initializeUser(riskLevel);
      setIsLoading(false);
      return txId;
    } catch (err) {
      console.error('初始化用户失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient]);

  // 检查用户是否已初始化
  const checkUserInitialized = useCallback(async () => {
    if (!wallet.publicKey) return false;
    
    try {
      const client = getClient();
      return await client.isUserInitialized(wallet.publicKey);
    } catch (err) {
      console.error('检查用户状态失败:', err);
      return false;
    }
  }, [getClient, wallet.publicKey]);

  // 创建策略
  const createStrategy = useCallback(async (params: CreateStrategyParams) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const txId = await client.createStrategy(params);
      setIsLoading(false);
      return txId;
    } catch (err) {
      console.error('创建策略失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient]);

  // 执行策略
  const executeStrategy = useCallback(async (params: ExecuteStrategyParams) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const txId = await client.executeStrategy(params);
      setIsLoading(false);
      return txId;
    } catch (err) {
      console.error('执行策略失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient]);

  // 存入资金
  const depositFunds = useCallback(async (params: FundsOperationParams) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const txId = await client.depositFunds(params);
      setIsLoading(false);
      return txId;
    } catch (err) {
      console.error('存入资金失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient]);

  // 提取资金
  const withdrawFunds = useCallback(async (params: FundsOperationParams) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const txId = await client.withdrawFunds(params);
      setIsLoading(false);
      return txId;
    } catch (err) {
      console.error('提取资金失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient]);

  // 再平衡头寸
  const rebalancePositions = useCallback(async (strategyId: number) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const txId = await client.rebalancePositions(strategyId);
      setIsLoading(false);
      return txId;
    } catch (err) {
      console.error('再平衡头寸失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient]);

  // 获取用户配置
  const getUserProfile = useCallback(async () => {
    if (!wallet.publicKey) return null;
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const userProfile = await client.getUserProfile(wallet.publicKey);
      setIsLoading(false);
      return userProfile;
    } catch (err) {
      console.error('获取用户配置失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient, wallet.publicKey]);

  // 获取策略配置
  const getStrategyConfig = useCallback(async (strategyId: number) => {
    if (!wallet.publicKey) return null;
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const [userProfilePda] = findUserProfilePDA(wallet.publicKey);
      const strategyConfig = await client.getStrategyConfig(userProfilePda, strategyId);
      setIsLoading(false);
      return strategyConfig;
    } catch (err) {
      console.error('获取策略配置失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient, wallet.publicKey]);

  // 获取用户所有策略
  const getAllStrategies = useCallback(async () => {
    if (!wallet.publicKey) return [];
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      const strategies = await client.getAllStrategies(wallet.publicKey);
      setIsLoading(false);
      return strategies;
    } catch (err) {
      console.error('获取所有策略失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return [];
    }
  }, [getClient, wallet.publicKey]);
  
  // 监控策略性能
  const monitorStrategyPerformance = useCallback(async (strategyId: number) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = getClient();
      // 实现策略性能监控逻辑
      // 这可能需要从链上获取策略数据，然后计算性能指标
      const strategy = await getStrategyConfig(strategyId);
      if (!strategy) {
        throw new Error('策略不存在');
      }
      
      // 假设我们需要计算一些性能指标
      // 这里只是一个示例，实际实现可能需要更多数据
      const performance = {
        strategyId,
        currentValue: 0, // 这需要实际计算
        initialValue: 0, // 这需要从历史数据中获取
        roi: 0, // 需要计算
        lastRebalanced: strategy.lastExecutedAt,
        portfolioHealth: 'healthy' // 基于某些条件判断
      };
      
      setIsLoading(false);
      return performance;
    } catch (err) {
      console.error('监控策略性能失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getClient, getStrategyConfig]);

  // 获取市场数据
  const getMarketData = useCallback(async (assetIds: number[]) => {
    setIsLoading(true);
    setError(null);
    
    try {
      // 这里我们可能需要调用外部API来获取市场数据
      // 这只是一个模拟示例
      const marketData = assetIds.map(id => ({
        assetId: id,
        price: Math.random() * 1000, // 模拟价格
        volume24h: Math.random() * 1000000,
        change24h: (Math.random() - 0.5) * 10,
        timestamp: Date.now()
      }));
      
      setIsLoading(false);
      return marketData;
    } catch (err) {
      console.error('获取市场数据失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return [];
    }
  }, []);

  // 分析用户组合风险
  const analyzePortfolioRisk = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const strategies = await getAllStrategies();
      if (strategies.length === 0) {
        return { riskScore: 0, riskLevel: 'none', recommendations: [] };
      }
      
      // 模拟风险分析计算
      // 在实际应用中，这应该基于实际数据和算法
      const allocations = strategies.flatMap(s => s.allocations);
      const riskScore = Math.min(100, Math.max(0, Math.random() * 100));
      
      let riskLevel;
      if (riskScore < 20) riskLevel = 'very low';
      else if (riskScore < 40) riskLevel = 'low';
      else if (riskScore < 60) riskLevel = 'moderate';
      else if (riskScore < 80) riskLevel = 'high';
      else riskLevel = 'very high';
      
      const recommendations = [
        '考虑在投资组合中增加稳定币比例',
        '减少单一协议的风险暴露',
        '定期对投资组合进行再平衡'
      ];
      
      setIsLoading(false);
      return { riskScore, riskLevel, recommendations };
    } catch (err) {
      console.error('分析组合风险失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [getAllStrategies]);

  // 更新用户风险偏好
  const updateRiskPreference = useCallback(async (newRiskLevel: number) => {
    setIsLoading(true);
    setError(null);
    
    try {
      // 注意：当前DefiToolsClient中没有直接提供更新风险偏好的方法
      // 这里我们可以先重新初始化用户来更新风险等级
      // 在实际项目中，应该在SDK中增加对应的方法
      const txId = await initializeUser(newRiskLevel);
      setIsLoading(false);
      return txId;
    } catch (err) {
      console.error('更新风险偏好失败:', err);
      setError(err instanceof Error ? err : new Error(String(err)));
      setIsLoading(false);
      return null;
    }
  }, [initializeUser]);

  return {
    isLoading,
    error,
    isConnected: !!wallet.publicKey,
    // 用户管理
    initializeUser,
    checkUserInitialized,
    getUserProfile,
    updateRiskPreference,
    
    // 策略管理
    createStrategy,
    executeStrategy,
    getStrategyConfig,
    getAllStrategies,
    monitorStrategyPerformance,
    
    // 资金操作
    depositFunds,
    withdrawFunds,
    rebalancePositions,
    
    // 分析工具
    getMarketData,
    analyzePortfolioRisk
  };
}