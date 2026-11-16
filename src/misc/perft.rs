use std::{sync::Arc, time::Instant};

use crate::{
    board::Board,
    movegen::{MoveGenerator, move_list::MoveList},
    transposition::defs::{HashData, Transposition},
};

#[derive(Clone, Copy, Default)]
pub struct PerftData {
    depth: u8,
    leaf_nodes: u64,
}

impl HashData for PerftData {
    fn depth(&self) -> u8 {
        self.depth
    }
    fn empty() -> Self {
        Self::default()
    }
}

impl PerftData {
    pub fn new(depth: u8, leaf_nodes: u64) -> Self {
        Self { depth, leaf_nodes }
    }
    pub fn get_leaf_nodes(&self, depth: u8) -> Option<u64> {
        if self.depth == depth {
            Some(self.leaf_nodes)
        } else {
            None
        }
    }
}

pub fn run(depth: u8, board: &mut Board, mg: Arc<MoveGenerator>, tt_size: usize) {
    let mut total_time: u128 = 0;
    let mut total_nodes: u64 = 0;
    let mut hash_full = String::from("");

    let mut transposition = Transposition::<PerftData>::new(tt_size);
    let tt_enabled = transposition.is_enabled();

    println!("Benchmarking perft 1-{depth}:");
    println!("{board}");

    for d in 0..=depth {
        let now = Instant::now();

        let leaf_nodes = perft(board, d, &mg, &mut transposition, tt_enabled);

        let elapsed = now.elapsed().as_millis();
        let leaves_per_second = ((leaf_nodes * 1000) as f64 / elapsed as f64).floor();

        total_nodes += leaf_nodes;
        total_time += elapsed;

        if tt_enabled {
            hash_full = format!(", hash full: {}%", transposition.hash_full_percent());
        }

        println!(
            "Perft {d}: {leaf_nodes} ({elapsed} ms, {leaves_per_second} leaves/sec{hash_full})"
        )
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

pub fn perft(
    board: &mut Board,
    depth: u8,
    mg: &MoveGenerator,
    transposition: &mut Transposition<PerftData>,
    tt_enabled: bool,
) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut leaf_nodes = 0;

    if tt_enabled
        && let Some(data) = transposition.get(board.game_state.zobrist_key)
        && let Some(leaf_nodes) = data.get_leaf_nodes(depth)
    {
        return leaf_nodes;
    }

    let mut move_list = MoveList::new();

    mg.generate_moves(board, &mut move_list);

    for i in 0..move_list.len() {
        let m = move_list.get_move(i);
        if board.make_move(m, mg) {
            leaf_nodes += perft(board, depth - 1, mg, transposition, tt_enabled);
            board.unmake_last_move();
        }
    }

    if tt_enabled {
        transposition.insert(
            board.game_state.zobrist_key,
            PerftData::new(depth, leaf_nodes),
        );
    }

    leaf_nodes
}
