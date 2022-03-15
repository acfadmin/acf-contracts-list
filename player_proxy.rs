use anchor_lang::prelude::*;

declare_id!("GZsQrh9g3J5rubktesKWfZJYuuF6Lwhnm3A8Ryvq9bqT");


use anchor_spl::token::{self, Transfer};
const LAMPORT_TO_SOL: f64 = 0.000000001;
#[program]
pub mod player_proxy {
    use super::*;
    pub fn initialize_player(ctx: Context<InitializePlayer>, pi_bump: u8) -> ProgramResult {
        ctx.accounts.details.bump = pi_bump.clone();
       
        Ok(())
    }

    pub fn update_player(ctx: Context<UpdatePlayer>, username: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.details;
        base_account.username = username;
        ctx.accounts.details.unbuilt_barns = 0;
        ctx.accounts.details.built_barns = 0;
        Ok(())
    }

    pub fn buy_barn_space(ctx: Context<BuyBarn>) -> ProgramResult {



        let seeds = &[b"barn_authority".as_ref(), &[ctx.accounts.barn_authority.bump]];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_acf_account.clone(),
            to: ctx.accounts.collection_account.clone(),
            authority: ctx.accounts.authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        let amount = (1.0 / LAMPORT_TO_SOL).round() as u64;
        if let Err(err) = token::transfer(cpi_ctx, amount) {
            return Err(err);
        }


        // let cpi_accounts = Transfer {
        //     from: ctx.accounts.user_acf_account.clone(),
        //     to: ctx.accounts.collection_account.clone(),
        //     authority: ctx.accounts.authority.clone(),
        // };
        // let amount = (1.0 / LAMPORT_TO_SOL).round() as u64;
        // let cpi_program = ctx.accounts.token_program.clone();
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // if let Err(err) = token::transfer(cpi_ctx, amount) {
        //     return Err(err);
        // }

        ctx.accounts.details.unbuilt_barns += 1;
        Ok(())
    }
    pub fn build_barn_space(ctx: Context<BuildBarn>) -> ProgramResult {
        if (ctx.accounts.details.unbuilt_barns <= 0) {
            msg!("Error: Barns have to bought before they can be built");
            panic!();
        } else {

        }
        ctx.accounts.details.unbuilt_barns -= 1;
        ctx.accounts.details.built_barns += 1;
        Ok(())
    }

    pub fn initialize_barn_authority(ctx: Context<InitializeBarnAuthority>, auth_bump: u8) -> ProgramResult {
        ctx.accounts.authority_data.bump = auth_bump.clone();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(pi_bump: u8)]
pub struct InitializePlayer<'info> {
    #[account(init, seeds = [b"playerinfo".as_ref(), user.key.as_ref()], bump=pi_bump, payer = user,  space = 128 + 128)]
    pub details: Account<'info, Details>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePlayer<'info> {
    #[account(mut, seeds = [b"playerinfo".as_ref(), user.key.as_ref()], bump = details.bump)]
    pub details: Account<'info, Details>,
    #[account(mut)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BuyBarn<'info> {
    #[account(mut)]
    pub user_acf_account: AccountInfo<'info>,
    #[account(mut)]
    pub collection_account: AccountInfo<'info>,
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, seeds = [b"playerinfo".as_ref(), authority.key.as_ref()], bump = details.bump)]
    pub details: Account<'info, Details>,
    #[account(mut, seeds = [b"barn_authority".as_ref()], bump = barn_authority.bump)]
    pub barn_authority: Account<'info, BarnAuthorityData>,
}

#[derive(Accounts)]
pub struct BuildBarn<'info> {
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(mut, seeds = [b"playerinfo".as_ref(), user.key.as_ref()], bump = details.bump)]
    pub details: Account<'info, Details>,
}


#[derive(Accounts)]
#[instruction(auth_bump: u8)]
pub struct InitializeBarnAuthority<'info> {
    #[account(init, seeds = [b"barn_authority".as_ref()], bump=auth_bump, payer = user,  space = 16)]
    pub authority_data: Account<'info, BarnAuthorityData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct BarnAuthorityData {
    pub bump: u8,
}


#[account]
#[derive(Default)]
pub struct Details {
    pub username: String,
    pub bump: u8,
    pub built_barns: u8,
    pub unbuilt_barns: u8,
}
