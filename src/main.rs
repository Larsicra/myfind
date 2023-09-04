use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use colored::Colorize;

fn find<P: AsRef<Path>>(roots: &Vec<P>, regex: &Vec<Regex>, flag: &bool) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    if *flag {
        println!("{}", "所有遍历到的文件".yellow());
        println!("{}", "---------------".yellow());
    }   
    for root in roots {
        // walk_tree(root.as_ref(), regex, &mut matches, flag)?;  
        match walk_tree(root.as_ref(), regex, &mut matches, flag) {
            Ok(()) => {
            },
            Err(er) => println!("{:?} 因为 {} 不是有效目录", er, root.as_ref().to_string_lossy().red()),
        };
    }
    Ok(matches)
}

fn walk_tree(
    dir: &Path,
    regexes: &Vec<Regex>,
    matches: &mut Vec<String>,
    flag: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !dir.is_dir() {
        return Err("not a directory".into());            // only at the beginning
    } else {
        for entry in fs::read_dir(dir) ? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                walk_tree(&path, regexes, matches, flag)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                let mut corr: bool = false;
                for regex in regexes {
                    if regex.is_match(filename) {
                        matches.push(path.to_string_lossy().to_string());
                        corr = true;
                        break;
                    }
                }
                if *flag {
                    if corr {
                        println!("{}", path.to_string_lossy().on_blue());
                    } else {
                        println!("{}", path.to_string_lossy());
                    }
                }
            }
        }
    }
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {                     // not enough
        eprintln!("使用方式：{} <目录> --name <正则表达式> (-v)", args[0]);
        process::exit(1);
    }
    
    let mut dirs: Vec<&String> = Vec::new();
    let mut patterns: Vec<&String> = Vec::new();
    let mut spl: bool = true;
    let mut flag: bool = false;
    for it in &args[1..] {
        if spl {
            if *it == "--name" {
                spl = false;
            } else {
                dirs.push(it);
            }
        } else {
            if *it == "-v" {
                flag = true;
            } else {
                patterns.push(it);
            }
        }
    };

    if dirs.len() == 0 || patterns.len() == 0 {
        eprintln!("使用方式：{} <目录> --name <正则表达式> (-v)", args[0]);
        process::exit(1);
    }

    let mut regexes: Vec<Regex> = Vec::new();

    for pattern in patterns {
        let regex = match Regex::new(pattern) {     // get regex, result{}
            Ok(re) => re,
            Err(err) => {
                eprintln!("正则表达式无效 '{}': {}", pattern, err);    // can't convert
                process::exit(1);
            }
        };
        regexes.push(regex);
    }

    match find(&dirs, &regexes, &flag) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("未找到");
            } else {
                println!("{}", "找到如下匹配".yellow());
                println!("{}", "-----------".yellow());
                for file in matches {
                    println!("{}", file);
                }
            }
        }
        Err(error) => {
            eprintln!("错误: {}", error);
            process::exit(1);
        }
    }
}
