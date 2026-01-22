use crate::ui::app;

mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?; 
    Ok(())
}
