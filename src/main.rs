use clap::Parser;
use spwd::Opts;
use std::process;

fn main() {
    let opts = Opts::parse();
    if opts.license {
        print_license();
        process::exit(0);
    }
    spwd::run(opts)
}

fn print_license() {
    println!(
        "\
License: 
                 Copyright (C) 2023 Emilly M.S. / S.H.

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
"
    );
}

// Emilly S.H. <3
