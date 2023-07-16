use std::{env, io::{stdout, BufWriter, Write}, fs, sync::Mutex};

use indicatif::{ProgressBar, ProgressStyle};
use rayon::{ThreadPoolBuilder, ThreadPool};
use stylized::{Coloured, Colour};
use regex::Regex;

struct Options {
    progress_bar: Option<ProgressBar>,
    look_inside: Option<Regex>
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 3 {
        error("invalid argument count - usage: rsearch [folder] [search] (arguments)")
    }
    // if args.len() < 3 || args.last().unwrap().starts_with("--") {
    //     args.push(env::current_dir().unwrap().to_str().unwrap().to_string())
    // }
    let mut options = Options {
        progress_bar: None,
        look_inside: {
            let iter = args.iter();
            let mut has_lookin = false;
            let mut lookin = None;
            for i in iter {
                if has_lookin {
                    lookin = Some(Regex::new(i).unwrap_or_else(|_| error("given look-in search is not a valid regex")));
                    break
                }
                if i == "--look-in" {
                    has_lookin = true
                }
            }
            lookin
        },
    };

    let thread_pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    let regex = Regex::new(&args[2]).unwrap_or_else(|_| error("given search is not a valid regex"));

    // println!("{:?}", args[0]);
    let metadata = fs::metadata(&args[1]).unwrap_or_else(|_| error("provided path can't be accessed"));
    if !metadata.is_dir() {
        error("provided path is not a folder")
    }


    println!("{}", format!("Searching for files inside {}", args[1]).colour(Colour::RGB(33, 222, 137)).bold());
    if !args.contains(&"--no-progress-bar".to_string()) {
        let pb = ProgressBar::new(0);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} {elapsed_precise:.green} {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"));
        // pb.update(|x| x.set_len(count as u64));
        options.progress_bar = Some(pb);
    }
    
    let mut files = Vec::with_capacity(16384);
    if options.progress_bar.is_some() {
        thread_pool.join(
            || count(&args[1], &options, &thread_pool),
            || search_folder(&args[1], &regex, &mut files, &options, &thread_pool),
        );
    } else {
        search_folder(&args[1], &regex, &mut files, &options, &thread_pool)
    }

    if let Some(x) = options.progress_bar {
        x.update(|x| x.set_pos(x.len().unwrap()));
        assert!(x.length().unwrap() == x.position());
    }
    if files.is_empty() {
        warn("no match")
    } else {
        std::io::stdout().flush().unwrap();
        let mut buffer = BufWriter::new(stdout().lock());
        let padding = files.len().to_string().len();
        files.iter().enumerate().for_each(|(index, string)| {buffer.write_all(format!(" {}{} - {}\n", " ".repeat(padding - (index+1).to_string().len()), (index+1).to_string().colour(Colour::RGB(137, 33, 222)).bold(), string.to_string().colour(Colour::RGB(222, 137, 33))).as_bytes()).unwrap();})
    }
}

fn count(path: &str, state: &Options, pool: &ThreadPool) {
    let directory_read = match fs::read_dir(path) {
        Ok(v) => v,
        Err(_) => {
            // warn(&format!("{} can't be accessed", path));
            return
        },
    };

    
    let mut counter = 0;
    for path in directory_read {
        counter += 1;
        let directory_entry = match path {
            Ok(v) => v,
            Err(_) => {
                warn("directory can't be read");
                continue
            }
        };

        if directory_entry.file_type().unwrap().is_dir() {
            pool.scope(|_| count(directory_entry.path().to_str().unwrap(), state, pool));
            continue
        }
    }
    
    state.progress_bar.as_ref().unwrap().update(|x| x.set_len(x.len().unwrap() + counter as u64));
}


#[inline(always)]
fn search_folder(path: &str, search: &Regex, files: &mut Vec<String>, state: &Options, pool: &ThreadPool) {
    let directory_read = match fs::read_dir(path) {
        Ok(v) => v,
        Err(_) => {
            if state.progress_bar.is_none() {
                warn(&format!("{} can't be accessed", path));
            }
            return
        },
    };
    for path in directory_read {
        let directory_entry = match path {
            Ok(v) => v,
            Err(_) => {
                // warn("directory can't be read");
                continue
            }
        };

        
        let path = directory_entry.path();
        let name = path.file_name().unwrap();
        if path.is_dir() {
            pool.scope(|_| search_folder(path.to_str().unwrap(), search, files, state, pool));
            continue
        }
        
        if let Some(x) = &state.progress_bar {
            x.inc(1)
        }
        
        if search.is_match(name.to_str().unwrap()) {
            if let Some(regex) = &state.look_inside {
                let file = match fs::read_to_string(&path) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if !regex.is_match(&file) {
                    continue
                }
            }
            files.push(path.to_str().unwrap().to_string());
            continue
        }
        
    }
}

#[inline(always)]
fn error(string: &str) -> ! {
    println!("{} {} {} {}", "[".grey(), "!".red().bold(), "]".grey(), string.colour(Colour::RGB(200, 0, 0)));
    std::process::exit(1)
}

#[inline(always)]
fn warn(string: &str) {
    println!("{} {} {} {}", "[".grey(), "!".colour(Colour::RGB(255, 200, 0)).bold(), "]".grey(), string.colour(Colour::RGB(190, 140, 30)));
}
