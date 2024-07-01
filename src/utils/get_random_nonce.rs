use rand::Rng;

pub fn get_random_nonce(upper_limit: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let nonce: u32 = rng.gen_range(0..upper_limit);
    nonce
}
