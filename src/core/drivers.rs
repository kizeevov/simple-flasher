use std::env::current_dir;

pub fn install_drivers() -> Result<(), ()> {
    let mut current_dir = current_dir().map_err(|_| ())?;
    current_dir.push("drivers\\CH340\\CH341SER.INF");

    let output = std::process::Command::new("pnputil")
        .current_dir("drivers/CH340/")
        .arg("-i")
        .arg("-a")
        .arg(current_dir)
        .output()
        .map_err(|_| ())?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("{:?}", stdout);
    if stdout.to_lowercase().contains("oem") {
        return Ok(());
    }

    Err(())
}
