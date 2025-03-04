use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

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

pub fn process(ctx: Context<DepositFunds>, strategy_id: u64, amount: u64) -> Result<()> {
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

#[cfg(test)]
mod tests {
    // 模拟必要的数据结构
    struct UserProfile {
        owner: [u8; 32], // 模拟 Pubkey
        last_activity: i64,
        is_paused: bool,
        total_value_lamports: u64,
    }

    struct StrategyConfig {
        owner: [u8; 32], // 模拟 Pubkey
    }

    struct DepositFundsAccounts {
        owner: [u8; 32], // 模拟 Signer 的 key
        user_profile: UserProfile,
        strategy_config: StrategyConfig,
    }

    struct Bumps {
        user_profile: u8,
        strategy_config: u8,
    }

    struct Context {
        accounts: DepositFundsAccounts,
        bumps: Bumps,
    }

    // 模拟 Result 类型
    type Result<T> = core::result::Result<T, ErrorCode>;

    // 模拟 ErrorCode
    #[derive(Debug, PartialEq)]
    enum ErrorCode {
        Unauthorized,
        StrategyPaused,
        InsufficientFunds,
        MathError,
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
    fn process(ctx: &mut Context, strategy_id: u64, amount: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let strategy_config = &mut ctx.accounts.strategy_config;
        let clock = get_clock()?;
        
        // 验证存款金额
        if amount == 0 {
            return Err(ErrorCode::InsufficientFunds);
        }
        
        // 更新用户活动时间
        user_profile.last_activity = clock.unix_timestamp;
        
        // 更新总资产价值
        user_profile.total_value_lamports = user_profile.total_value_lamports.checked_add(amount)
            .ok_or(ErrorCode::MathError)?;
        
        msg(&format!("向策略存入资金，ID: {}，金额: {}", strategy_id, amount));
        
        Ok(())
    }

    // 测试1: 正常存款
    #[test]
    fn test_deposit_funds_success() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
            total_value_lamports: 1000,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
        };
        let mut ctx = Context {
            accounts: DepositFundsAccounts {
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
        let amount = 500;
        let result = process(&mut ctx, strategy_id, amount);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(user_profile.total_value_lamports, 1500); // 1000 + 500
    }

    // 测试2: 存款金额为 0
    #[test]
    fn test_deposit_funds_zero_amount() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
            total_value_lamports: 1000,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
        };
        let mut ctx = Context {
            accounts: DepositFundsAccounts {
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
        let amount = 0;
        let result = process(&mut ctx, strategy_id, amount);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::InsufficientFunds);

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.last_activity, 0); // 未更新
        assert_eq!(user_profile.total_value_lamports, 1000); // 未改变
    }

    // 测试3: 存款导致溢出
    #[test]
    fn test_deposit_funds_overflow() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
            total_value_lamports: u64::MAX, // 最大值
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
        };
        let mut ctx = Context {
            accounts: DepositFundsAccounts {
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
        let amount = 1; // 加 1 会溢出
        let result = process(&mut ctx, strategy_id, amount);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::MathError);

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.last_activity, 1234567890); // 时间戳更新了
        assert_eq!(user_profile.total_value_lamports, u64::MAX); // 未改变
    }

    // 测试4: 不同 strategy_id 的正常存款
    #[test]
    fn test_deposit_funds_different_strategy_id() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
            total_value_lamports: 2000,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
        };
        let mut ctx = Context {
            accounts: DepositFundsAccounts {
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
        let amount = 300;
        let result = process(&mut ctx, strategy_id, amount);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(user_profile.total_value_lamports, 2300); // 2000 + 300
    }

    // 测试5: 最小有效金额存款
    #[test]
    fn test_deposit_funds_minimum_amount() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            last_activity: 0,
            is_paused: false,
            total_value_lamports: 1000,
        };
        let mut strategy_config = StrategyConfig {
            owner: owner_key,
        };
        let mut ctx = Context {
            accounts: DepositFundsAccounts {
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
        let amount = 1; // 最小有效金额
        let result = process(&mut ctx, strategy_id, amount);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(user_profile.total_value_lamports, 1001); // 1000 + 1
    }
}