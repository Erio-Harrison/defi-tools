use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct CreateStrategy<'info> {
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
        init,
        payer = owner,
        space = 8 + std::mem::size_of::<StrategyConfig>() + 
                4 + (std::mem::size_of::<Allocation>() * 10), // 最多10个分配项
        seeds = [
            b"strategy", 
            user_profile.key().as_ref(),
            &user_profile.strategy_counter.to_le_bytes()
        ],
        bump
    )]
    pub strategy_config: Account<'info, StrategyConfig>,
    
    pub system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<CreateStrategy>, 
    allocations: Vec<Allocation>,
    rebalance_condition: RebalanceCondition,
    max_slippage_bps: u16
) -> Result<()> {
    // 验证资产分配
    let mut total_weight = 0;
    for allocation in allocations.iter() {
        total_weight += allocation.target_weight_bps;
    }
    
    // 确保总权重为10000基点 (100%)
    if total_weight != 10000 {
        return Err(ErrorCode::InvalidAllocation.into());
    }
    
    // 验证滑点设置
    if max_slippage_bps > 1000 { // 最大10%
        return Err(ErrorCode::InvalidSlippage.into());
    }
    
    let user_profile = &mut ctx.accounts.user_profile;
    let strategy_config = &mut ctx.accounts.strategy_config;
    let clock = Clock::get()?;
    
    // 设置策略配置
    strategy_config.owner = ctx.accounts.owner.key();
    strategy_config.strategy_id = user_profile.strategy_counter;
    strategy_config.allocations = allocations;
    strategy_config.rebalance_condition = rebalance_condition;
    strategy_config.created_at = clock.unix_timestamp;
    strategy_config.last_executed_at = 0;
    strategy_config.max_slippage_bps = max_slippage_bps;
    
    // 增加用户的策略计数器
    user_profile.strategy_counter += 1;
    user_profile.last_activity = clock.unix_timestamp;
    
    msg!("策略已创建，ID: {}", strategy_config.strategy_id);
    
    Ok(())
}