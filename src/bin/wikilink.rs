use simple_pagerank::Pagerank;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut pr = Pagerank::<String>::new();

    let stdin = io::stdin();

    let now = Instant::now();

    println!("Reading file and creating link graph");

    for (i, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        let words: Vec<String> = line
            .trim()
            .split('\t')
            .map(|s| s.parse().unwrap())
            .collect();

        if words.len() != 4 {
            continue;
        }

        if i > 0 {
            pr.add_edge(words[1].clone(), words[3].clone());
        }
    }

    println!("Ready in {} secs", now.elapsed().as_secs());

    println!("Graph size: {}", pr.len());

    let mut times = 0;

    loop {
        let t = Instant::now();
        let iter = pr.calculate_step();
        times += 1;
        println!(
            "Iteration {} with convergance {} ({} secs)",
            times,
            iter,
            t.elapsed().as_secs()
        );
        if iter < 0.001 {
            break;
        }
    }

    pr.nodes()
        .iter()
        .map(|node| println!("{} -> {}", node.id(), node.score()))
        .for_each(drop);

    Ok(())
}
