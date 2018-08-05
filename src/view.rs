use super::*;
use term_grid::{Grid, GridOptions, Direction, Filling};
use term_size::dimensions;

pub fn view(data: Vec<Entry>, opts: &Opts) {
    let (term_width, _) = dimensions().unwrap_or((80, 24));
    eprintln!("w: {}", term_width);

    if opts.long {
        for el in data {
            println!("{}", el);
        }
    } else {
        let mut grid = Grid::new(GridOptions {
            filling:   Filling::Spaces(5),
            direction: Direction::LeftToRight,
        });
        
        for file_name in data.into_iter().map(|e: Entry| { e.name }) {
            grid.add(file_name.into())
        }
        
        println!("{}", grid.fit_into_width(term_width).unwrap())
    }
}