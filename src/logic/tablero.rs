use rand::*;

pub struct Tablero {
    pub tab: Vec<Vec<i8>>,
    pub cofre_coords: (usize, usize),
    pub main_coords: (usize, usize),
}

impl Tablero {
    pub fn new(mat: &Vec<Vec<i8>>) -> Tablero {
        let mut tab = mat.to_vec();

        for i in 5..15 {
            for j in 2..7 {
                tab[i][j] = -3 //casa
            }
        }
        let mut a = random_element(&-1, &1, &mut tab);
        a = random_element(&-2, &15, a); //trampas
        a = random_element(&-8, &1, a); //personaje
        a = random_element(&9, &10, a); //huevos
        a = random_element(&6, &1, a); //cofre 1

        tab = a.to_vec();

        let cofre_coords = find_coord(&tab, &6);
        let main_coords = find_coord(&tab, &-8);

        Tablero {
            tab,
            cofre_coords,
            main_coords,
        }
    }
}

fn random_element<'a>(val: &i8, u: &u8, mat: &'a mut Vec<Vec<i8>>) -> &'a mut Vec<Vec<i8>> {
    let mut rng = thread_rng();
    let mut cont = 0;
    loop {
        let randomy = rng.gen_range(0..mat.len());
        let randomx = rng.gen_range(0..mat[1].len());

        if mat[randomy][randomx] == 0 {
            mat[randomy][randomx] = *val;
            cont += 1;
            if cont == *u {
                break;
            }
        }
    }
    mat
}

fn find_coord(mat: &Vec<Vec<i8>>, val: &i8) -> (usize, usize) {
    let (y, x) = (
        mat.into_iter()
            .position(|x| x.into_iter().any(|f| f == val))
            .unwrap(),
        mat.into_iter()
            .find(|x| x.to_vec().into_iter().any(|f| f == *val))
            .unwrap()
            .into_iter()
            .position(|&f| f == *val)
            .unwrap(),
    );

    (y, x)
}
