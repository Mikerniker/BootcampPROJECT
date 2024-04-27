use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult; //ADDed
                                                            
declare_id!("DTnSHxZHtvsAP7orMXNiDYFU694oBHZUjiXZn8gbV7Xi");

#[program]
pub mod bootcamp {
    use super::*;

    //create PDA
    pub fn create_bootcamp(
        ctx: Context<CreateBootcamp>,
        name: String,
        description: String,
        duration: u64,
        deposit_amount: u64,
        // status: bool,
        // refund: bool,
    ) -> ProgramResult {
        let bootcamp = &mut ctx.accounts.bootcamp;
        bootcamp.name = name;
        bootcamp.balance = 0;
        bootcamp.description = description;
        bootcamp.duration = duration;
        bootcamp.deposit_amount = deposit_amount;
        // bootcamp.status = status;
        // bootcamp.refund = refund;
        bootcamp.owner = *ctx.accounts.user.key; //owner is Hencode Club
        Ok(())
    }

    // Transfer the deposit from the student account to the bootcamp account.
    pub fn student_deposit(
        ctx: Context<StudentDeposit>, amount: u64) -> ProgramResult {
        let transaction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),  //students account/ public key
            &ctx.accounts.bootcamp.key(),   //destination PDA
            amount
        );

        anchor_lang::solana_program::program::invoke(
            &transaction,
            &[
                ctx.accounts.user.to_account_info(),  //students account
                ctx.accounts.bootcamp.to_account_info()   //bootcamp pda
            ],
        )?;
        (&mut ctx.accounts.bootcamp).balance += amount;  //updated balance with user payment to bootcamp account
        Ok(())
    }

    // Refund student's deposit 
    pub fn refund(ctx: Context<Refund>, amount: u64) -> ProgramResult {
        let bootcamp = &mut ctx.accounts.bootcamp;
        let user = &mut ctx.accounts.user;
        if bootcamp.owner != user.key() {
            return Err(ProgramError::IncorrectProgramId);
        }
        
        let rent = Rent::get()?.minimum_balance(bootcamp.to_account_info().data_len());

        if **bootcamp.to_account_info().lamports.borrow() - rent < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        **bootcamp.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateBootcamp<'info> {
    #[account(
        init,
        seeds = [b"bootcamp", user.key().as_ref()],
        bump,
        payer = user,
        space = 5000,
    )]
    pub bootcamp: Account<'info, BootcampAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BootcampAccount {
    pub name: String,
    pub balance: u64,
    pub owner: Pubkey,
    pub description: String,
    pub duration: u64,
    pub deposit_amount: u64, // The deposit that each student should pay (lamports)
    // pub status: bool,
    // pub refund: bool,
}

#[derive(Accounts)]
pub struct StudentDeposit<'info> {   
   #[account(mut)]
    pub bootcamp: Account<'info, BootcampAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Refund<'info>{
    #[account(mut)]
    pub bootcamp: Account<'info, BootcampAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}    
