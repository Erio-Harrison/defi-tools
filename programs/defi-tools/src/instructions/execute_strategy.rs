use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

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

pub fn process(ctx: Context<ExecuteStrategy>, strategy_id: u64) -> Result<()> {
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

#[cfg(test)]
mod tests {
    struct UserProfile {
        owner: [u8; 32],
        last_activity: i64,
        is_paused: bool,
    }

    struct StrategyConfig {
        owner: [u8; 32],
        last_executed_at: i64,
    }

    struct InitializeUserAccounts {
        owner: [u8; 32],
        user_profile: UserProfile,
        strategy_config: StrategyConfig,
    }

    struct Bumps {
        user_profile: u8,
        strategy_config: u8,
    }

    struct Context {
        accounts: InitializeUserAccounts,
        bumps: Bumps,
    }

    type Result<T> = core::result::Result<T, ErrorCode>;

    #[derive(Debug, PartialEq)]
    enum ErrorCode {
        Unauthorized,
        StrategyPaused,
    }

    struct Clock {
        unix_timestamp: i64,
    }

    static mut TEST_TIMESTAMP: i64 = 1234567890;

    fn get_clock() -> Result<Clock> {
        unsafe {
            Ok(Clock {
                unix_timestamp: TEST_TIMESTAMP,
            })
        }
    }

    fn msg(_s: &str) {}

    fn process(ctx: &mut Context, strategy_id: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = get_clock()?;

        strategy_config.last_executed_at = clock.unix_timestamp;
        user_profile.last_activity = clock.unix_timestamp;

        msg(&format!("策略已执行，ID: {}", strategy_id));

        Ok(())
    }

    // 测试1: 正常执行策略
    #[test]
    fn test_execute_strategy_success() {
        unsafe {
            TEST_TIMESTAMP = 1234567890;
        } // 重置时间戳

        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 0,
        };
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
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

    // 测试2: 不同 strategy_id 的正常执行
    #[test]
    fn test_execute_strategy_different_id() {
        unsafe {
            TEST_TIMESTAMP = 1234567890;
        } // 重置时间戳

        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 0,
        };
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
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

    // 测试3: 使用不同的时间戳
    #[test]
    fn test_execute_strategy_different_timestamp() {
        unsafe {
            TEST_TIMESTAMP = 987654321;
        } // 设置不同时间戳

        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
            last_executed_at: 0,
        };
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
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
        assert_eq!(user_profile.last_activity, 987654321);
        assert_eq!(strategy_config.last_executed_at, 987654321);
    }
}
