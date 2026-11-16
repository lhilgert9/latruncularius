use std::{sync::Arc, time::Instant};

use crate::{
    board::Board,
    movegen::{MoveGenerator, move_list::MoveList},
};

pub fn run(depth: u8, board: &mut Board, mg: Arc<MoveGenerator>) {
    let mut total_time: u128 = 0;
    let mut total_nodes: u64 = 0;

    println!("Benchmarking perft 1-{depth}:");
    println!("{board}");

    for d in 0..=depth {
        let now = Instant::now();

        let leaf_nodes = perft(board, d, &mg);

        let elapsed = now.elapsed().as_millis();
        let leaves_per_second = ((leaf_nodes * 1000) as f64 / elapsed as f64).floor();

        total_nodes += leaf_nodes;
        total_time += elapsed;

        println!("Perft {d}: {leaf_nodes} ({elapsed} ms, {leaves_per_second} leaves/sec)")
    }

    // Stockfish like perft
    // if depth < 1 {
    //     println!("info Error: depth must be 1 or greater");
    //     return;
    // }
    //
    // let mut move_list = MoveList::new();
    // mg.generate_moves(&board, &mut move_list);

    // for i in 0..move_list.len() {
    //     let m = move_list.get_move(i);
    //     if board.make_move(m, &mg) {
    //         let now = Instant::now();

    //         let nodes = perft(board, depth - 1, &mg);
    //         let elapsed = now.elapsed().as_millis();

    //         board.unmake_last_move();
    //         total_nodes += nodes;
    //         total_time += elapsed;

    //         println!("{}: {}", m, nodes);
    //     }
    // }
    // println!("\nNodes searched: {}", total_nodes);

    let final_lnps = ((total_nodes * 1000) as f64 / total_time as f64).floor();
    println!("Total time spent: {total_time} ms");
    println!("Execution speed: {final_lnps} leaves/second");
}

pub fn perft(board: &mut Board, depth: u8, mg: &MoveGenerator) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut leaf_nodes = 0;

    let mut move_list = MoveList::new();

    mg.generate_moves(board, &mut move_list);

    for i in 0..move_list.len() {
        let m = move_list.get_move(i);
        if board.make_move(m, mg) {
            leaf_nodes += perft(board, depth - 1, mg);
            board.unmake_last_move();
        }
    }

    leaf_nodes
}
