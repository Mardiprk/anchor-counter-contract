use anchor_lang::prelude::*;

declare_id!("F81Y8ZkQb9xdwRgRDhYyfuJibjRXpRaxfrpk998ZJbi2");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = *ctx.accounts.user.key;
        msg!("Counter Initialized with count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Counter incremented: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(counter.count > 0, CounterError::CannotDecrementBelowZero);
        counter.count -= 1;
        msg!("Counter decremented: {}", counter.count);
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter reset to: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = Counter::LEN
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(
        mut,
        has_one = authority @ CounterError::NotAuthorized
        )]
    pub counter: Account<'info, Counter>,

    pub authority: Signer<'info>,
}

// main account
#[account]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}

impl Counter {
    pub const LEN: usize = 8 + 8 + 32;
}

#[error_code]
pub enum CounterError {
    #[msg("Cannot decrement below zero!")]
    CannotDecrementBelowZero,
    #[msg("Not Authorized")]
    NotAuthorized,
}
