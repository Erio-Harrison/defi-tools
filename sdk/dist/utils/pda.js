"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.findPositionTrackerPDA = exports.findStrategyPDA = exports.findUserProfilePDA = exports.PROGRAM_ID = void 0;
const web3_js_1 = require("@solana/web3.js");
const bn_js_1 = __importDefault(require("bn.js"));
// 程序ID：应该与您的部署ID匹配
exports.PROGRAM_ID = new web3_js_1.PublicKey('CdH2ymLMr7RyYcd1nyDZm59DRv6JgrtzuAxoH7STFvnm');
/**
 * 查找用户配置PDA
 * @param owner 用户钱包地址
 * @returns [PDA, bump]
 */
function findUserProfilePDA(owner) {
    return web3_js_1.PublicKey.findProgramAddressSync([Buffer.from('user'), owner.toBuffer()], exports.PROGRAM_ID);
}
exports.findUserProfilePDA = findUserProfilePDA;
/**
 * 查找策略配置PDA
 * @param userProfilePda 用户配置PDA
 * @param strategyCounter 策略计数器
 * @returns [PDA, bump]
 */
function findStrategyPDA(userProfilePda, strategyCounter) {
    return web3_js_1.PublicKey.findProgramAddressSync([
        Buffer.from('strategy'),
        userProfilePda.toBuffer(),
        new bn_js_1.default(strategyCounter).toArrayLike(Buffer, 'le', 8)
    ], exports.PROGRAM_ID);
}
exports.findStrategyPDA = findStrategyPDA;
/**
 * 查找头寸追踪PDA
 * @param strategyId 策略ID
 * @param protocol 协议名称
 * @param asset 资产名称
 * @returns [PDA, bump]
 */
function findPositionTrackerPDA(strategyId, protocol, asset) {
    return web3_js_1.PublicKey.findProgramAddressSync([
        Buffer.from('position'),
        new bn_js_1.default(strategyId).toArrayLike(Buffer, 'le', 8),
        Buffer.from(protocol),
        Buffer.from(asset)
    ], exports.PROGRAM_ID);
}
exports.findPositionTrackerPDA = findPositionTrackerPDA;
