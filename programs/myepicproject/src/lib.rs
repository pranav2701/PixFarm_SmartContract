use anchor_lang::prelude::*;

declare_id!("CmKwdS7AEeFY8p21X9UEHao8CPK6B6JgwSzeGFdafRSG");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        // base_account.total_pix = 0;

        Ok(())
    }

    // pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    //     let base_account = &mut ctx.accounts.base_account;
    //     let user = &mut ctx.accounts.user;

    //     let item = ItemStruct{
    //         gif_link: gif_link.to_string(),
    //         user_address: *user.to_account_info().key,
    //         votes: 0
    //     };

    //     base_account.gif_list.push(item);
    //     base_account.total_gifs += 1;
    //     Ok(())
    // }

    pub fn update_hex(ctx: Context<AddHex>, index: u64, hex_val: String) -> ProgramResult {
        //up_vote
        let base_account = &mut ctx.accounts.base_account;

        let i = index as u64;

        // base_account.hex_list[i] = hex_val.to_string();

        let item = ItemStruct {
            hex_code: hex_val.to_string(),
            user_address: *base_account.to_account_info().key,
            pix_id: i,
        };

        base_account.hex_list.push(item);

        Ok(())
    }

    // pub fn down_vote(ctx: Context<UpdateHex>, index: u64, hex_val: String) -> ProgramResult {
    //     let base_account = &mut ctx.accounts.base_account;

    //     let i = index as usize;

    //     hex_list[i] = hex_val;

    //     Ok(())
    // }
}

#[error]
pub enum Err {
    #[msg("No item with that url found")]
    NoItemFound,
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddHex<'info> {
    //AddGif
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    // #[account(mut)]
    // pub user: Signer<'info>,
}

// #[derive(Accounts)]
// pub struct UpdateHex<'info> { //UpdateGif
//     #[account(mut)]
//     pub base_account: Account<'info, BaseAccount>,
// }

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub hex_code: String, //giflink
    pub user_address: Pubkey,
    pub pix_id: u64, //votes
}

#[account]
pub struct BaseAccount {
    // pub total_pix: u64,
    pub hex_list: Vec<ItemStruct>, //giflist
}
