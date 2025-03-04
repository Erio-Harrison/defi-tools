use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

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

pub fn process(
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

#[cfg(test)]
mod tests {
    // 模拟必要的数据结构
    struct UserProfile {
        owner: [u8; 32], // 模拟 Pubkey
        strategy_counter: u64,
        last_activity: i64,
    }

    #[derive(Clone)]
    struct Allocation {
        target_weight_bps: u16, // 目标权重（基点）
    }

    #[derive(Clone)]
    enum RebalanceCondition {
        TimeBased(u64), // 模拟一个简单的基于时间的条件
    }

    struct StrategyConfig {
        owner: [u8; 32], // 模拟 Pubkey
        strategy_id: u64,
        allocations: Vec<Allocation>,
        rebalance_condition: RebalanceCondition,
        created_at: i64,
        last_executed_at: i64,
        max_slippage_bps: u16,
    }

    struct CreateStrategyAccounts {
        owner: [u8; 32], // 模拟 Signer 的 key
        user_profile: UserProfile,
        strategy_config: StrategyConfig,
    }

    struct Bumps {
        user_profile: u8,
        strategy_config: u8,
    }

    struct Context {
        accounts: CreateStrategyAccounts,
        bumps: Bumps,
    }

    // 模拟 Result 类型
    type Result<T> = core::result::Result<T, ErrorCode>;

    // 模拟 ErrorCode
    #[derive(Debug, PartialEq)]
    enum ErrorCode {
        Unauthorized,
        InvalidAllocation,
        InvalidSlippage,
    }

    // 模拟 Clock
    struct Clock {
        unix_timestamp: i64,
    }

    // 模拟 Clock::get 函数
    fn get_clock() -> Result<Clock> {
        Ok(Clock { unix_timestamp: 1234567890 }) // 默认时间戳
    }

    // 模拟 msg! 宏
    fn msg(_s: &str) {
        // 在测试中无需实际输出
    }

    // 测试目标函数
    fn process(
        ctx: &mut Context,
        allocations: Vec<Allocation>,
        rebalance_condition: RebalanceCondition,
        max_slippage_bps: u16,
    ) -> Result<()> {
        let mut total_weight = 0;
        for allocation in allocations.iter() {
            total_weight += allocation.target_weight_bps;
        }
        
        if total_weight != 10000 {
            return Err(ErrorCode::InvalidAllocation);
        }
        
        if max_slippage_bps > 1000 {
            return Err(ErrorCode::InvalidSlippage);
        }
        
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = get_clock()?;
        
        strategy_config.owner = ctx.accounts.owner;
        strategy_config.strategy_id = user_profile.strategy_counter;
        strategy_config.allocations = allocations;
        strategy_config.rebalance_condition = rebalance_condition;
        strategy_config.created_at = clock.unix_timestamp;
        strategy_config.last_executed_at = 0;
        strategy_config.max_slippage_bps = max_slippage_bps;
        
        user_profile.strategy_counter += 1;
        user_profile.last_activity = clock.unix_timestamp;
        
        msg(&format!("策略已创建，ID: {}", strategy_config.strategy_id));
        
        Ok(())
    }

    // 测试1: 正常创建策略
    #[test]
    fn test_create_strategy_success() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            strategy_counter: 0,
            last_activity: 0,
        };
        let mut strategy_config = StrategyConfig {
            owner: [0; 32],
            strategy_id: 0,
            allocations: vec![],
            rebalance_condition: RebalanceCondition::TimeBased(0),
            created_at: 0,
            last_executed_at: 0,
            max_slippage_bps: 0,
        };
        let mut ctx = Context {
            accounts: CreateStrategyAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let allocations = vec![
            Allocation { target_weight_bps: 5000 },
            Allocation { target_weight_bps: 5000 },
        ];
        let rebalance_condition = RebalanceCondition::TimeBased(86400); // 每天
        let max_slippage_bps = 500; // 5%

        let result = process(&mut ctx, allocations.clone(), rebalance_condition.clone(), max_slippage_bps);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        let strategy_config = &ctx.accounts.strategy_config;
        assert_eq!(user_profile.strategy_counter, 1);
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(strategy_config.owner, owner_key);
        assert_eq!(strategy_config.strategy_id, 0);
        assert_eq!(strategy_config.allocations.len(), 2);
        assert_eq!(strategy_config.allocations[0].target_weight_bps, 5000);
        assert_eq!(strategy_config.allocations[1].target_weight_bps, 5000);
        if let RebalanceCondition::TimeBased(time) = strategy_config.rebalance_condition {
            assert_eq!(time, 86400);
        }
        assert_eq!(strategy_config.created_at, 1234567890);
        assert_eq!(strategy_config.last_executed_at, 0);
        assert_eq!(strategy_config.max_slippage_bps, 500);
    }

    // 测试2: 分配总和不等于 10000
    #[test]
    fn test_create_strategy_invalid_allocation() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            strategy_counter: 0,
            last_activity: 0,
        };
        let mut strategy_config = StrategyConfig {
            owner: [0; 32],
            strategy_id: 0,
            allocations: vec![],
            rebalance_condition: RebalanceCondition::TimeBased(0),
            created_at: 0,
            last_executed_at: 0,
            max_slippage_bps: 0,
        };
        let mut ctx = Context {
            accounts: CreateStrategyAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let allocations = vec![
            Allocation { target_weight_bps: 4000 },
            Allocation { target_weight_bps: 5000 }, // 总和 9000 < 10000
        ];
        let rebalance_condition = RebalanceCondition::TimeBased(86400);
        let max_slippage_bps = 500;

        let result = process(&mut ctx, allocations, rebalance_condition, max_slippage_bps);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::InvalidAllocation);

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.strategy_counter, 0); // 未增加
        assert_eq!(user_profile.last_activity, 0); // 未更新
    }

    // 测试3: 滑点超过 1000 bps
    #[test]
    fn test_create_strategy_invalid_slippage() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            strategy_counter: 0,
            last_activity: 0,
        };
        let mut strategy_config = StrategyConfig {
            owner: [0; 32],
            strategy_id: 0,
            allocations: vec![],
            rebalance_condition: RebalanceCondition::TimeBased(0),
            created_at: 0,
            last_executed_at: 0,
            max_slippage_bps: 0,
        };
        let mut ctx = Context {
            accounts: CreateStrategyAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let allocations = vec![
            Allocation { target_weight_bps: 5000 },
            Allocation { target_weight_bps: 5000 },
        ];
        let rebalance_condition = RebalanceCondition::TimeBased(86400);
        let max_slippage_bps = 1500; // 15% > 10%

        let result = process(&mut ctx, allocations, rebalance_condition, max_slippage_bps);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::InvalidSlippage);

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.strategy_counter, 0); // 未增加
        assert_eq!(user_profile.last_activity, 0); // 未更新
    }

    // 测试4: 最大分配项 (10 个)
    #[test]
    fn test_create_strategy_max_allocations() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            strategy_counter: 0,
            last_activity: 0,
        };
        let mut strategy_config = StrategyConfig {
            owner: [0; 32],
            strategy_id: 0,
            allocations: vec![],
            rebalance_condition: RebalanceCondition::TimeBased(0),
            created_at: 0,
            last_executed_at: 0,
            max_slippage_bps: 0,
        };
        let mut ctx = Context {
            accounts: CreateStrategyAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let allocations = vec![
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
            Allocation { target_weight_bps: 1000 },
        ];
        let rebalance_condition = RebalanceCondition::TimeBased(86400);
        let max_slippage_bps = 500;

        let result = process(&mut ctx, allocations.clone(), rebalance_condition.clone(), max_slippage_bps);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        let strategy_config = &ctx.accounts.strategy_config;
        assert_eq!(user_profile.strategy_counter, 1);
        assert_eq!(strategy_config.allocations.len(), 10);
        assert_eq!(strategy_config.max_slippage_bps, 500);
    }

    // 测试5: 最小滑点 (0 bps)
    #[test]
    fn test_create_strategy_min_slippage() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            strategy_counter: 0,
            last_activity: 0,
        };
        let mut strategy_config = StrategyConfig {
            owner: [0; 32],
            strategy_id: 0,
            allocations: vec![],
            rebalance_condition: RebalanceCondition::TimeBased(0),
            created_at: 0,
            last_executed_at: 0,
            max_slippage_bps: 0,
        };
        let mut ctx = Context {
            accounts: CreateStrategyAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let allocations = vec![
            Allocation { target_weight_bps: 5000 },
            Allocation { target_weight_bps: 5000 },
        ];
        let rebalance_condition = RebalanceCondition::TimeBased(86400);
        let max_slippage_bps = 0; // 最小滑点

        let result = process(&mut ctx, allocations.clone(), rebalance_condition.clone(), max_slippage_bps);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        let strategy_config = &ctx.accounts.strategy_config;
        assert_eq!(user_profile.strategy_counter, 1);
        assert_eq!(strategy_config.max_slippage_bps, 0);
        assert_eq!(strategy_config.created_at, 1234567890);
    }
}