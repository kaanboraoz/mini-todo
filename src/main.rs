use crate::ui::app;

mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?; // Burada app fonksiyonunu bir değer olarak gönderiyoruz fonksiyonu call eden ve argüman gönderne run fonksiyonudur
    Ok(())
}
