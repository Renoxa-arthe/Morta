use anchor_lang::prelude::*;

#[program]
pub mod mint_contract {
    use super::*;

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let token_data = &mut ctx.accounts.token_data;
        token_data.supply += amount; // increase supply
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub token_data: Account<'info, TokenData>,
    pub user: Signer<'info>,
}

#[account]
pub struct TokenData {
    pub name: String,
    pub symbol: String,
    pub supply: u64,
}
