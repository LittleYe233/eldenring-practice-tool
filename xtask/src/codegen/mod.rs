use anyhow::{bail, Result};

mod aob_scans;
mod item_ids;
mod params;

pub(crate) fn codegen(step: Option<&str>) -> Result<()> {
    match step {
        Some("aob_scans") => aob_scans::get_base_addresses(),
        Some("params") => params::codegen()?,
        Some("item_ids") => item_ids::codegen()?,
        None => {
            aob_scans::get_base_addresses();
            params::codegen()?;
            item_ids::codegen()?;
        },
        Some(other) => {
            bail!(
                "unknown codegen step: '{}'\navailable steps: aob_scans, params, item_ids",
                other
            );
        },
    }

    Ok(())
}
