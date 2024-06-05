use anchor_lang::prelude::*;

declare_id!("559uNwHJmMoKtbeWthqWdNg5yBvBQMtx52QjAPbXCcHo");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
