pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HVNEuy9WDgrNsoqfQcTHJVpMYoDTyoyJe9VivY3Z4Hx");

#[program]
pub mod sol_basic_launchpad {
    use super::*;

    pub fn create_native_pool(
        ctx: Context<CreateFairlaunchPool>,
        unlock_date: i64,
        pool_size: u64,
        minimum_token_amount: u64,
        maximum_token_amount: u64,
        rate: u64,
        token_mint_decimals: u8,
    ) -> Result<()> {
        instructions::create_fairlaunch_pool::handler(
            ctx,
            unlock_date,
            pool_size,
            minimum_token_amount,
            maximum_token_amount,
            rate,
            token_mint_decimals,
        )
    }

    pub fn start_launch_pool(ctx: Context<StartLaunchPool>) -> Result<()> {
        instructions::start_launch_pool::handler(ctx)
    }

    pub fn buy_token_with_native(ctx: Context<BuyTokenWithNative>, amount: u64) -> Result<()> {
        instructions::buy_token_with_native::handler(ctx, amount)
    }

    pub fn complete_launch_pool(ctx: Context<CompleteLaunchPool>) -> Result<()> {
        instructions::complete_launch_pool::handler(ctx)
    }

    pub fn claim_token(ctx: Context<ClaimToken>) -> Result<()> {
        instructions::claim_token::handler(ctx)
    }
}
