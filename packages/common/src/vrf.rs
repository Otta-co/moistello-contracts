use soroban_sdk::Env;
pub fn random_in_range(env: &Env, max: u32) -> u32 {
    let salt = env.prng().gen::<u64>();
    let sequence = env.ledger().sequence();
    let seed = salt.wrapping_add(sequence as u64);
    if max == 0 { return 0; }
    (seed % (max as u64)) as u32
}
pub fn shuffle_positions(env: &Env, n: u32) -> soroban_sdk::Vec<u32> {
    let mut positions = soroban_sdk::Vec::new(env);
    for i in 0..n { positions.push_back(i); }
    for i in (1..n as usize).rev() {
        let j = random_in_range(env, (i as u32) + 1) as usize;
        let tmp = positions.get(i as u32).unwrap_or(0);
        let j_val = positions.get(j as u32).unwrap_or(0);
        positions.set(i as u32, j_val);
        positions.set(j as u32, tmp);
    }
    positions
}
