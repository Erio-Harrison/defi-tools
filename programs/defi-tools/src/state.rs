use anchor_lang::prelude::*;

// 用户配置文件账户
#[account]
pub struct UserProfile {
    // 用户钱包地址
    pub owner: Pubkey,              
    // 风险偏好 (1-5，5为最高风险容忍度)
    pub risk_level: u8,             
    // 策略计数器(用于生成唯一ID)
    pub strategy_counter: u64,      
    // 资金库PDA的bump种子
    pub vault_bump: u8,             
    // 上次操作时间戳
    pub last_activity: i64,         
    // 总资产价值 (lamports)
    pub total_value_lamports: u64,  
    // 是否暂停(紧急状态)
    pub is_paused: bool,            
}

// 策略配置账户
#[account]
pub struct StrategyConfig {
    // 关联用户
    pub owner: Pubkey,              
    // 策略唯一ID
    pub strategy_id: u64,           
    // 资产分配(协议,资产,权重)
    pub allocations: Vec<Allocation>,
    // 再平衡触发条件
    pub rebalance_condition: RebalanceCondition,
    // 创建时间戳
    pub created_at: i64,            
    // 最后执行时间戳
    pub last_executed_at: i64,      
    // 最大允许滑点(基点)
    pub max_slippage_bps: u16,      
}

// 头寸追踪账户
#[account]
pub struct PositionTracker {
    // 关联策略ID
    pub strategy_id: u64,           
    // 协议名称
    pub protocol: [u8; 32],         
    // 资产类型
    pub asset: [u8; 32],            
    // 头寸价值(lamports)
    pub position_value_lamports: u64,
    // 初始APY(基点)
    pub initial_apy_bps: u32,        
    // 最新APY(基点)
    pub current_apy_bps: u32,        
    // 最后更新时间戳
    pub last_updated_at: i64,        
}

// 协议注册表
#[account]
pub struct ProtocolRegistry {
    // 协议名称
    pub name: [u8; 32],             
    // 协议程序ID
    pub program_id: Pubkey,         
    // 协议版本
    pub version: [u8; 16],          
    // 最小兼容版本
    pub min_compatible_version: [u8; 16],
    // 活跃状态
    pub active: bool,               
    // 风险评分(0-100)
    pub risk_score: u8,             
}

// 资产分配项
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Allocation {
    // 协议类型枚举
    pub protocol: u8,              
    // 资产类型枚举
    pub asset: u8,                 
    // 目标权重(基点,总和应为10000)
    pub target_weight_bps: u16,    
}

// 再平衡条件
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RebalanceCondition {
    // 最小时间间隔(秒)
    pub time_interval_seconds: u64, 
    // 最大允许偏差(基点)
    pub max_deviation_bps: u16,    
    // 是否自动执行
    pub auto_rebalance: bool,      
}

// 协议类型枚举(采用u8表示)
pub enum Protocol {
    Raydium = 0,
    Solend = 1,
    Orca = 2,
    Mango = 3,
}

// 资产类型枚举(采用u8表示)
pub enum Asset {
    SOL = 0,
    USDC = 1,
    USDT = 2,
}