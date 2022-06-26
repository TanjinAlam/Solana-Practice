use anchor_lang::prelude::*;

declare_id!("Dc7vD8qEpaGm6qr43HhUdLLeeChKDp4qpNvy39ogeqJT");

#[program]
pub mod gm_anchor {
   use super::*;
   pub fn execute(ctx: Context<Execute>, name: String) -> Result<()> {
       let gm_account = &mut ctx.accounts.gm_account;
       // taking input parameter from client and store it.
       gm_account.name = name;
       msg!("GM {}", gm_account.name);
       Ok(())
   }
}

#[derive(Accounts)]
pub struct Execute<'info> {
   #[account(init, payer = user, space = 8 + 32)]
   pub gm_account: Account<'info, GreetingAccount>,
   #[account(mut)]
   pub user: Signer<'info>,
   pub system_program: Program<'info, System>,
}

#[account]
pub struct GreetingAccount {
   pub name: String,
}