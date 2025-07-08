use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;
declare_id!("8UzZwP7FjcjVyaEf7kwjjzr4JizBKtmjUdmzACk5vu7s");

#[program]
pub mod blueshift_anchor_vault {

    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        // deposit logic

        // check if vault is empty
        require_eq!(
            ctx.accounts.vault.lamports(),
            0,
            VaultError::VaultAlreadyExist
        );

        // ensure the amount exceeds the rent-exempt limit
        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );

        transfer(
            CpiContext::new(
                ctx.accounts.system_prgram.to_account_info(),
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        // withdraw

        

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_prgram: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    // error enum
    #[msg("vault already exists")]
    VaultAlreadyExist,
    #[msg("Invalid Amount")]
    InvalidAmount,
}
