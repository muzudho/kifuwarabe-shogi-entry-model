use crate::log::LogExt;
use crate::protocol::uxi::Rand;
use casual_logger::Log;
use rand::Rng;

impl Rand {
    pub fn rand() {
        // 乱数の試し
        let secret_number = rand::thread_rng().gen_range(1, 101); //1~100
        Log::print_notice(&format!("乱数={}", secret_number));
    }
}
