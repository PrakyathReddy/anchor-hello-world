use anchor_lang::prelude::*;
declare_id!("4DEfPTWzbW1LYWickNN7Hv9ksE9yE17xnJgrFacqnAER");
#[program]
pub mod hello_world {
    use super::*;
    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, from solana smart contract");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize {}