use std::io::{stdout, Result};
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    layout::Alignment,
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

pub fn init(name: &str) -> Result<()> {
    // Iniciar en modo de pantalla alternativa y modo sin buffer
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // Crear el backend y el terminal
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Limpiar la pantalla
    terminal.clear()?;

    loop {
        // Dibujar en la pantalla
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(format!("Bienvenido {} (presione 'q' para salir)", name))
                    .alignment(Alignment::Center)
                    .white()
                    .on_green(),
                area,
            );
        })?;

        // Revisar si hay eventos de entrada
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Restaurar el modo de terminal original
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
