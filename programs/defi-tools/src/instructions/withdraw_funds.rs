use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

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
    
    // 更新用户活动时间
    user_profile.last_activity = clock.unix_timestamp;
    
    // 更新总资产价值
    user_profile.total_value_lamports = user_profile.total_value_lamports.checked_sub(amount)
        .ok_or(ErrorCode::MathError)?;
    
    // 在实际应用中，这里会调用资金提取和转移的逻辑
    
    msg!("从策略提取资金，ID: {}，金额: {}", strategy_id, amount);
    
    Ok(())
}