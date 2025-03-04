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