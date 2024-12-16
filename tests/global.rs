use unreal::global::{global_lock, seed_global};

#[test]
fn main() {
    seed_global(0).unwrap();
    check_three_random_names();

    seed_global(0).unwrap();
    check_three_random_names();
}

fn check_three_random_names() {
    assert_eq!(global_lock().unwrap().first_name(), "Mortimer");
    assert_eq!(global_lock().unwrap().first_name(), "Raina");
    assert_eq!(global_lock().unwrap().first_name(), "Alden");
}
