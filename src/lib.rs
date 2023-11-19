use clap::Parser;
use std::{
    env,
    process::{self, Command},
};

#[derive(Parser, Debug)]
#[command(version, author, about, next_line_help = true)]
pub struct Opts {
    // {{{
    /// How many directories have a full name
    #[arg(short, long, default_value_t = 2)]
    pub full_name_count: usize,

    /// Print the physical path, whithout any symbolic links.
    #[arg(short = 'P', long)]
    pub physical_path: bool,

    /// Print license
    #[arg(short, long)]
    pub license: bool,
}
// }}}

pub fn run(opts: Opts) {
    // {{{
    let mut cwd = if opts.physical_path {
        // Get cwd {{{
        let cwd = env::current_dir().expect("Unable to obtain current directory!");

        cwd.into_os_string()
            .into_string()
            .expect("Current directory is not UTF-8 encoded!")
    } else {
        // 'sh -c pwd' is used because 'env::current_dir()' resolves sym links
        let cwd = Command::new("sh")
            .args(["-c", "pwd"])
            .output()
            .expect("Unable to run 'pwd' with 'sh -c'");

        String::from_utf8_lossy(&cwd.stdout).trim().to_string()
    };
    // }}}

    let home = env::var("HOME").expect("HOME variable is invalid or unset");
    cwd = cwd.replace(&home, "~");

    let dir_count = cwd.split('/').count();
    if dir_count <= opts.full_name_count {
        print!("{}", cwd);
        process::exit(0);
    }

    let num_short_names = dir_count - opts.full_name_count;

    // Shorten the path {{{
    let mut dirs: Vec<String> = cwd.split('/').map(|s| s.to_string()).collect();

    // Start at 1 to avoid '~' or '/'
    for i in 1..num_short_names {
        let first_char = dirs[i]
            .chars()
            .next()
            .expect("There's always a character at the start of a directory name.");

        if first_char == '.' && (dirs[i].len() >= 2) {
            dirs[i].replace_range(2.., "");
        } else {
            dirs[i].replace_range(1.., "");
        }
    }
    cwd = dirs.join("/");
    // }}}

    print!("{}", cwd);
}
// }}}

// Emilly S.H. <3
