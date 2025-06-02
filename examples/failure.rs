use proc_result::ProcResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let status = std::process::Command::new("examples/exit_code_1.sh").status()?;
    let result: ProcResult = status.into();
    result.ok()?;
    Ok(())
}
