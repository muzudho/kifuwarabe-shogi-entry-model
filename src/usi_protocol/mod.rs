pub mod go;
pub mod input_usi;
pub mod is_ready;
pub mod output_xfen;
pub mod position;
pub mod set_option;
pub mod usi;
pub mod usi_new_game;

pub struct Go {
    pub btime: u64,
    pub wtime: u64,
    pub binc: u64,
    pub winc: u64,
}

pub struct IsReady {}

pub struct Position {}

pub struct SetOption {}

pub struct Usi {}

pub struct UsiNewGame {}
