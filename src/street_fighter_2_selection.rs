pub mod preloaded;

use preloaded::Direction;

use self::preloaded::Position;

pub fn super_street_fighter_selection(
    fighters: &[&[&str]],
    position: Position,
    moves: &[Direction],
) -> Vec<String> {
    let mut res = Vec::<String>::new();
    let mut pos =  position.clone();
    for mov in moves {
        match mov {
            &Direction::Up => {
                let (name, posi) = vertical(fighters, -1, pos);
                res.push(name);
                pos = posi;
            },
            &Direction::Down => {
                let (name, posi) = vertical(fighters, 1, pos);
                res.push(name);
                pos = posi;
            },
            &Direction::Left => {
                let (name, posi) = horizontal(fighters, -1, pos); 
                res.push(name);
                pos = posi;
            },
            &Direction::Right => {
                let(name, posi) = horizontal(fighters, 1, pos); 
                res.push(name);
                pos = posi;
            },
        }
    }
    res
}

fn vertical(fighters: &[&[&str]], direc: i8, position: Position) -> (String, Position){
    let mut pos = position.clone();
    let len = fighters.len() - 1;
    let dir : usize = 1;
    if direc == 1 {
        if !(pos.y + dir > len){
           pos.y += dir
        }
    } else if direc == -1{
        if !(pos.y.wrapping_sub(dir) > len){
           pos.y = pos.y.wrapping_sub(dir);
        }
    }
    if fighters[pos.y][pos.x].to_string() == "" {
        return vertical(fighters, direc * -1, pos);
    } 
    (fighters[pos.y][pos.x].to_string(), pos)
}

fn horizontal(fighters: &[&[&str]], direc: i8, position: Position) -> (String, Position){
    let mut pos = position.clone();
    let width : usize = fighters[0].len();
    let dir : usize = 1;
    if direc == 1{        
        match pos.x + dir {
            x if x >= width => pos.x = 0,
            x if x < width => pos.x += dir,
            _ => panic!(), 
        }
    }
    if direc == -1{        
        match pos.x.wrapping_sub(dir) {
            x if x >= width => pos.x = width - 1,
            x if x < width => pos.x = pos.x.wrapping_sub(dir),
            _ => panic!(), 
        }
    }
    if fighters[pos.y][pos.x].to_string() == ""{
        return horizontal(fighters, direc, pos);
    }
    (fighters[pos.y][pos.x].to_string(), pos)
}

#[cfg(test)]
mod tests {
    use super::{super_street_fighter_selection, preloaded::{Direction, Position}};

    #[rustfmt::skip]
    const FIGHTERS_A: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Blanka",   "Guile", ""       ],
        &[ "Balrog",    "Ken",  "Chun Li", "Zangief", "Dhalsim", "Sagat"  ],
        &[   "Vega", "T.Hawk", "Fei Long",  "Deejay",   "Cammy", "M.Bison"],
    ];
    
    #[rustfmt::skip]
    const FIGHTERS_B: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Cammy",  "Blanka",   "Guile",        "", "Chun Li" ],
        &[ "Balrog",    "Ken",  "Chun Li",       "", "M.Bison", "Zangief", "Dhalsim", "Sagat"   ],
        &[   "Vega",       "", "Fei Long", "Balrog",  "Deejay",   "Cammy",        "", "T.Hawk"  ],
    ];
    
    #[rustfmt::skip]
    const FIGHTERS_C: [&[&'static str]; 6] = [
        &[        "",     "Ryu",  "E.Honda",  "Cammy" ],
        &[  "Balrog",     "Ken",  "Chun Li",       "" ],
        &[    "Vega",        "", "Fei Long", "Balrog",],
        &[  "Blanka",   "Guile",         "", "Chun Li"],
        &[ "M.Bison", "Zangief",  "Dhalsim", "Sagat"  ],
        &[  "Deejay",   "Cammy",         "", "T.Hawk" ],
    ];    
    
    #[test]
    fn no_selection() {
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 0), &[] as &[Direction]),
            vec![] as Vec<String>,
            "it should work with no selection cursor moves",
        );
    }
    
    #[test]
    fn empty_vertical_spaces_single_move() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up]),
            vec!["Balrog"],
            "it should stop on empty spaces vertically",
        );
    }
    
    #[test]
    fn empty_vertical_spaces_multiple_moves() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up, Up, Up, Up]),
            vec!["Balrog", "Balrog", "Balrog", "Balrog"],
            "it should stop on empty spaces vertically (up)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Down, Down, Down, Down]),
            vec!["Vega", "Vega", "Vega", "Vega"],
            "it should stop on empty spaces vertically (down)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Up, Up, Up, Up]),
            vec!["Sagat", "Sagat", "Sagat", "Sagat"],
            "it should stop on empty spaces vertically (up)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Down, Down, Down, Down]),
            vec!["M.Bison", "M.Bison", "M.Bison", "M.Bison"],
            "it should stop on empty spaces vertically (down)",
        );        
    }
    
    #[test]
    fn rotate_horizontally() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Ryu", "Guile", "Blanka", "E.Honda", "Ryu", "Guile", "Blanka", "E.Honda"],
            "it should rotate horizontally (left)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(3, 1), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken"],
            "it should rotate horizontally (left)",
        );        
    }
    
    #[test]
    fn rotate_horizontally_with_empty_spaces() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Ryu", "E.Honda"],
            "it should rotate horizontally with empty spaces",
        );        
    }
    
    #[test]
    fn rotate_on_all_rows() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Ryu", "E.Honda", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "M.Bison", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );        
    }
    
    #[test]
    fn should_rotate_on_all_rows() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(3, 0), &[Down, Right, Right, Right, Down, Left, Left, Down, Right, Right, Right, Up]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Chun Li"],
            "it should rotate on all rows",
        );
    }
    
    #[test]
    fn should_work_with_longer_grid() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 0), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Down, Right, Right, Down, Right, Right, Right, Down, Left, Left, Left, Down, Left, Left, Left]),
            vec!["E.Honda", "Ryu", "Ken", "Chun Li", "Balrog", "Ken", "Chun Li", "Fei Long", "Vega", "Balrog", "Fei Long", "Vega", "Blanka", "Guile", "Chun Li", "Sagat", "M.Bison", "Zangief", "Dhalsim", "Dhalsim", "Zangief", "M.Bison", "Sagat", "T.Hawk", "Cammy", "Deejay", "T.Hawk"],
            "it should work with longer grid",
        );        
    }
    
    #[test]
    fn should_work_with_odd_initial_position() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 3), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Up, Right, Right,  Up, Right, Right, Right]),
            vec!["Guile", "Blanka", "M.Bison", "Zangief", "Dhalsim", "Sagat", "M.Bison", "Deejay", "T.Hawk", "Cammy", "Deejay", "T.Hawk", "Sagat", "M.Bison", "Zangief", "Guile", "Chun Li", "Blanka", "Guile"],
            "it should work with odd initial position",
        );        
    }
}