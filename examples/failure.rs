use proc_result::ToProcResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new("examples/exit_code_1.sh")
        .status()?
        .to_proc_result()?;
    Ok(())
}
