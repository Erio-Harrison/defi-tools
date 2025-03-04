"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.DefiToolsClient = void 0;
// client.ts
const web3_js_1 = require("@solana/web3.js");
const anchor = __importStar(require("@coral-xyz/anchor"));
const anchor_1 = require("@coral-xyz/anchor");
const bn_js_1 = __importDefault(require("bn.js"));
const pda_1 = require("./utils/pda");
const transaction_1 = require("./utils/transaction");
// 导入IDL
const defi_tools_json_1 = __importDefault(require("./idl/defi_tools.json"));
class DefiToolsClient {
    /**
     * 创建DeFi工具集客户端
     * @param connection Solana连接
     * @param wallet Anchor钱包适配器
     */
    constructor(connection, wallet) {
        this.connection = connection;
        this.wallet = wallet;
        // 初始化程序并指定正确的类型
        const provider = new anchor.AnchorProvider(connection, wallet, { commitment: 'confirmed' });
        this.program = new anchor_1.Program(defi_tools_json_1.default, provider);
    }
    /**
     * 初始化用户配置
     * @param riskLevel 风险等级 (1-5)
     * @returns 交易签名
     */
    async initializeUser(riskLevel) {
        if (riskLevel < 1 || riskLevel > 5) {
            throw new Error("风险等级必须在1-5之间");
        }
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(this.wallet.publicKey);
            // 关键修改：强制类型断言
            const tx = await this.program.methods
                .initializeUser(riskLevel)
                .accounts({
                owner: this.wallet.publicKey,
                userProfile: userProfilePda,
                systemProgram: web3_js_1.SystemProgram.programId
            })
                .transaction();
            const signature = await (0, transaction_1.sendAndConfirmTransactionWithRetry)(this.connection, tx, this.wallet.payer ? [this.wallet.payer] : []);
            await (0, transaction_1.waitForTransaction)(this.connection, signature);
            return signature;
        }
        catch (error) {
            console.error("初始化用户失败:", error);
            throw error;
        }
    }
    /**
     * 创建投资策略
     * @param params 策略参数
     * @returns 交易签名
     */
    async createStrategy(params) {
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(this.wallet.publicKey);
            const userProfile = await this.getUserProfile(this.wallet.publicKey);
            const [strategyPda] = (0, pda_1.findStrategyPDA)(userProfilePda, userProfile.strategyCounter);
            // 关键修复步骤：分步断言
            const method = this.program.methods // 第一步：强制断言 methods 为 any
                .createStrategy(params.allocations, params.rebalanceCondition, params.maxSlippageBps);
            const accounts = method.accounts({
                owner: this.wallet.publicKey,
                userProfile: userProfilePda,
                strategyConfig: strategyPda,
                systemProgram: web3_js_1.SystemProgram.programId
            });
            const tx = await accounts.transaction(); // 第二步：显式声明类型
            const signature = await (0, transaction_1.sendAndConfirmTransactionWithRetry)(this.connection, tx, [this.wallet.payer]);
            await (0, transaction_1.waitForTransaction)(this.connection, signature);
            return signature;
        }
        catch (error) {
            console.error("创建策略失败:", error);
            throw error;
        }
    }
    /**
     * 执行策略
     * @param params 执行参数
     * @returns 交易签名
     */
    async executeStrategy(params) {
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(this.wallet.publicKey);
            const [strategyPda] = (0, pda_1.findStrategyPDA)(userProfilePda, params.strategyId);
            // 强制断言链式调用
            const tx = await this.program.methods
                .executeStrategy(new bn_js_1.default(params.strategyId))
                .accounts({
                owner: this.wallet.publicKey,
                userProfile: userProfilePda,
                strategyConfig: strategyPda,
                systemProgram: web3_js_1.SystemProgram.programId
            })
                .transaction();
            const signature = await (0, transaction_1.sendAndConfirmTransactionWithRetry)(this.connection, tx, // 最终断言为 Transaction 类型
            [this.wallet.payer]);
            await (0, transaction_1.waitForTransaction)(this.connection, signature);
            return signature;
        }
        catch (error) {
            console.error("执行策略失败:", error);
            throw error;
        }
    }
    /**
     * 存入资金
     * @param params 存款参数
     * @returns 交易签名
     */
    async depositFunds(params) {
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(this.wallet.publicKey);
            const [strategyPda] = (0, pda_1.findStrategyPDA)(userProfilePda, params.strategyId);
            // 双断言策略：强制方法链类型 + 最终交易类型
            const tx = await this.program.methods.depositFunds(new bn_js_1.default(params.strategyId), new bn_js_1.default(params.amount))
                .accounts({
                owner: this.wallet.publicKey,
                userProfile: userProfilePda,
                strategyConfig: strategyPda,
                systemProgram: web3_js_1.SystemProgram.programId
            })
                .transaction();
            const signature = await (0, transaction_1.sendAndConfirmTransactionWithRetry)(this.connection, tx, [this.wallet.payer]);
            await (0, transaction_1.waitForTransaction)(this.connection, signature);
            return signature;
        }
        catch (error) {
            console.error("存入资金失败:", error);
            throw error;
        }
    }
    /**
     * 提取资金
     * @param params 提款参数
     * @returns 交易签名
     */
    async withdrawFunds(params) {
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(this.wallet.publicKey);
            const [strategyPda] = (0, pda_1.findStrategyPDA)(userProfilePda, params.strategyId);
            // 双断言策略：强制方法链类型 + 最终交易类型
            const tx = await this.program.methods.withdrawFunds(new bn_js_1.default(params.strategyId), new bn_js_1.default(params.amount))
                .accounts({
                owner: this.wallet.publicKey,
                userProfile: userProfilePda,
                strategyConfig: strategyPda,
                systemProgram: web3_js_1.SystemProgram.programId,
            })
                .transaction();
            const signature = await (0, transaction_1.sendAndConfirmTransactionWithRetry)(this.connection, tx, [this.wallet.payer]);
            await (0, transaction_1.waitForTransaction)(this.connection, signature);
            return signature;
        }
        catch (error) {
            console.error("提取资金失败:", error);
            throw error;
        }
    }
    /**
     * 再平衡头寸
     * @param strategyId 策略ID
     * @returns 交易签名
     */
    async rebalancePositions(strategyId) {
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(this.wallet.publicKey);
            const [strategyPda] = (0, pda_1.findStrategyPDA)(userProfilePda, strategyId);
            // 双断言策略：强制方法链类型 + 最终交易类型
            const tx = await this.program.methods.rebalancePositions(new bn_js_1.default(strategyId))
                .accounts({
                owner: this.wallet.publicKey,
                userProfile: userProfilePda,
                strategyConfig: strategyPda,
                systemProgram: web3_js_1.SystemProgram.programId,
            })
                .transaction();
            const signature = await (0, transaction_1.sendAndConfirmTransactionWithRetry)(this.connection, tx, [this.wallet.payer]);
            await (0, transaction_1.waitForTransaction)(this.connection, signature);
            return signature;
        }
        catch (error) {
            console.error("再平衡头寸失败:", error);
            throw error;
        }
    }
    /**
     * 获取用户配置数据
     * @param ownerAddress 用户地址
     * @returns 用户配置数据
     */
    async getUserProfile(ownerAddress) {
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(ownerAddress);
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
        }
        catch (error) {
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
    async getStrategyConfig(userProfilePda, strategyId) {
        try {
            const [strategyPda] = (0, pda_1.findStrategyPDA)(userProfilePda, strategyId);
            const strategyConfig = await this.program.account.strategyConfig.fetch(strategyPda);
            return {
                owner: strategyConfig.owner,
                strategyId: strategyConfig.strategyId,
                allocations: strategyConfig.allocations.map((a) => ({
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
        }
        catch (error) {
            console.error("获取策略配置失败:", error);
            throw error;
        }
    }
    /**
     * 获取用户所有策略
     * @param ownerAddress 用户地址
     * @returns 策略配置数据数组
     */
    async getAllStrategies(ownerAddress) {
        try {
            // 获取用户配置
            const userProfile = await this.getUserProfile(ownerAddress);
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(ownerAddress);
            // 获取所有策略
            const strategies = [];
            for (let i = 0; i < userProfile.strategyCounter; i++) {
                try {
                    const strategy = await this.getStrategyConfig(userProfilePda, i);
                    strategies.push(strategy);
                }
                catch (error) {
                    console.warn(`获取策略 ${i} 失败:`, error);
                    // 继续获取下一个策略
                }
            }
            return strategies;
        }
        catch (error) {
            console.error("获取所有策略失败:", error);
            throw error;
        }
    }
    /**
     * 检查用户配置是否已初始化
     * @param ownerAddress 用户地址
     * @returns 是否已初始化
     */
    async isUserInitialized(ownerAddress) {
        try {
            const [userProfilePda] = (0, pda_1.findUserProfilePDA)(ownerAddress);
            await this.program.account.userProfile.fetch(userProfilePda);
            return true;
        }
        catch (error) {
            return false;
        }
    }
}
exports.DefiToolsClient = DefiToolsClient;
