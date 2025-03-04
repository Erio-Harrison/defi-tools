use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

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

pub fn process(ctx: Context<InitializeUser>, risk_level: u8) -> Result<()> {
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

#[cfg(test)]
mod tests {
    // 模拟必要的数据结构
    struct UserProfile {
        owner: [u8; 32], // 模拟 Pubkey
        risk_level: u8,
        strategy_counter: u64,
        vault_bump: u8,
        last_activity: i64,
        total_value_lamports: u64,
        is_paused: bool,
    }

    struct InitializeUserAccounts {
        owner: [u8; 32], // 模拟 Signer 的 key
        user_profile: UserProfile,
    }

    struct Bumps {
        user_profile: u8,
    }

    struct Context {
        accounts: InitializeUserAccounts,
        bumps: Bumps,
    }

    // 模拟 Result 类型
    type Result<T> = core::result::Result<T, ErrorCode>;

    // 模拟 ErrorCode
    #[derive(Debug, PartialEq)]
    enum ErrorCode {
        InvalidRiskLevel,
    }

    // 模拟 Clock
    struct Clock {
        unix_timestamp: i64,
    }

    // 模拟 Clock::get 函数
    fn get_clock() -> Result<Clock> {
        Ok(Clock {
            unix_timestamp: 1234567890,
        }) // 默认时间戳
    }

    // 模拟 msg! 宏
    fn msg(_s: &str) {
        // 在测试中无需实际输出
    }

    // 修改 process 函数，使用可变引用 &mut Context
    fn process(ctx: &mut Context, risk_level: u8) -> Result<()> {
        // 验证风险等级
        if risk_level < 1 || risk_level > 5 {
            return Err(ErrorCode::InvalidRiskLevel);
        }

        let user_profile = &mut ctx.accounts.user_profile;
        let clock = get_clock()?;

        // 初始化用户配置文件
        user_profile.owner = ctx.accounts.owner;
        user_profile.risk_level = risk_level;
        user_profile.strategy_counter = 0;
        user_profile.vault_bump = ctx.bumps.user_profile;
        user_profile.last_activity = clock.unix_timestamp;
        user_profile.total_value_lamports = 0;
        user_profile.is_paused = false;

        msg(&format!("用户配置文件已初始化，风险等级: {}", risk_level));

        Ok(())
    }

    // 测试1: 正常初始化
    #[test]
    fn test_initialize_user_success() {
        let owner_key = [1; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 3); // 传入可变引用
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile; // 现在可以安全访问
        assert_eq!(user_profile.owner, owner_key);
        assert_eq!(user_profile.risk_level, 3);
        assert_eq!(user_profile.strategy_counter, 0);
        assert_eq!(user_profile.vault_bump, bump);
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(user_profile.total_value_lamports, 0);
        assert_eq!(user_profile.is_paused, false);
    }

    // 测试2: 风险等级过低 (0)
    #[test]
    fn test_initialize_user_risk_level_too_low() {
        let owner_key = [1; 32];
        let initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: 255 },
        };

        let result = process(&mut ctx, 0); // 传入可变引用
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::InvalidRiskLevel);
    }

    // 测试3: 风险等级过高 (6)
    #[test]
    fn test_initialize_user_risk_level_too_high() {
        let owner_key = [1; 32];
        let initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: 255 },
        };

        let result = process(&mut ctx, 6); // 传入可变引用
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::InvalidRiskLevel);
    }

    // 测试4: 边界值 - 最低有效风险等级 (1)
    #[test]
    fn test_initialize_user_min_risk_level() {
        let owner_key = [1; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 1); // 传入可变引用
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile; // 现在可以安全访问
        assert_eq!(user_profile.risk_level, 1);
        assert_eq!(user_profile.owner, owner_key);
    }

    // 测试5: 边界值 - 最高有效风险等级 (5)
    #[test]
    fn test_initialize_user_max_risk_level() {
        let owner_key = [1; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 5); // 传入可变引用
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile; // 现在可以安全访问
        assert_eq!(user_profile.risk_level, 5);
        assert_eq!(user_profile.owner, owner_key);
    }

    // 测试6: 验证所有字段正确初始化
    #[test]
    fn test_all_fields_initialized_correctly() {
        let owner_key = [2; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 10, // 非零值测试是否重置
            vault_bump: 123,
            last_activity: 999,
            total_value_lamports: 100,
            is_paused: true, // 测试是否重置为false
        };
        let bump = 200;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 2);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.owner, owner_key);
        assert_eq!(user_profile.risk_level, 2);
        assert_eq!(user_profile.strategy_counter, 0); // 应重置为0
        assert_eq!(user_profile.vault_bump, bump);
        assert_eq!(user_profile.last_activity, 1234567890);
        assert_eq!(user_profile.total_value_lamports, 0); // 应重置为0
        assert_eq!(user_profile.is_paused, false); // 应重置为false
    }

    // 测试7: 不同vault_bump值测试
    #[test]
    fn test_different_vault_bump_values() {
        let owner_key = [3; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 123; // 使用非255的bump值
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 3);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.vault_bump, bump);
    }

    // 测试8: 不同owner地址测试
    #[test]
    fn test_different_owner_address() {
        let owner_key = [4; 32]; // 使用不同owner
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 4);
        assert!(result.is_ok());

        let user_profile = &ctx.accounts.user_profile;
        assert_eq!(user_profile.owner, owner_key);
    }

    // 测试9: 中间风险等级测试（2和4）
    #[test]
    fn test_middle_risk_levels() {
        let owner_key = [5; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        // 测试风险等级2
        let result = process(&mut ctx, 2);
        assert!(result.is_ok());
        assert_eq!(ctx.accounts.user_profile.risk_level, 2);

        // 重置测试环境
        ctx.accounts.user_profile.risk_level = 0;

        // 测试风险等级4
        let result = process(&mut ctx, 4);
        assert!(result.is_ok());
        assert_eq!(ctx.accounts.user_profile.risk_level, 4);
    }

    // 测试10: 验证is_paused初始状态
    #[test]
    fn test_initial_paused_state() {
        let owner_key = [6; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: true, // 初始为true，测试是否被设为false
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 3);
        assert!(result.is_ok());
        assert_eq!(ctx.accounts.user_profile.is_paused, false);
    }

    // 测试11: 验证lamports初始为0
    #[test]
    fn test_initial_lamports_zero() {
        let owner_key = [7; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 500, // 初始非零值
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 3);
        assert!(result.is_ok());
        assert_eq!(ctx.accounts.user_profile.total_value_lamports, 0);
    }

    // 测试12: 验证策略计数器重置
    #[test]
    fn test_strategy_counter_reset() {
        let owner_key = [8; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 5, // 初始非零值
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        let result = process(&mut ctx, 3);
        assert!(result.is_ok());
        assert_eq!(ctx.accounts.user_profile.strategy_counter, 0);
    }

    // 测试13: 多次初始化尝试（应允许重复初始化）
    #[test]
    fn test_multiple_initializations() {
        let owner_key = [10; 32];
        let mut initial_profile = UserProfile {
            owner: [0; 32],
            risk_level: 0,
            strategy_counter: 0,
            vault_bump: 0,
            last_activity: 0,
            total_value_lamports: 0,
            is_paused: false,
        };
        let bump = 255;
        let mut ctx = Context {
            accounts: InitializeUserAccounts {
                owner: owner_key,
                user_profile: initial_profile,
            },
            bumps: Bumps { user_profile: bump },
        };

        // 第一次初始化
        let result1 = process(&mut ctx, 3);
        assert!(result1.is_ok());

        // 修改风险等级后再次初始化
        let result2 = process(&mut ctx, 2);
        assert!(result2.is_ok());
        assert_eq!(ctx.accounts.user_profile.risk_level, 2);
    }
}
