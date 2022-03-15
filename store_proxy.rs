use anchor_lang::prelude::*;

declare_id!("2sAY4rakKUDG4auJMcwdU4PUK51xSkp2d1wDwzF2Vyai");

use anchor_spl::token::{self, SetAuthority, Transfer};
// use anchor_spl::token::{self, Burn, MintTo, };
#[program]
pub mod store_proxy {
    use super::*;

    pub fn list_for_sale(ctx: Context<ListForSaleDetails>) -> ProgramResult {

        let cpi_accounts = Transfer {
            from: ctx.accounts.owner_token_account.clone(),
            to: ctx.accounts.store_token_account.clone(),
            authority: ctx.accounts.token_owner.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        if let Err(err) = token::transfer(cpi_ctx, 1) {
            return Err(err);
        }

        Ok(())
    }

    pub fn delist_from_sale(ctx: Context<DelistFromSaleDetails>) -> ProgramResult {
        let seeds = &[b"store_authority".as_ref(), &[ctx.accounts.authority_data.bump]];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.store_token_account.clone(),
            to: ctx.accounts.owner_token_account.clone(),
            authority: ctx.accounts.store_authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        if let Err(err) = token::transfer(cpi_ctx, 1) {
            return Err(err);
        }

        Ok(())
    }

    pub fn deduct_buying_charges(
        ctx: Context<DeductBuyingChargesDetails>,
        list_price_lamports: u64,
        fees_lamports: u64
    ) -> ProgramResult {
        let source = ctx.accounts.buyer.clone();
        let destination = ctx.accounts.seller.clone();
        let liquidity_account = ctx.accounts.liquidity_account.clone(); 

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &source.key,
            &destination.key,
            list_price_lamports,
        );

        let res = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                source.to_account_info().clone(),
                destination.to_account_info().clone(),
                ctx.accounts.system_program.clone(),
            ],
        )?;

        let ix_liquidity = anchor_lang::solana_program::system_instruction::transfer(
            &source.key,
            &liquidity_account.key,
            fees_lamports,
        );

        let res_liquidity = anchor_lang::solana_program::program::invoke(
            &ix_liquidity,
            &[
                source.to_account_info().clone(),
                liquidity_account.to_account_info().clone(),
                ctx.accounts.system_program.clone(),
            ],
        )?;

        Ok(())
    }

    pub fn buy_chicken_from_store(
        ctx: Context<BuyChickenDetails>,
        list_price_lamports: u64,
        fees_lamports: u64
    ) -> ProgramResult {
        let source = ctx.accounts.buyer.clone();
        let destination = ctx.accounts.seller.clone();
        let liquidity_account = ctx.accounts.liquidity_account.clone(); 

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &source.key,
            &destination.key,
            list_price_lamports,
        );

        let res = anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                source.to_account_info().clone(),
                destination.to_account_info().clone(),
                ctx.accounts.system_program.clone(),
            ],
        )?;

        let ix_liquidity = anchor_lang::solana_program::system_instruction::transfer(
            &source.key,
            &liquidity_account.key,
            fees_lamports,
        );

        let res_liquidity = anchor_lang::solana_program::program::invoke(
            &ix_liquidity,
            &[
                source.to_account_info().clone(),
                liquidity_account.to_account_info().clone(),
                ctx.accounts.system_program.clone(),
            ],
        )?;

        let seeds = &[b"store_authority".as_ref(), &[ctx.accounts.authority_data.bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.store_token_account.clone(),
            to: ctx.accounts.buyer_token_account.clone(),
            authority: ctx.accounts.store_authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        if let Err(err) = token::transfer(cpi_ctx, 1) {
            return Err(err);
        }

        Ok(())
    }

    pub fn initialize(ctx: Context<InitializeAuthority>, auth_bump: u8) -> ProgramResult {
        ctx.accounts.authority_data.bump = auth_bump.clone();
        Ok(())
    }

    pub fn initialize_secondary(ctx: Context<InitializeSecondaryAuthority>, auth_bump: u8) -> ProgramResult {
        ctx.accounts.authority_data.bump = auth_bump.clone();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(auth_bump: u8)]
pub struct InitializeAuthority<'info> {
    #[account(init, seeds = [b"store_authority".as_ref()], bump=auth_bump, payer = user,  space = 16)]
    pub authority_data: Account<'info, AuthorityData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct AuthorityData {
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(auth_bump: u8)]
pub struct InitializeSecondaryAuthority<'info> {
    #[account(init, seeds = [b"store_second_authority".as_ref()], bump=auth_bump, payer = user,  space = 16)]
    pub authority_data: Account<'info, SecondaryAuthorityData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct SecondaryAuthorityData {
    pub bump: u8,
}


#[derive(Accounts)]
#[instruction(sale_bump: u8)]
pub struct CreateSaleDetails<'info> {
    #[account(init, seeds = ["acfsaledetails".as_bytes(), &id().as_ref(), mint.key.as_ref()], bump=sale_bump, payer = user,  space = 128 + 128)]
    pub sale_details: Account<'info, SaleDetails>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ListForSaleDetails<'info> {
    #[account(signer)]
    pub token_owner: AccountInfo<'info>,
    #[account(mut)]
    pub owner_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub store_token_account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DelistFromSaleDetails<'info> {
    #[account(mut)]
    pub store_authority: AccountInfo<'info>,
    #[account(mut)]
    pub owner_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub store_token_account: AccountInfo<'info>,
    #[account(mut, seeds = [b"store_authority".as_ref()], bump = authority_data.bump)]
    pub authority_data: Account<'info, AuthorityData>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DeductBuyingChargesDetails<'info> {
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: AccountInfo<'info>,
    #[account(mut)]
    pub liquidity_account: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}




#[derive(Accounts)]
pub struct BuyChickenDetails<'info> {
    #[account(mut)]
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub store_authority: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: AccountInfo<'info>,
    #[account(mut)]
    pub liquidity_account: AccountInfo<'info>,
    #[account(mut)]
    pub buyer_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub store_token_account: AccountInfo<'info>,
    #[account(mut, seeds = [b"store_authority".as_ref()], bump = authority_data.bump)]
    pub authority_data: Account<'info, AuthorityData>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}


#[account]
#[derive(Default)]
pub struct SaleDetails {
    list_price_lamports: u64,
    current_owner: Pubkey,
    bump: u8,
}

// simple story:
// this progrma will contain a pda that will hold chickens and their sale information until they can be sold
