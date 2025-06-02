use proc_result::ProcResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result: ProcResult = std::process::Command::new("examples/exit_code_0.sh")
        .status()?
        .into();
    result.ok()?;
    Ok(())
}
