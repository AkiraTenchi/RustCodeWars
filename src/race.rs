pub fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }
    const SEC_PER_HOUR: i32 = 3600;
    const SEC_PER_MINUTE: i32 = 60;
    let delta = v2 - v1;
    let adv_per_sec: f64 = delta as f64 / SEC_PER_HOUR as f64;
    let sec_till_catchup = (((g as f64 / adv_per_sec) * 1000.0).round() / 1000.0) as i32;
    let hours = sec_till_catchup / SEC_PER_HOUR;
    let minutes = (sec_till_catchup - (hours as i32 * SEC_PER_HOUR)) / SEC_PER_MINUTE;
    let seconds =
        sec_till_catchup - (hours as i32 * SEC_PER_HOUR) - (minutes as i32 * SEC_PER_MINUTE);
    Some(vec![hours, minutes, seconds])
}

#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 81, 550), None);
}
