use super::*;
use entry::*;
use term_grid::{Grid, GridOptions, Direction, Filling, Cell};
use term_size::dimensions;
use ansi_term::Style;
use chrono::{DateTime, Local};

use self::StyleType::*;

#[allow(dead_code)]
enum StyleType {
    Header(String),
    Perm(Permissions),
    Size(u64),
    User(String),
    Group(String),
    Time(DateTime<Local>),
    Name(String, FileType),
    Links(u64),
    Inode,
    Blocks,
}

pub fn view(data: Vec<Entry>, opts: &Opts) {
    let (term_width, _) = dimensions().unwrap_or((80, 24));

    if opts.display_ops.long {
        // === Long mode === //
        let num_cols = opts.display_ops.long_options.active_count();

        // Create grid
        let mut grid = Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        });

        // Header
        grid.add( style( Header("Permissions"  .into() )));
        grid.add( style( Header("Size"         .into() )));
        grid.add( style( Header("User"         .into() )));
        grid.add( style( Header("Date_Modified".into() )));
        grid.add( style( Header("Name"         .into() )));

        for e in data {
            let e: Entry = e;
            let user_name = e.user.name().into();

            grid.add( style( Perm( e.permissions       )));
            grid.add( style( Size( e.size              )));
            grid.add( style( User( user_name           )));
            grid.add( style( Time( e.modified          )));
            grid.add( style( Name( e.name, e.file_type )));
        }
        print!("{}", grid.fit_into_columns(num_cols))
    } else {
        // === Norml mode === //
        let mut grid = Grid::new(GridOptions {
            filling:   Filling::Spaces(5),
            direction: Direction::LeftToRight,
        });

        for e in data {
            let e: Entry = e;
            grid.add(style(StyleType::Name (e.name, e.file_type )));
        }

        print!("{}", grid.fit_into_width(term_width).unwrap())
    }
}

fn style(st: StyleType) -> Cell {
    use ansi_term::Colour::*;

    let style: Style = match &st {
        Header(_)         => Style::default().underline(),
        Time  (_)         => Blue.normal(),
        Name(_, filetype) => filetype_style(filetype),
        _                 => Style::default()
    };

    let s: String = match st {
        Header(text   ) => text,
        Perm  (perm   ) => format!("{}", perm),
        Size  (size   ) => format!("{}", size),
        User  (user   ) => user,
        Group (group  ) => group,
        Time  (time   ) => format!("{}", time.format("%_d %b %H:%M")),
        Name  (name, _) => name,
        Links (links  ) => format!("{}", links),
        Inode           => unimplemented!(),
        Blocks          => unimplemented!(),
    };
    // Cell's standard implementation counts color/style codes as
    // extra character and therefore it't with-management got messed up

    let width = s.len();
    Cell {
        contents: style.paint(s).to_string(),
        width: width,
    }
}

fn filetype_style(file_type: &FileType) -> Style {
    use ansi_term::Colour::*;

    match file_type {
        FileType::File        => Style::default(),
        FileType::Dir         => Blue.normal() ,
        FileType::Symlink     => Cyan.normal() ,
        FileType::BlockDevice => Yellow.bold() ,
        FileType::CharDevice  => Yellow.bold() ,
        FileType::Pipe        => Yellow.normal() ,
        FileType::Socket      => Red.bold() ,
        FileType::Unknown     => Style::default() ,
    }
}