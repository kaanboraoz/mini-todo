use ratatui::{DefaultTerminal, Frame, style::Style, widgets::{Block, Borders, Padding, Widget}};

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()>
{
    loop {
        terminal.draw(render)?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

pub fn render(frame: &mut Frame) {

    let app_dash: Block<'_> = Block::new()
    .title("Dashboard")
    .borders(Borders::ALL)
    .border_style(Style::new()
    .yellow()).padding(Padding::new(50, 50, 50, 50));

    frame.render_widget(app_dash, frame.area());
}
