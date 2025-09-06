#![allow(unexpected_cfgs)]
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

    // Modified to accept ipfs_hash
    pub fn add_diploma(
        ctx: Context<AddDiploma>,
        diploma_id: String,
        ipfs_hash: String,
    ) -> Result<()> {
        require!(diploma_id.len() > 0, DiplomaError::EmptyDiplomaId);
        require!(diploma_id.len() <= 100, DiplomaError::DiplomaIdTooLong);
        require!(ipfs_hash.len() > 0, DiplomaError::EmptyIpfsHash);
        require!(ipfs_hash.len() <= 60, DiplomaError::IpfsHashTooLong); // Assuming max 60 chars for base58 CID

        let diploma_registry = &mut ctx.accounts.diploma_registry;
        let diploma = &mut ctx.accounts.diploma;

        // Initialize the diploma account
        diploma.authority = ctx.accounts.authority.key();
        diploma.diploma_id = diploma_id;
        diploma.ipfs_hash = ipfs_hash; // Store the IPFS hash
        diploma.verified = true; // Mark as verified upon addition
        diploma.created_at = Clock::get()?.unix_timestamp;

        // Increment the count of diplomas in the registry
        diploma_registry.count += 1;

        msg!("Diploma added: {}", diploma.diploma_id);
        msg!("IPFS Hash: {}", diploma.ipfs_hash);
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
    // Note: No explicit 'verify_diploma' function on-chain.
    // Verification happens off-chain by comparing the hash from IPFS with the hash stored on-chain.
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
#[instruction(diploma_id: String, ipfs_hash: String)]
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
        // Calculate space for Diploma:
        // 8 (discriminator)
        // + 32 (authority: Pubkey)
        // + (4 + 100) (diploma_id: String max 100 chars)
        // + (4 + 60) (ipfs_hash: String max 60 chars)
        // + 1 (verified: bool)
        // + 8 (created_at: i64)
        space = 8 + 32 + (4 + 100) + (4 + 60) + 1 + 8,
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
    pub ipfs_hash: String,
    pub verified: bool,
    pub created_at: i64,
}

#[error_code]
pub enum DiplomaError {
    #[msg("Diploma ID cannot be empty")]
    EmptyDiplomaId,
    #[msg("Diploma ID is too long")]
    DiplomaIdTooLong,
    #[msg("IPFS Hash cannot be empty")]
    EmptyIpfsHash,
    #[msg("IPFS Hash is too long")]
    IpfsHashTooLong,
    #[msg("Diploma is already revoked")]
    DiplomaAlreadyRevoked,
}
