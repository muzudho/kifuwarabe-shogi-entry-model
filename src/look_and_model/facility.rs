use crate::cosmic::recording::Movement;
use crate::log::LogExt;
use crate::look_and_model::GameTable;
use casual_logger::Log;

/// 台所はこちらだぜ☆（＾～＾）！指し手の一覧が見れるぜ☆（＾～＾）！
pub struct Kitchen {}
impl Kitchen {
    /// 現在の局面での、指し手の一覧を表示するぜ☆（＾～＾）
    pub fn print_ways(table: &GameTable, ways: &Vec<Movement>) {
        Log::print_notice(&format!("Moves count={}", ways.len()));
        // 辞書順ソート
        let mut move_names = Vec::new();
        for move_ in ways {
            let ss_str = format!(
                "{}{}",
                format!("{}", move_),
                if let Some(captured_move) = move_.captured {
                    let piece_type = table.get_type(
                        if let Some(piece_num) = table.piece_num_at(&captured_move.source) {
                            piece_num
                        } else {
                            panic!(Log::print_fatal("(Err.62) Invalid piece_num."));
                        },
                    );
                    format!(" ({})", piece_type)
                } else {
                    "".to_string()
                }
            );
            move_names.push(ss_str);
        }
        // move_names.sort();
        move_names.sort_by(|y_str, x_str| {
            let y_arr: Vec<_> = y_str.chars().collect();
            let x_arr: Vec<_> = x_str.chars().collect();
            use std::cmp::min;
            let len = min(y_arr.len(), x_arr.len());

            use std::cmp::Ordering;
            for i in 0..len {
                match x_arr[i].cmp(&y_arr[i]) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {}
                }
            }

            // Returns Ordering::Greater, Ordering::Less, Ordering::Equal.
            x_arr.len().cmp(&y_arr.len())
        });
        move_names.reverse();

        for (i, move_name) in move_names.into_iter().enumerate() {
            Log::print_notice(&format!("[{}] {}", i, move_name));
        }
    }
}
