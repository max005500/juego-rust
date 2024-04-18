use super::tablero::Tablero;

pub struct Movimiento {
    pub new_coords: (usize, usize),
    pub castigo: bool,
    pub llave: bool,
    pub end: bool,
    pub mat: Vec<Vec<i8>>,
}

impl Movimiento {
    pub fn next(mov: char, board: &mut Tablero) -> Movimiento {
        let (mut row, mut col) = board.main_coords;
        let aux = board.main_coords;
        match mov {
            'w' => row -= 1,
            'd' => col += 1,
            's' => row += 1,
            'a' => col -= 1,
            _ => println!("ingrese movimiento valido"),
        };
        board.main_coords = (row, col);
        let (mat, castigo, llave, casa, end) = reglas(board);

        let new_coords: (usize, usize);

        if !casa {
            new_coords = board.main_coords;
        } else {
            new_coords = aux;
        }

        Movimiento {
            new_coords,
            mat,
            castigo,
            llave,
            end,
        }
    }
}

fn reglas(board: &mut Tablero) -> (Vec<Vec<i8>>, bool, bool, bool, bool) {
    let (row, col) = board.main_coords;
    let (rowc, colc) = board.cofre_coords;
    let mut mat = board.tab.clone();

    let cofre = board.main_coords == board.cofre_coords;
    let casa = mat[row][col] == -3;
    let trampa = mat[row][col] == -2;
    let huevo = mat[row][col] > 0;
    let llavef = mat[rowc][colc] == 0;
    let end = mat[row][col] == -1 && llavef;

    if cofre {
        mat[row][col] = 0;
        (mat, trampa, cofre, casa, end)
    } else {
        if huevo {
            mat = mat
                .into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|f| f - (if f <= 9 && f > 0 { 1 } else { 0 }))
                        .collect::<Vec<i8>>()
                })
                .collect::<Vec<Vec<i8>>>();
        }
        (mat, trampa, llavef, casa, end)
    }
}

