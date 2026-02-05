use clap::{Parser, ValueEnum};
use swayipc::Connection;

#[derive(Parser)]
struct Cli {
    #[arg(value_enum)]
    direction: Direction,

    #[arg(long, short)]
    move_window: bool,

    /// Número máximo de escritorios (por defecto 10)
    #[arg(long, default_value_t = 6)]
    max: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
enum Direction {
    Next,
    Prev,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut conn = Connection::new()?;

    // 1. Obtener el workspace enfocado actualmente
    let workspaces = conn.get_workspaces()?;
    let current_ws = workspaces
        .iter()
        .find(|w| w.focused)
        .ok_or("No hay workspace enfocado")?;

    let current_num = current_ws.num;

    // 2. Calcular el siguiente número (Lógica circular 1 al MAX)
    let target_num = match cli.direction {
        Direction::Next => {
            if current_num >= cli.max { 1 } else { current_num + 1 }
        }
        Direction::Prev => {
            if current_num <= 1 { cli.max } else { current_num - 1 }
        }
    };

    // 3. Ejecutar comando
    let command = if cli.move_window {
        format!(
            "move container to workspace number {t}; workspace number {t}",
            t = target_num
        )
    } else {
        format!("workspace number {}", target_num)
    };

    conn.run_command(&command)?;
    Ok(())
}
