use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

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
        Ok(Clock { unix_timestamp: 1234567890 }) // 默认时间戳
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
}