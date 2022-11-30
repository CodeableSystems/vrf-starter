use anchor_lang::prelude::Clock;
use anchor_lang::prelude::*;
use std::str::FromStr;
pub mod util;

declare_id!("HofEGgj4wYPfPLoBJ42Q4rWM9QTLprHHXhTNXfRSJ9hP");

#[program]
pub mod vrf_starter {
    use super::*;

    pub fn get_random(ctx: Context<RequestRandom>) -> Result<()> {
        let mastervrf = Pubkey::from_str("3maGWtnB3Uo6oBKbi4WSpSiz1hNch7K6bJqYWmddiQtw").unwrap();
        let vrf = &mut ctx.accounts.vrf;
        if mastervrf != *vrf.key {
            return Err(ErrorCode::InvalidVrf.into());
        }
        let vrfdata = &ctx.accounts.vrf.to_account_info();
        let account_data = &mut &**vrfdata.try_borrow_mut_data()?;
        
        /* these two are simply a way to index into the randomness */
        /* other sources can be considered, or they can omitted if the random data changes every slot */
        let slot = Clock::get()?.slot as u32;
        let ts = Clock::get()?.unix_timestamp as u32;

        let buf = &account_data[41..297]; //[u8;256] -- complete set: [41..297]
        let idx = util::concat(&[123456789, ts % 10 + slot % 10]);
        let randomness: &[u8; 256] = &(&buf[..]).try_into().unwrap();
        let random_data = util::expand(*randomness, idx);
        msg!("random data: {:?}", random_data); //remove this line in production
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RequestRandom<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    vrf: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid random source")]
    InvalidVrf,
}
