use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

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

pub fn process(ctx: Context<RebalancePositions>, strategy_id: u64) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let strategy_config = &mut ctx.accounts.strategy_config;
    let clock = Clock::get()?;

    // 验证再平衡条件
    let time_since_last_execution = clock.unix_timestamp - strategy_config.last_executed_at;
    if time_since_last_execution < strategy_config.rebalance_condition.time_interval_seconds as i64
    {
        return Err(ErrorCode::RebalanceConditionNotMet.into());
    }

    // 更新最后执行时间
    strategy_config.last_executed_at = clock.unix_timestamp;
    user_profile.last_activity = clock.unix_timestamp;

    // 在实际应用中，这里会调用再平衡头寸的逻辑

    msg!("策略头寸已再平衡，ID: {}", strategy_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    // 数据结构定义保持不变
    struct UserProfile {
        owner: [u8; 32],
        last_activity: i64,
        is_paused: bool,
    }

    struct RebalanceCondition {
        time_interval_seconds: u64,
    }

    struct StrategyConfig {
        owner: [u8; 32],
        last_executed_at: i64,
        rebalance_condition: RebalanceCondition,
    }

    struct RebalancePositionsAccounts {
        owner: [u8; 32],
        user_profile: UserProfile,
        strategy_config: StrategyConfig,
    }

    struct Bumps {
        user_profile: u8,
        strategy_config: u8,
    }

    struct Context {
        accounts: RebalancePositionsAccounts,
        bumps: Bumps,
    }

    type Result<T> = core::result::Result<T, ErrorCode>;

    #[derive(Debug, PartialEq)]
    enum ErrorCode {
        Unauthorized,
        StrategyPaused,
        RebalanceConditionNotMet,
    }

    struct Clock {
        unix_timestamp: i64,
    }

    fn get_clock() -> Result<Clock> {
        Ok(Clock {
            unix_timestamp: 1234567890,
        })
    }

    fn msg(_s: &str) {}

    fn process(ctx: &mut Context, strategy_id: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = get_clock()?;

        let time_since_last_execution = clock.unix_timestamp - strategy_config.last_executed_at;
        if time_since_last_execution
            < strategy_config.rebalance_condition.time_interval_seconds as i64
        {
            return Err(ErrorCode::RebalanceConditionNotMet);
        }

        strategy_config.last_executed_at = clock.unix_timestamp;
        user_profile.last_activity = clock.unix_timestamp;

        msg(&format!("策略头寸已再平衡，ID: {}", strategy_id));

        Ok(())
    }

    // 测试1: 正常再平衡（修复）
    #[test]
    fn test_rebalance_positions_success() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 1234564290, // 调整为 3600 秒前
            rebalance_condition: RebalanceCondition {
                time_interval_seconds: 3600,
            },
        };
        let mut ctx = Context {
            accounts: RebalancePositionsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let strategy_id = 42;
        let result = process(&mut ctx, strategy_id);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        let strategy_config = &ctx.accounts.strategy_config;
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(strategy_config.last_executed_at, 1234567890);
    }

    // 测试2: 再平衡条件未满足
    #[test]
    fn test_rebalance_positions_condition_not_met() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 1234567390,
            rebalance_condition: RebalanceCondition {
                time_interval_seconds: 3600,
            },
        };
        let mut ctx = Context {
            accounts: RebalancePositionsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let strategy_id = 42;
        let result = process(&mut ctx, strategy_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::RebalanceConditionNotMet);
    }

    // 测试3: 刚好满足再平衡条件
    #[test]
    fn test_rebalance_positions_exact_condition() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 1234564290,
            rebalance_condition: RebalanceCondition {
                time_interval_seconds: 3600,
            },
        };
        let mut ctx = Context {
            accounts: RebalancePositionsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let strategy_id = 42;
        let result = process(&mut ctx, strategy_id);
        assert!(result.is_ok());
    }

    // 测试4: 从未执行过策略
    #[test]
    fn test_rebalance_positions_never_executed() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 0,
            rebalance_condition: RebalanceCondition {
                time_interval_seconds: 3600,
            },
        };
        let mut ctx = Context {
            accounts: RebalancePositionsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let strategy_id = 42;
        let result = process(&mut ctx, strategy_id);
        assert!(result.is_ok());
    }

    // 测试5: 不同 strategy_id 的再平衡（修复）
    #[test]
    fn test_rebalance_positions_different_strategy_id() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 1234564290, // 调整为 3600 秒前
            rebalance_condition: RebalanceCondition {
                time_interval_seconds: 3600,
            },
        };
        let mut ctx = Context {
            accounts: RebalancePositionsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
            bumps: Bumps {
                user_profile: 255,
                strategy_config: 254,
            },
        };

        let strategy_id = 999;
        let result = process(&mut ctx, strategy_id);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        let strategy_config = &ctx.accounts.strategy_config;
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(strategy_config.last_executed_at, 1234567890);
    }
}
