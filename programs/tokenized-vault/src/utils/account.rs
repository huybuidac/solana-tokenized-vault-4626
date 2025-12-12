use anchor_lang::{
    prelude::{Result, SolanaSysvar},
    solana_program::{
        account_info::AccountInfo, program::invoke, rent::Rent, system_instruction::transfer,
    },
    Lamports,
};

pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    rent_sysvar: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let rent = &Rent::from_account_info(&rent_sysvar)?;
    if rent.minimum_balance(account.data_len()) > account.get_lamports() {
        let extra_lamports = rent.minimum_balance(account.data_len()) - account.get_lamports();
        invoke(
            &transfer(payer.key, account.key, extra_lamports),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}
