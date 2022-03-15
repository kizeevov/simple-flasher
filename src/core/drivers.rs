pub fn install_drivers() -> Result<(), ()> {
    std::process::Command::new("drivers/CH340/SETUP.EXE")
        .current_dir("drivers/CH340/")
        .spawn()
        .map_err(|_| ())?;

    Ok(())
}
