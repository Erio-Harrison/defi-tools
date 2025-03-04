use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("操作未授权")]
    Unauthorized,

    #[msg("无效的风险等级，必须在1-5范围内")]
    InvalidRiskLevel,

    #[msg("无效的策略ID")]
    InvalidStrategyId,

    #[msg("无效的资产分配，总权重必须为10000基点(100%)")]
    InvalidAllocation,

    #[msg("无效的滑点设置，必须在0-1000基点(0-10%)范围内")]
    InvalidSlippage,

    #[msg("策略已暂停")]
    StrategyPaused,

    #[msg("资金不足")]
    InsufficientFunds,

    #[msg("交易滑点超过最大允许值")]
    SlippageExceeded,

    #[msg("协议不兼容")]
    IncompatibleProtocol,

    #[msg("协议未注册")]
    ProtocolNotRegistered,

    #[msg("操作超时")]
    OperationTimeout,

    #[msg("数学错误")]
    MathError,

    #[msg("头寸不存在")]
    PositionNotFound,

    #[msg("再平衡条件未满足")]
    RebalanceConditionNotMet,

    #[msg("紧急模式已激活")]
    EmergencyModeActive,
}
