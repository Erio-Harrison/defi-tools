"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.AssetType = exports.ProtocolType = void 0;
// 协议类型枚举
var ProtocolType;
(function (ProtocolType) {
    ProtocolType[ProtocolType["Raydium"] = 0] = "Raydium";
    ProtocolType[ProtocolType["Solend"] = 1] = "Solend";
    ProtocolType[ProtocolType["Orca"] = 2] = "Orca";
    ProtocolType[ProtocolType["Mango"] = 3] = "Mango";
})(ProtocolType = exports.ProtocolType || (exports.ProtocolType = {}));
// 资产类型枚举
var AssetType;
(function (AssetType) {
    AssetType[AssetType["SOL"] = 0] = "SOL";
    AssetType[AssetType["USDC"] = 1] = "USDC";
    AssetType[AssetType["USDT"] = 2] = "USDT";
})(AssetType = exports.AssetType || (exports.AssetType = {}));
