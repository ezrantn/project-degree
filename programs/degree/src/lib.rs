use anchor_lang::prelude::*;

declare_id!("NtBiZ2Bo9cyN8pGrwvtCgGPvdfai7Hv3nBU8zr5BSfm");

#[program]
pub mod degree {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let diploma_registry = &mut ctx.accounts.diploma_registry;
        diploma_registry.authority = ctx.accounts.authority.key();
        diploma_registry.count = 0;

        msg!(
            "Diploma registry initialized by: {:?}",
            diploma_registry.authority
        );
        Ok(())
    }

    pub fn add_diploma(ctx: Context<AddDiploma>, diploma_id: String) -> Result<()> {
        require!(diploma_id.len() > 0, DiplomaError::EmptyDiplomaId);
        require!(diploma_id.len() <= 100, DiplomaError::DiplomaIdTooLong);

        let diploma_registry = &mut ctx.accounts.diploma_registry;
        let diploma = &mut ctx.accounts.diploma;

        // Initialize the diploma account
        diploma.authority = ctx.accounts.authority.key();
        diploma.diploma_id = diploma_id;
        diploma.verified = true;
        diploma.created_at = Clock::get()?.unix_timestamp;

        // Increment the count of diplomas in the registry
        diploma_registry.count += 1;

        msg!("Diploma added: {}", diploma.diploma_id);
        Ok(())
    }

    pub fn revoke_diploma(ctx: Context<RevokeDiploma>) -> Result<()> {
        let diploma = &mut ctx.accounts.diploma;
        let diploma_registry = &mut ctx.accounts.diploma_registry;

        require!(diploma.verified, DiplomaError::DiplomaAlreadyRevoked);

        diploma.verified = false;
        diploma_registry.count -= 1;

        msg!("Diploma revoked: {}", diploma.diploma_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8, // discriminator + pubkey + u64 counter
        seeds = [b"diploma-registry"],
        bump
    )]
    pub diploma_registry: Account<'info, DiplomaRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(diploma_id: String)]
pub struct AddDiploma<'info> {
    #[account(
        mut,
        seeds = [b"diploma-registry"],
        bump
    )]
    pub diploma_registry: Account<'info, DiplomaRegistry>,

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 100 + 1 + 8, // discriminator + pubkey + string prefix + max string length + bool + timestamp
        seeds = [b"diploma", diploma_id.as_bytes()],
        bump
    )]
    pub diploma: Account<'info, Diploma>,

    #[account(
        mut,
        constraint = authority.key() == diploma_registry.authority
    )]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeDiploma<'info> {
    #[account(
        mut,
        seeds = [b"diploma-registry"],
        bump
    )]
    pub diploma_registry: Account<'info, DiplomaRegistry>,

    #[account(
        mut,
        seeds = [b"diploma", diploma.diploma_id.as_bytes()],
        bump,
        constraint = authority.key() == diploma_registry.authority
    )]
    pub diploma: Account<'info, Diploma>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct DiplomaRegistry {
    pub authority: Pubkey,
    pub count: u64,
}

#[account]
pub struct Diploma {
    pub authority: Pubkey,
    pub diploma_id: String,
    pub verified: bool,
    pub created_at: i64,
}

#[error_code]
pub enum DiplomaError {
    #[msg("Diploma ID cannot be empty")]
    EmptyDiplomaId,
    #[msg("Diploma ID is too long")]
    DiplomaIdTooLong,
    #[msg("Diploma is already revoked")]
    DiplomaAlreadyRevoked,
}
