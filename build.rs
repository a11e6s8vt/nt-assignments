use clap_complete::{generate_to, shells::*};
use clap_mangen::Man;
use std::fs::File;
use std::io::Error;
use std::path::Path;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    let app_name = "nt-tools";
    let mut cmd = cli();
    cmd.set_bin_name(app_name);
    //generate_to(Bash, &mut cmd, app_name, &outdir)?;
    //generate_to(Zsh, &mut cmd, app_name, &outdir)?;
    //generate_to(Fish, &mut cmd, app_name, &outdir)?;
    generate_to(PowerShell, &mut cmd, app_name, &outdir)?;
    //generate_to(Elvish, &mut cmd, app_name, &outdir)?;

    // let file = Path::new("man-page").join("nt-tools.1");
    // std::fs::create_dir_all("man-page")?;
    // let mut file = File::create(file)?;

    // Man::new(cmd).render(&mut file)?;

    Ok(())
}
