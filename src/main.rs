use colored::*;
use game_huevos::logic::{mov::Movimiento, tablero::Tablero};

fn main() {
    let mut matrix: Tablero = Tablero::new(&vec![vec![0; 12]; 20]);
    let mut time = 0;
    let (rowc, colc) = matrix.cofre_coords;
    let mut huevitos = 0;
    let mut conejitos = 0;
    let mut llave = 0;

    loop {
        let (row, col) = matrix.main_coords;
        let aux = matrix.tab[row][col];
        matrix.tab[row][col] = -8;
        let mut mov: String = String::new();
        println!(
            "pocision columna: {} fila: {} tiempo: {} segundos",
            col, row, time
        );
        println!(
            "llave: {} conejitos: {} huevitos: {} ",
            llave, conejitos, huevitos
        );
        println!("**************************************");
        for i in matrix.tab.iter() {
            print!("|");
            for j in i.into_iter() {
                match j {
                    -3 => print!(" {} ", "X".blue().bold()),
                    -8 => print!(" {} ", "P".yellow().bold().italic()),
                    -2 => print!(" {} ", "T".red().bold()),
                    -1 => print!(" {} ", "C".cyan().bold()),
                    6 => print!(" {} ", "L".white().bold()),
                    1..=5 => print!(" {} ", "H".magenta().bold()),
                    7..=9 => print!(" {} ", "H".magenta().bold()),
                    _ => print!(" {} ", "*".green().bold()),
                }
            }
            print!("|");
            println!("");
        }
        println!("**************************************");
        println!(
            "{:?}",
            matrix
                .tab
                .iter()
                .find(|x| x.to_vec().into_iter().any(|f| f == -8))
                .unwrap()
        );
        matrix.tab[row][col] = if aux < -1 { 0 } else { aux };
        huevitos += if aux > 0 { 1 } else { 0 };

        std::io::stdin().read_line(&mut mov).unwrap_or_default();
        let movc: Vec<char> = mov.chars().collect();

        if movc[0] == 'e' {
            break;
        }
        let mo = Movimiento::next(movc[0], &mut matrix);

        matrix.tab = mo.mat;
        matrix.main_coords = mo.new_coords;

        time += if mo.castigo { 15 } else { 1 };
        (llave, conejitos, matrix.tab[rowc][colc]) = (
            if mo.llave { 1 } else { 0 },
            if mo.llave { 6 } else { 0 },
            if mo.llave { 0 } else { 6 },
        );

        if mo.end {
            break;
        }
        std::process::Command::new("clear").status().unwrap();
    }
}

