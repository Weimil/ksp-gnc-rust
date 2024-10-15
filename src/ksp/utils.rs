fn fmt_ut(ut: u64) -> String {
    let yt = 60 * 60 * 6 * 426;
    let dt = 60 * 60 * 6;
    let ht = 60 * 60;
    let mt = 60;

    let y = ut / yt;
    let d = ut - y * yt / dt;
    let h = ut - d * dt / ht;
    let m = ut - h * ht / mt;
    let s = ut - m * mt;

    format!("Y{y} D{d} {h}:{m}:{s}")
}
