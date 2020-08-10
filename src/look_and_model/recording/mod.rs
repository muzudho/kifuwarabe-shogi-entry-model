pub mod phase;

/// 局面ハッシュを作るときに、フェーズ用に配列があって、それのサイズに使ってるぜ☆（＾～＾）
// pub const PHASE_FIRST: usize = 0;
pub const PHASE_SECOND: usize = 1;
pub const PHASE_LEN: usize = 2;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    First,
    Second,
}
