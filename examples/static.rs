use minus::error::MinusError;

fn main() -> Result<(), MinusError> {
    for _ in 0..2 {
        let output = minus::Pager::new();

        output.set_exit_strategy(minus::ExitStrategy::PagerQuit)?;

        for i in 0..=100 {
            output.push_str(&format!("{}\n", i))?;
        }

        minus::page_all(output)?;
        }
    Ok(())
}
