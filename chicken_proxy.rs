use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Transfer};

use solana_program::sysvar::clock::Clock;
const LAMPORT_TO_SOL: f64 = 0.000000001;
declare_id!("5ELo1geYjDwpw5ur3qjpQbHMdZ48wGBZZyF4WDromhEk");

#[program]
pub mod chicken_proxy {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAuthority>, auth_bump: u8) -> ProgramResult {
        ctx.accounts.authority_data.bump = auth_bump.clone();
        Ok(())
    }
    // feed chicken
    pub fn feed_chicken(ctx: Context<FeedChicken>, amount: u64) -> ProgramResult {
        if ctx.accounts.stats.is_dead == true {
            return Err(ChickenError::ChickenDead.into());
        }
        let current_day = Clock::get().unwrap().unix_timestamp as u64;
        let mut time_passed = 0; 

        if current_day > ctx.accounts.stats.last_fed_date {
            time_passed = current_day.checked_sub(ctx.accounts.stats.last_fed_date).unwrap();
        }

        let cpi_accounts = Burn {
            mint: ctx.accounts.feed_mint.clone(),
            to: ctx.accounts.feed_to.clone(),
            authority: ctx.accounts.token_authority.clone(),
        };
        
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        if let Err(error) = token::burn(cpi_ctx, amount) {
            return Err(error);
        }

        // for testing
        // if ctx.accounts.stats.fertility < 255 {
        //     ctx.accounts.stats.fertility += 7;
        // }

        //let one_day = 86400;
        let one_day = 300; //set to 10 min for testing 

        if ctx.accounts.stats.last_fed_date == 0 {
            if ctx.accounts.stats.happiness < 255 {
                ctx.accounts.stats.happiness += 1;
            }
            if ctx.accounts.stats.fertility < 255 {
                ctx.accounts.stats.fertility += 1;
            }
        }
        let relative = time_passed / one_day;
        if ctx.accounts.stats.last_fed_date > 0 && relative >= 1 {

            if ctx.accounts.stats.fertility < 255 {
                ctx.accounts.stats.fertility += 1;
            }
            if ctx.accounts.stats.happiness < 255 {
                ctx.accounts.stats.happiness += 1;
            }

            msg!("Relative {}", relative); 

            if relative >= 1 && relative < 2 {
                if ctx.accounts.stats.hunger < 255 {
                    ctx.accounts.stats.hunger += 1;
                }
            }

            if relative >= 2 && relative < 3 {
                if ctx.accounts.stats.hunger < 255 {
                    ctx.accounts.stats.hunger += 2;
                }
            }
            if relative >= 3 && relative < 4 {
                if ctx.accounts.stats.hunger < 255 {
                    ctx.accounts.stats.hunger += 4;
                }
            }
            if relative >= 4 && relative < 5 {
                if ctx.accounts.stats.hunger < 255 {
                    ctx.accounts.stats.hunger += 8;
                }
            }
            if relative >= 5 && relative < 7 {
                if ctx.accounts.stats.hunger < 255 {
                    ctx.accounts.stats.hunger += 16;
                }
            }
            if relative >= 7 {
                ctx.accounts.stats.is_dead = true;
            }
        }
        msg!("Hunger {}", ctx.accounts.stats.hunger ); 
        msg!("Amount {}", amount as u8); 

        if ctx.accounts.stats.hunger > 0 {
            let amount_to_check = (amount as f64 * LAMPORT_TO_SOL).round() as u8; 
            if amount_to_check > ctx.accounts.stats.hunger {
                ctx.accounts.stats.hunger = 0;
            } else {
                ctx.accounts.stats.hunger = ctx.accounts.stats.hunger.checked_sub(amount_to_check).unwrap();
            }
        }

        // return Err(ChickenError::ChickenDead.into());
        
        ctx.accounts.stats.last_fed_date = current_day;
        msg!("Last fed date {}", current_day); 
        Ok(())
    }

    //breed_chicken
    pub fn breed_chicken(ctx: Context<BreedChicken>) -> ProgramResult {
        if ctx.accounts.stats_for_first_chicken.is_dead == true
            || ctx.accounts.stats_for_second_chicken.is_dead == true
        {
            return Err(ChickenError::ChickenDead.into());
        }
        let stats_for_first_chicken = &mut ctx.accounts.stats_for_first_chicken;
        let stats_for_second_chicken = &mut ctx.accounts.stats_for_second_chicken;

        if stats_for_first_chicken.breeds == 0 || stats_for_second_chicken.breeds == 0 {
            return Err(ChickenError::NoBreedsLeft.into());
        }

        if stats_for_first_chicken.fertility < 7 || stats_for_second_chicken.fertility < 7 {
            return Err(ChickenError::NoFertilityLeft.into());
        }



        let seeds = &[b"breeding_authority".as_ref(), &[ctx.accounts.acf_authority.bump]];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.acf_from.clone(),
            to: ctx.accounts.acf_to.clone(),
            authority: ctx.accounts.user.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        let amount = (1.0 / LAMPORT_TO_SOL).round() as u64;
        if let Err(err) = token::transfer(cpi_ctx, amount) {
            return Err(err);
        }


        // let cpi_accounts = Burn {
        //     mint: ctx.accounts.acf_mint.clone(),
        //     to: ctx.accounts.acf_to.clone(),
        //     authority: ctx.accounts.user.clone(),
        // };
        // let cpi_program = ctx.accounts.token_program.clone();
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // let amount = (1.0 / LAMPORT_TO_SOL).round() as u64;

        // if let Err(error) = token::burn(cpi_ctx, amount) {
        //     return Err(error);
        // }

        stats_for_first_chicken.fertility -= 7;
        stats_for_second_chicken.fertility -= 7;

        if stats_for_first_chicken.happiness < 255 {
            stats_for_first_chicken.happiness += 1;
        }
        if stats_for_second_chicken.happiness < 255 {
            stats_for_second_chicken.happiness += 1;
        }

        if stats_for_first_chicken.health < 253 {
            stats_for_first_chicken.health += 1;
        }

        if stats_for_second_chicken.health < 255 {
            stats_for_second_chicken.health += 1;
        }
        stats_for_first_chicken.breeds -= 1;
        stats_for_second_chicken.breeds -= 1;

        Ok(())
    }

    pub fn burn_chicken(ctx: Context<BurnChicken>) -> ProgramResult {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.clone(),
            to: ctx.accounts.to.clone(),
            authority: ctx.accounts.token_authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        if let Err(error) = token::burn(cpi_ctx, 1) {
            return Err(error);
        }
        Ok(())
    }

    pub fn cull_chicken(ctx: Context<CullChicken>) -> ProgramResult {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.clone(),
            to: ctx.accounts.to.clone(),
            authority: ctx.accounts.token_authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        if let Err(error) = token::burn(cpi_ctx, 1) {
            return Err(error);
        }

        let seeds = &[b"culling_authority".as_ref(), &[ctx.accounts.cull_authority.bump]];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.feed_from.clone(),
            to: ctx.accounts.feed_to.clone(),
            authority: ctx.accounts.feed_mint_authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        let amount = (20.0 / LAMPORT_TO_SOL).round() as u64;
        if let Err(err) = token::transfer(cpi_ctx, amount) {
            return Err(err);
        }

        Ok(())
    }

    pub fn create_chicken_stats_account(
        ctx: Context<ChickenStatsAccount>,
        bump: u8,
    ) -> ProgramResult {
        ctx.accounts.stats.bump = bump.clone();
        ctx.accounts.stats.breeds = 5;
        Ok(())
    }

    pub fn update_chicken_stats_account(
        ctx: Context<UpdateChickenStats>,
        key: String,
        value: u8,
    ) -> ProgramResult {
        if key == "hunger" {
            ctx.accounts.stats.hunger = value.clone();
        }
        if key == "happiness" {
            ctx.accounts.stats.happiness = value.clone();
        }
        if key == "fertility" {
            ctx.accounts.stats.fertility = value.clone();
        }
        if key == "health" {
            ctx.accounts.stats.health = value.clone();
        }
        if key == "breeds" {
            ctx.accounts.stats.health = value.clone();
        }
        if key == "feeds" {
            ctx.accounts.stats.health = value.clone();
        }
        Ok(())
    }

    pub fn hatch_chicken(ctx: Context<HatchChicken>) -> ProgramResult {
        let cpi_accounts = Burn {
            mint: ctx.accounts.egg_mint.clone(),
            to: ctx.accounts.egg_to.clone(),
            authority: ctx.accounts.authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        if let Err(error) = token::burn(cpi_ctx, 1) {
            return Err(error);
        }

        Ok(())
    }

    pub fn initialize_cull_authority(ctx: Context<InitializeCullAuthority>, auth_bump: u8) -> ProgramResult {
        ctx.accounts.authority_data.bump = auth_bump.clone();
        Ok(())
    }
    pub fn initialize_breed_authority(ctx: Context<InitializeBreedAuthority>, auth_bump: u8) -> ProgramResult {
        ctx.accounts.authority_data.bump = auth_bump.clone();
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(auth_bump: u8)]
pub struct InitializeAuthority<'info> {
    #[account(init, seeds = [b"chicken_authority".as_ref()], bump=auth_bump, payer = user,  space = 16)]
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
pub struct FeedChicken<'info> {
    #[account(mut)]
    pub feed_mint: AccountInfo<'info>,
    #[account(mut)]
    pub feed_to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, seeds = ["stats".as_bytes(), &id().as_ref(), mint.key.as_ref()], bump = stats.bump)]
    pub stats: Account<'info, ChickenStats>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub token_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BreedChicken<'info> {
    #[account(mut)]
    pub acf_mint: AccountInfo<'info>,
    #[account(mut)]
    pub acf_to: AccountInfo<'info>,
    #[account(mut)]
    pub acf_from: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(mut, seeds = ["stats".as_bytes(), &id().as_ref(), chicken_one_mint.key.as_ref()], bump = stats_for_first_chicken.bump)]
    pub stats_for_first_chicken: Account<'info, ChickenStats>,
    #[account(mut, seeds = ["stats".as_bytes(), &id().as_ref(), chicken_two_mint.key.as_ref()], bump = stats_for_second_chicken.bump)]
    pub stats_for_second_chicken: Account<'info, ChickenStats>,
    #[account(mut, seeds = [b"breeding_authority".as_ref()], bump = acf_authority.bump)]
    pub acf_authority: Account<'info, BreedAuthorityData>,
    #[account(mut)]
    pub chicken_one_mint: AccountInfo<'info>,
    #[account(mut)]
    pub chicken_two_mint: AccountInfo<'info>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnChicken<'info> {
    #[account(signer)]
    pub token_authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CullChicken<'info> {
    #[account(signer)]
    pub token_authority: AccountInfo<'info>,
    #[account(mut)]
    pub feed_mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    #[account(mut, seeds = [b"culling_authority".as_ref()], bump = cull_authority.bump)]
    pub cull_authority: Account<'info, CullAuthorityData>,
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub feed_to: AccountInfo<'info>,
    #[account(mut)]
    pub feed_from: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct HatchChicken<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    pub egg_mint: AccountInfo<'info>,
    #[account(mut)]
    pub egg_to: AccountInfo<'info>,

    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(stats_account_bump: u8)]
pub struct ChickenStatsAccount<'info> {
    #[account(init, seeds = ["stats".as_bytes(), &id().as_ref(), mint.key.as_ref()], bump=stats_account_bump, payer = user,  space = 128 + 128)]
    pub stats: Account<'info, ChickenStats>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
}



#[derive(Accounts)]
pub struct UpdateChickenStats<'info> {
    #[account(mut, seeds = ["stats".as_bytes(), &id().as_ref(), mint.key.as_ref()], bump = stats.bump)]
    pub stats: Account<'info, ChickenStats>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct ChickenStats {
    dna: String,
    bump: u8,
    health: u8,
    hunger: u8,
    happiness: u8,
    fertility: u8,
    feeds: u8,
    breeds: u8,
    last_fed_date: u64,
    list_price_lamports: u64,
    current_owner: Pubkey,
    for_sale: bool,
    is_dead: bool,
}

#[derive(Accounts)]
#[instruction(auth_bump: u8)]
pub struct InitializeCullAuthority<'info> {
    #[account(init, seeds = [b"culling_authority".as_ref()], bump=auth_bump, payer = user,  space = 16)]
    pub authority_data: Account<'info, CullAuthorityData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct CullAuthorityData {
    pub bump: u8,
}



#[derive(Accounts)]
#[instruction(auth_bump: u8)]
pub struct InitializeBreedAuthority<'info> {
    #[account(init, seeds = [b"breeding_authority".as_ref()], bump=auth_bump, payer = user,  space = 16)]
    pub authority_data: Account<'info, BreedAuthorityData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct BreedAuthorityData {
    pub bump: u8,
}


#[error]
pub enum ChickenError {
    #[msg("Chicken is dead")]
    ChickenDead,
    #[msg("Chickens cannot be bred as they don't have any breeds left")]
    NoBreedsLeft,
    #[msg("Chickens cannot be bred as they don't have any fertility left")]
    NoFertilityLeft,
}
