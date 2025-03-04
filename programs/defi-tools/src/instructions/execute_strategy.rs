use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

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