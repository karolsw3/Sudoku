use sudoku_backend::ops::setup::ActivityCache;
use std::thread::sleep;
use chrono::Duration;


#[test]
fn activity_cache() {
    let dur = Duration::milliseconds(2);
    let ac = ActivityCache::new(dur);

    ac.register_activity(1);
    ac.register_activity(3);
    assert_eq!(ac.active_users(), 2);

    sleep(dur.to_std().unwrap() / 2);
    ac.register_activity(2);

    sleep(dur.to_std().unwrap() / 2);
    assert_eq!(ac.active_users(), 1);

    sleep(dur.to_std().unwrap() / 2);
    assert_eq!(ac.active_users(), 0);
}
