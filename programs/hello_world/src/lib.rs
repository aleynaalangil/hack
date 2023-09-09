use anchor_lang::prelude::*;

declare_id!("6uN7xTWQHT31nLaeoREvmcL79bqd6KAzVvmGpn3pRCYv");

// #[program]
// pub mod hello_world {
//     use super::*;
//
//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }
//
// #[derive(Accounts)]
// pub struct Initialize {}
#[program]
mod hello_world {
    use super::*;
    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!("Hello World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}