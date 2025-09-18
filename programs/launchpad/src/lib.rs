use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWq4a9YPFsrtd3HDS7y3o5AFy"); // temp program ID

#[program]
pub mod launchpad {
    use super::*;

    pub fn initialize_token(
        ctx: Context<InitializeToken>,
        name: String,
        symbol: String,
        supply: u64,
    ) -> Result<()> {
        let token_data = &mut ctx.accounts.token_data;
        token_data.name = name;
        token_data.symbol = symbol;
        token_data.supply = supply;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32 + 8)]
    pub token_data: Account<'info, TokenData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
      }
