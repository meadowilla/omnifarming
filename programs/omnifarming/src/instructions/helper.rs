use anchor_lang::{prelude::*, system_program};

pub fn preview_deposit<'info>(info: &OmniFarmingInfo, assets: u64) -> Result<(u64)> {
    let shares = if info.total_shares == 0 || info.total_assets == 0 {
        // 1:1 ratio
        assets
    } else {
        // calculate shares using ratio
        (assets as u128)
            .checked_mul(info.total_shares as u128)
            .unwrap()
            .checked_div(info.total_assets as u128)
            .unwrap() as u64
    };
    Ok((shares))
}





