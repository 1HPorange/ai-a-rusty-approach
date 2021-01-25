use ai_lmao::search::{examples::*, SearchResult, *};

fn main() {
    let romania = generate_romania_roadmap();

    println!("From {} to {} via", ARAD, BUCHAREST);

    print!("- Best-first: ");
    match best_first(&romania, &ARAD, |s| s == BUCHAREST, |s| s.cost as isize) {
        Some(SearchResult { cost, path, .. }) => println!("{} {:?}", cost, path),
        None => println!("No path found"),
    }

    print!("- Breadth-first: ");
    match breadth_first(&romania, &ARAD, |s| s == BUCHAREST) {
        Some(SearchResult { cost, path, .. }) => println!("{} {:?}", cost, path),
        None => println!("No path found"),
    }

    print!("- Depth-first: ");
    match depth_first(&romania, &ARAD, None, |s| s == BUCHAREST) {
        Some(SearchResult { cost, path, .. }) => println!("{} {:?}", cost, path),
        None => println!("No path found"),
    }

    print!("- Iterative-deepening: ");
    match iterative_deepening(&romania, &ARAD, None, |s| s == BUCHAREST) {
        Some(SearchResult { cost, path, .. }) => println!("{} {:?}", cost, path),
        None => println!("No path found"),
    }

    print!("- Bidirectional Best-first: ");
    match bidirectional_best_first(&romania, &ARAD, &BUCHAREST, |_| true) {
        Some(SearchResult { cost, path, .. }) => println!("{} {:?}", cost, path),
        None => println!("No path found"),
    }
}
