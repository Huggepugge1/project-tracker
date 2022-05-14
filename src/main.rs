use std::fs;
use std::io::{Write, stdin};
use std::path;
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

fn set_up_bar(size: i32, done_color: &str, not_done_color: &str, chars: &str) -> ProgressBar {
    let bar = ProgressBar::new(100);
    let style = format!("[{{bar:{0}.{1}/{2}}}]", size, done_color, not_done_color);
    bar.set_style(ProgressStyle::default_bar().template(&style).progress_chars(chars));
    return bar;
}

fn read_file(file_path: &str) -> String {
    let content: String = fs::read_to_string(file_path)
        .expect(&format!("Failed to read file \"{}\"", file_path))
        .to_string();
    return content;
}

fn create_goal(word_limit: &str, project: &str) {
    let mut file = fs::File::create(format!("projects/{}.txt", project))
        .expect(&format!("Error while creating to file {}", project));
    file.write_all(word_limit.as_ref())
        .expect(&format!("Error while writing to file \"{}\"", project));
}

fn check_goal(file_path: &str, project: &str) -> i32 {
    let file_content: String = read_file(file_path);
    let list: Vec<&str> = file_content.split_whitespace().collect();
    let goal: i32 = fs::read_to_string(format!("projects/{}.txt", project)).unwrap().trim().parse().unwrap();
    if list.len() > 0 {
        return (list.len() as f64 / goal as f64 * 100.0) as i32;
    } else {
        return 0;
    }
}

fn set_up_txt_project(project: &str) -> String {
    if !path::Path::new(&format!("projects/{}.txt", project)).exists() {
        println!("Word limit: ");
        let mut word_limit = String::new();
        stdin().read_line(&mut word_limit).expect("Failed to read input");
        create_goal(&word_limit, project)
    }
    let mut path = String::new();
    println!("Path to project: ");
    stdin().read_line(&mut path).expect("Failed to read input");
    return path.trim().to_string();
}

fn main() {
    println!("Project name: ");
    let mut project: String = String::new();
    stdin().read_line(&mut project).expect("Failed to read input");
    project = project.trim().to_string();
    let path = set_up_txt_project(&project);
    let bar = set_up_bar(100, "green", "blue", "##-");
    bar.set_position(0);
    loop {
        let progress: i32 = check_goal(&path, &project);
        if progress >= 100 {
            break;
        } else if progress > 0 {
            bar.set_position(progress as u64);
        }
        thread::sleep(Duration::from_millis(500));
    }
    bar.finish();
    println!("{} complete!", project);
}
