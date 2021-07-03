use ptree::{print_tree, TreeBuilder};
use std::collections::HashMap;
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
    return c_count / 4;
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

        // for line in lines.iter() {
        //     println!("\n{}", line);
        //     println!("{}", count_tabs(line));
        //     println!("line_is_a_comment: {}", line_is_a_comment(line));
        // }

        // println!("{:?}", lines);

    // assemble tree
        let mut comment_closing_depths: Vec<usize> = Vec::new();
        let mut current_depth = 0;

        let mut tree = TreeBuilder::new("code".to_string());

        let mut begin_child_count = 0;
        let mut end_child_count = 0;

        for line in lines {
            current_depth = count_tabs(line);

            if line_is_a_comment(line) {
                tree.begin_child(line[count_tabs(line)..].to_string());
                begin_child_count += 1;
                comment_closing_depths.push(current_depth);
            }

            if comment_closing_depths.len() > 0 {
                println!("comment_closing_depths.len() > 0");
                println!("contains_non_space(line): {}", contains_non_space(line));
                if contains_non_space(line) {
                    while current_depth <= comment_closing_depths[comment_closing_depths.len() - 1] {
                        println!("loop!");
                        comment_closing_depths.pop();
                        tree.end_child();
                        end_child_count += 1;
                    }
                }
            }
        }

        println!("begin_child_count: {}", begin_child_count);
        println!("end_child_count: {}", end_child_count);

    // print_tree(&tree).unwrap();

    // example code
        // // Build a tree using a TreeBuilder
        // let tree = TreeBuilder::new("tree".to_string())
        //     .begin_child("branch".to_string())
        //         // .add_empty_child("leaf".to_string())
        //         // .add_empty_child("leaf2".to_string())
        //         .begin_child("branch2".to_string())

        //             .add_empty_child("empty branch".to_string())

        //             .begin_child("branch2".to_string())
        //                 .add_empty_child("empty branch".to_string())
        //             .end_child()

        //             .begin_child("branch2".to_string())
        //             .end_child()

        //             .begin_child("branch2".to_string())
        //             .end_child()

        //         .end_child()
        //     .end_child()
        //     .add_empty_child("empty branch".to_string())
        //     .build();

        // // Print out the tree using default formatting
        // print_tree(&tree).unwrap();
}
