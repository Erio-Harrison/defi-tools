use anchor_lang::prelude::*;

pub mod state;

// 声明程序ID
declare_id!("CdH2ymLMr7RyYcd1nyDZm59DRv6JgrtzuAxoH7STFvnm");

// 状态定义
#[account]
pub struct UserProfile {
    pub owner: Pubkey,              
    pub risk_level: u8,             
    pub strategy_counter: u64,      
    pub vault_bump: u8,             
    pub last_activity: i64,         
    pub total_value_lamports: u64,  
    pub is_paused: bool,            
}

#[account]
pub struct StrategyConfig {
    pub owner: Pubkey,              
    pub strategy_id: u64,           
    pub allocations: Vec<Allocation>,
    pub rebalance_condition: RebalanceCondition,
    pub created_at: i64,            
    pub last_executed_at: i64,      
    pub max_slippage_bps: u16,      
}

#[account]
pub struct PositionTracker {
    pub strategy_id: u64,           
    pub protocol: [u8; 32],         
    pub asset: [u8; 32],            
    pub position_value_lamports: u64,
    pub initial_apy_bps: u32,        
    pub current_apy_bps: u32,        
    pub last_updated_at: i64,        
}

#[account]
pub struct ProtocolRegistry {
    pub name: [u8; 32],             
    pub program_id: Pubkey,         
    pub version: [u8; 16],          
    pub min_compatible_version: [u8; 16],
    pub active: bool,               
    pub risk_score: u8,             
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Allocation {
    pub protocol: u8,              
    pub asset: u8,                 
    pub target_weight_bps: u16,    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RebalanceCondition {
    pub time_interval_seconds: u64, 
    pub max_deviation_bps: u16,    
    pub auto_rebalance: bool,      
}

// 错误码定义
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

// 程序定义
#[program]
pub mod defi_tools {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("DeFi工具集已初始化: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, risk_level: u8) -> Result<()> {
        // 验证风险等级
        if risk_level < 1 || risk_level > 5 {
            return Err(ErrorCode::InvalidRiskLevel.into());
        }
        
        let user_profile = &mut ctx.accounts.user_profile;
        let clock = Clock::get()?;
        
        // 初始化用户配置文件
        user_profile.owner = ctx.accounts.owner.key();
        user_profile.risk_level = risk_level;
        user_profile.strategy_counter = 0;
        user_profile.vault_bump = ctx.bumps.user_profile;
        user_profile.last_activity = clock.unix_timestamp;
        user_profile.total_value_lamports = 0;
        user_profile.is_paused = false;
        
        msg!("用户配置文件已初始化，风险等级: {}", risk_level);
        
        Ok(())
    }

    pub fn create_strategy(
        ctx: Context<CreateStrategy>, 
        allocations: Vec<Allocation>,
        rebalance_condition: RebalanceCondition,
        max_slippage_bps: u16
    ) -> Result<()> {
        // 验证资产分配
        let mut total_weight = 0;
        for allocation in allocations.iter() {
            total_weight += allocation.target_weight_bps;
        }
        
        // 确保总权重为10000基点 (100%)
        if total_weight != 10000 {
            return Err(ErrorCode::InvalidAllocation.into());
        }
        
        // 验证滑点设置
        if max_slippage_bps > 1000 { // 最大10%
            return Err(ErrorCode::InvalidSlippage.into());
        }
        
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = Clock::get()?;
        
        // 设置策略配置
        strategy_config.owner = ctx.accounts.owner.key();
        strategy_config.strategy_id = user_profile.strategy_counter;
        strategy_config.allocations = allocations;
        strategy_config.rebalance_condition = rebalance_condition;
        strategy_config.created_at = clock.unix_timestamp;
        strategy_config.last_executed_at = 0;
        strategy_config.max_slippage_bps = max_slippage_bps;
        
        // 增加用户的策略计数器
        user_profile.strategy_counter += 1;
        user_profile.last_activity = clock.unix_timestamp;
        
        msg!("策略已创建，ID: {}", strategy_config.strategy_id);
        
        Ok(())
    }

    pub fn execute_strategy(ctx: Context<ExecuteStrategy>, strategy_id: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = Clock::get()?;
        
        // 更新最后执行时间
        strategy_config.last_executed_at = clock.unix_timestamp;
        user_profile.last_activity = clock.unix_timestamp;
        
        // 这里是实际执行策略的逻辑
        // 在实际应用中，这里会调用各种DeFi协议的交互逻辑
        
        msg!("策略已执行，ID: {}", strategy_id);
        
        Ok(())
    }
    
    pub fn deposit_funds(ctx: Context<DepositFunds>, strategy_id: u64, amount: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = Clock::get()?;
        
        // 验证存款金额
        if amount == 0 {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        
        // 更新用户活动时间
        user_profile.last_activity = clock.unix_timestamp;
        
        // 更新总资产价值
        user_profile.total_value_lamports = user_profile.total_value_lamports.checked_add(amount)
            .ok_or(ErrorCode::MathError)?;
        
        // 在实际应用中，这里会调用资金转移和存款的逻辑
        
        msg!("向策略存入资金，ID: {}，金额: {}", strategy_id, amount);
        
        Ok(())
    }
    
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, strategy_id: u64, amount: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let clock = Clock::get()?;
        
        // 验证提款金额
        if amount == 0 || amount > user_profile.total_value_lamports {
            return Err(ErrorCode::InsufficientFunds.into());
        }
        
        // 更新用户活动时间
        user_profile.last_activity = clock.unix_timestamp;
        
        // 更新总资产价值
        user_profile.total_value_lamports = user_profile.total_value_lamports.checked_sub(amount)
            .ok_or(ErrorCode::MathError)?;
        
        // 在实际应用中，这里会调用资金提取和转移的逻辑
        
        msg!("从策略提取资金，ID: {}，金额: {}", strategy_id, amount);
        
        Ok(())
    }
    
    pub fn rebalance_positions(ctx: Context<RebalancePositions>, strategy_id: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = Clock::get()?;
        
        // 验证再平衡条件
        let time_since_last_execution = clock.unix_timestamp - strategy_config.last_executed_at;
        if time_since_last_execution < strategy_config.rebalance_condition.time_interval_seconds as i64 {
            return Err(ErrorCode::RebalanceConditionNotMet.into());
        }
        
        // 更新最后执行时间
        strategy_config.last_executed_at = clock.unix_timestamp;
        user_profile.last_activity = clock.unix_timestamp;
        
        // 在实际应用中，这里会调用再平衡头寸的逻辑
        
        msg!("策略头寸已再平衡，ID: {}", strategy_id);
        
        Ok(())
    }
}

// 账户验证结构
#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        space = 8 + std::mem::size_of::<UserProfile>(),
        seeds = [b"user", owner.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(strategy_id: u64)]
pub struct ExecuteStrategy<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump,
        constraint = user_profile.owner == owner.key() @ ErrorCode::Unauthorized,
        constraint = !user_profile.is_paused @ ErrorCode::StrategyPaused
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [
            b"strategy", 
            user_profile.key().as_ref(),
            &strategy_id.to_le_bytes()
        ],
        bump,
        constraint = strategy_config.owner == owner.key() @ ErrorCode::Unauthorized
    )]
    pub strategy_config: Account<'info, StrategyConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateStrategy<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump,
        constraint = user_profile.owner == owner.key() @ ErrorCode::Unauthorized
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        init,
        payer = owner,
        space = 8 + std::mem::size_of::<StrategyConfig>() + 
                4 + (std::mem::size_of::<Allocation>() * 10), // 最多10个分配项
        seeds = [
            b"strategy", 
            user_profile.key().as_ref(),
            &user_profile.strategy_counter.to_le_bytes()
        ],
        bump
    )]
    pub strategy_config: Account<'info, StrategyConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(strategy_id: u64)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump,
        constraint = user_profile.owner == owner.key() @ ErrorCode::Unauthorized,
        constraint = !user_profile.is_paused @ ErrorCode::StrategyPaused
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [
            b"strategy", 
            user_profile.key().as_ref(),
            &strategy_id.to_le_bytes()
        ],
        bump,
        constraint = strategy_config.owner == owner.key() @ ErrorCode::Unauthorized
    )]
    pub strategy_config: Account<'info, StrategyConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(strategy_id: u64)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump,
        constraint = user_profile.owner == owner.key() @ ErrorCode::Unauthorized
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [
            b"strategy", 
            user_profile.key().as_ref(),
            &strategy_id.to_le_bytes()
        ],
        bump,
        constraint = strategy_config.owner == owner.key() @ ErrorCode::Unauthorized
    )]
    pub strategy_config: Account<'info, StrategyConfig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(strategy_id: u64)]
pub struct RebalancePositions<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump,
        constraint = user_profile.owner == owner.key() @ ErrorCode::Unauthorized,
        constraint = !user_profile.is_paused @ ErrorCode::StrategyPaused
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        mut,
        seeds = [
            b"strategy", 
            user_profile.key().as_ref(),
            &strategy_id.to_le_bytes()
        ],
        bump,
        constraint = strategy_config.owner == owner.key() @ ErrorCode::Unauthorized
    )]
    pub strategy_config: Account<'info, StrategyConfig>,
    
    pub system_program: Program<'info, System>,
}