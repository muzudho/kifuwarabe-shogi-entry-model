use crate::cosmic::smart::square::{AbsoluteAddress2D, FILE1U8};
use crate::law::cryptographic::*;
use crate::log::LogExt;
use crate::protocol::uxi::TeigiConv;
use casual_logger::Log;

impl TeigiConv {
    pub fn teigi_conv() {
        Log::print_notice("teigi::convのテスト");

        for ms in 1..9 {
            for hash in 0..10 {
                let sq = AbsoluteAddress2D::new(FILE1U8, ms);
                let next = push_sq_to_hash(hash, Some(&sq));
                let (hash_orig, square_orig) = pop_sq_from_hash(next);
                Log::print_notice( &format!("push_ms_to_hash(0b{:4b},0b{:5b})=0b{:11b} pop_sq_from_hash(...)=(0b{:4b},0b{:5b})"
                    ,hash
                    ,ms
                    ,next
                    ,hash_orig
                    ,if let Some(square_orig_val) = square_orig{ square_orig_val.serial_number()}else{0}
                ));
            }
        }
    }
}
