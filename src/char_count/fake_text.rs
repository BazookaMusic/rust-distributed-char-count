use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_fake_text(char_count: usize) -> String {
    let rng = thread_rng();

    let fake_text: String = rng
        .sample_iter(&Alphanumeric)
        .take(char_count)
        .map(char::from)
        .collect();

    fake_text
}