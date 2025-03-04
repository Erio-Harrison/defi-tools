use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

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

pub fn process(ctx: Context<WithdrawFunds>, strategy_id: u64, amount: u64) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let clock = Clock::get()?;

    // 验证提款金额
    if amount == 0 || amount > user_profile.total_value_lamports {
        return Err(ErrorCode::InsufficientFunds.into());
    }

    // 手动验证授权
    if ctx.accounts.owner.key() != user_profile.owner {
        return Err(ErrorCode::Unauthorized.into());
    }

    // 更新用户活动时间
    user_profile.last_activity = clock.unix_timestamp;

    // 更新总资产价值
    user_profile.total_value_lamports = user_profile
        .total_value_lamports
        .checked_sub(amount)
        .ok_or(ErrorCode::MathError)?;

    // 在实际应用中，这里会调用资金提取和转移的逻辑

    msg!("从策略提取资金，ID: {}，金额: {}", strategy_id, amount);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 模拟必要的数据结构
    struct UserProfile {
        owner: [u8; 32],
        total_value_lamports: u64,
        last_activity: i64,
    }

    struct StrategyConfig {
        owner: [u8; 32],
    }

    struct WithdrawFundsAccounts {
        owner: [u8; 32],
        user_profile: UserProfile,
        strategy_config: StrategyConfig,
    }

    struct Context {
        accounts: WithdrawFundsAccounts,
    }

    // 模拟 Result 类型
    type Result<T> = core::result::Result<T, ErrorCode>;

    #[derive(Debug, PartialEq)]
    enum ErrorCode {
        Unauthorized,
        InsufficientFunds,
        MathError,
    }

    // 模拟 Clock
    struct Clock {
        unix_timestamp: i64,
    }

    fn get_clock() -> Result<Clock> {
        Ok(Clock {
            unix_timestamp: 1234567890,
        })
    }

    fn process(ctx: &mut Context, strategy_id: u64, amount: u64) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let clock = get_clock()?;

        if amount == 0 || amount > user_profile.total_value_lamports {
            return Err(ErrorCode::InsufficientFunds);
        }

        if ctx.accounts.owner != user_profile.owner {
            return Err(ErrorCode::Unauthorized.into());
        }

        user_profile.last_activity = clock.unix_timestamp;
        user_profile.total_value_lamports = user_profile
            .total_value_lamports
            .checked_sub(amount)
            .ok_or(ErrorCode::MathError)?;

        Ok(())
    }

    #[test]
    fn test_withdraw_funds_success() {
        let owner_key = [1; 32];
        let mut user_profile = UserProfile {
            owner: owner_key,
            total_value_lamports: 1000,
            last_activity: 0,
        };
        let strategy_config = StrategyConfig { owner: owner_key };
        let mut ctx = Context {
            accounts: WithdrawFundsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
        };

        let result = process(&mut ctx, 1, 500);
        assert!(result.is_ok());
        assert_eq!(ctx.accounts.user_profile.total_value_lamports, 500);
        assert_eq!(ctx.accounts.user_profile.last_activity, 1234567890);
    }

    #[test]
    fn test_withdraw_funds_insufficient_balance() {
        let owner_key = [1; 32];
        let user_profile = UserProfile {
            owner: owner_key,
            total_value_lamports: 300,
            last_activity: 0,
        };
        let strategy_config = StrategyConfig { owner: owner_key };
        let mut ctx = Context {
            accounts: WithdrawFundsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
        };

        let result = process(&mut ctx, 1, 500);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::InsufficientFunds);
    }

    #[test]
    fn test_withdraw_funds_zero_amount() {
        let owner_key = [1; 32];
        let user_profile = UserProfile {
            owner: owner_key,
            total_value_lamports: 1000,
            last_activity: 0,
        };
        let strategy_config = StrategyConfig { owner: owner_key };
        let mut ctx = Context {
            accounts: WithdrawFundsAccounts {
                owner: owner_key,
                user_profile,
                strategy_config,
            },
        };

        let result = process(&mut ctx, 1, 0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::InsufficientFunds);
    }

    #[test]
    fn test_withdraw_funds_unauthorized() {
        let owner_key = [1; 32]; // 账户实际拥有者
        let unauthorized_key = [2; 32]; // 未授权用户（测试用）

        let user_profile = UserProfile {
            owner: owner_key, // 真实 owner
            total_value_lamports: 1000,
            last_activity: 0,
        };

        let strategy_config = StrategyConfig {
            owner: owner_key, // strategy 仍然属于 owner_key
        };

        let mut ctx = Context {
            accounts: WithdrawFundsAccounts {
                owner: unauthorized_key, // 传入未授权用户
                user_profile,
                strategy_config,
            },
        };

        let result = process(&mut ctx, 1, 500);

        // 这里修正：确保 Unauthorized 错误被正确返回
        assert!(result.is_err(), "Expected Unauthorized error, but got Ok");
        assert_eq!(
            result.unwrap_err(),
            ErrorCode::Unauthorized,
            "Expected Unauthorized error"
        );
    }
}
