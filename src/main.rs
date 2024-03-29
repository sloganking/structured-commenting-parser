use ptree::{print_tree, TreeBuilder};
use std::fs::File;
use std::io::prelude::*;

fn count_tabs(line: &str) -> usize {
    let mut c_count = 0;
    for (i, c) in line.chars().enumerate() {
        c_count = i;
        if c != ' ' {
            return (c_count) / 4;
        }
    }
    c_count / 4
}

fn line_is_a_comment(line: &str) -> bool {
    let mut slashes = 0;
    for c in line.chars() {
        if c != ' ' {
            if c == '/' {
                if slashes == 1 {
                    return true;
                } else {
                    slashes += 1;
                }
            } else {
                return false;
            }
        }
    }
    false
}

fn contains_non_space(line: &str) -> bool {
    for c in line.chars() {
        if c != ' ' {
            return true;
        }
    }
    false
}

fn main() {
    // parse lines from file
        // get code string from file
            let mut file = File::open("./input.rs").expect("Unable to open the file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read the file");

        // seperate string into lines
            let lines: Vec<&str> = contents.split('\n').collect();

        // remove all '/r'
            let lines: Vec<String> = lines.into_iter().map(|x| x.replace("\r", "")).collect();

    // assemble tree
        let mut comment_closing_depths: Vec<usize> = Vec::new();
        let mut tree = TreeBuilder::new("code".to_string());

        for line in lines {
            let current_depth = count_tabs(&line);

            // see if comments need closing
                if !comment_closing_depths.is_empty() && contains_non_space(&line){
                    while !comment_closing_depths.is_empty() && current_depth <= comment_closing_depths[comment_closing_depths.len() - 1] {
                        comment_closing_depths.pop();
                        tree.end_child();
                    }
                }
            // see if current line creates new comment
                if line_is_a_comment(&line) {
                    tree.begin_child(line[(count_tabs(&line)*4)+3..].to_string());
                    comment_closing_depths.push(current_depth);
                }
        }


    // turn tree to StringItem (not sure why)
    let tree = tree.build();

    print_tree(&tree).unwrap();
}
