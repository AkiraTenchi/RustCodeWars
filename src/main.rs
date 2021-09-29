use codewars::street_fighter_2_selection::super_street_fighter_selection;
use codewars::street_fighter_2_selection::preloaded::{Direction::Left, Position};

fn main() {
    const FIGHTERS_A: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Blanka",   "Guile", ""       ],
        &[ "Balrog",    "Ken",  "Chun Li", "Zangief", "Dhalsim", "Sagat"  ],
        &[   "Vega", "T.Hawk", "Fei Long",  "Deejay",   "Cammy", "M.Bison"],
    ];
    let res = super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Left, Left, Left, Left, Left, Left, Left, Left]);
    dbg!(res);
}
