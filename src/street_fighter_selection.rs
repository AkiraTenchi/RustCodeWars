mod preloaded;
use preloaded::Direction;

pub fn street_fighter_selection(
    fighters: &[[&str; 6]; 2],
    position: &[i64; 2],
    moves: &[Direction],
) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let mut pos =  position.clone();
    // res.push(fighters[position[0] as usize][position[1] as usize].to_string());
    for mov in moves {
        match mov {
            &Direction::Up => {
                let (name, posi) = vertical(fighters, -1, &pos);
                res.push(name);
                pos = posi;
            },
            &Direction::Down => {
                let (name, posi) = vertical(fighters, 1, &pos);
                res.push(name);
                pos = posi;
            },
            &Direction::Left => {
                let (name, posi) = horizontal(fighters, -1, &pos); 
                res.push(name);
                pos = posi;
            },
            &Direction::Right => {
                let(name, posi) = horizontal(fighters, 1, &pos); 
                res.push(name);
                pos = posi;
            },
        }
    }
    res
}

fn vertical(fighters: &[[&str; 6]; 2], dir: i64, position: &[i64; 2]) -> (String, [i64; 2]){
    let mut pos = position.clone();
    if !(pos[0] + dir > 1) && !(pos[0] + dir < 0){
       pos = [pos[0] + dir, pos[1]]; 
    }
    (fighters[pos[0] as usize][pos[1] as usize].to_string(), pos)
}

fn horizontal(fighters: &[[&str; 6]; 2], dir: i64, position: &[i64; 2]) -> (String, [i64; 2]){
    let mut pos = position.clone();
    match pos[1] + dir {
        6 => pos = [pos[0], 0],
        -1 => pos = [pos[0], 5],
        0..=5 => pos = [pos[0], pos[1] + dir],
        _ => panic!(), 
    }
    (fighters[pos[0] as usize][pos[1] as usize].to_string(), pos)
}

#[cfg(test)]
mod tests {
    use super::{preloaded::Direction, street_fighter_selection};

    const FIGHTERS: [[&str; 6]; 2] = [
        ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
        ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"],
    ];

    #[test]
    fn few_moves() {
        use Direction::*;
        let moves = [Up, Left, Right, Left, Left];

        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ryu", "Vega", "Ryu", "Vega", "Balrog"],
        );
    }

    #[test]
    fn no_moves() {
        let moves: [Direction; 0] = [];

        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            [] as [String; 0]
        );
    }

    #[test]
    fn moving_left() {
        use Direction::*;
        let moves = [Left, Left, Left, Left, Left, Left, Left, Left];

        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Vega", "Balrog", "Guile", "Blanka", "E.Honda", "Ryu", "Vega", "Balrog"],
        );
    }

    #[test]
    fn moving_right() {
        use Direction::*;
        let moves = [Right, Right, Right, Right, Right, Right, Right, Right];

        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["E.Honda", "Blanka", "Guile", "Balrog", "Vega", "Ryu", "E.Honda", "Blanka"],
        );
    }

    #[test]
    fn uses_all_4_directions_clockwise_twice() {
        use Direction::*;
        let moves = [Up, Left, Down, Right, Up, Left, Down, Right];

        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ryu", "Vega", "M.Bison", "Ken", "Ryu", "Vega", "M.Bison", "Ken"],
        );
    }

    #[test]
    fn always_moving_down() {
        use Direction::*;
        let moves = [Down, Down, Down, Down];

        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ken", "Ken", "Ken", "Ken"],
        );
    }

    #[test]
    fn always_moving_up() {
        use Direction::*;
        let moves = [Up, Up, Up, Up];

        assert_eq!(
            street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
            ["Ryu", "Ryu", "Ryu", "Ryu"],
        );
    }
}
