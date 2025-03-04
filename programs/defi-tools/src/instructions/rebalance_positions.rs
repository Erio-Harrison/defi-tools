use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

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