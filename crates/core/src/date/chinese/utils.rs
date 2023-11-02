// OrderMod模函数，等价于a % b，但当结果为0时，返回b
pub fn order_mod(a: i64, b: i64) -> i64 {
    let result = a % b;
    if result == 0 {
        b
    } else {
        result
    }
}

pub fn dd(jd: f64) -> (i32, i32, i32, i32, i32, i32) {
    // 取得日数的整数部份A及小数部分F
    let mut d = (jd + 0.5).floor();
    let mut f = jd + 0.5 - d;
    let mut c = 0.0;

    if d >= 2299161.0 {
        c = ((d - 1867216.25) / 36524.25).floor();
        d += 1.0 + c - (c / 4.0).floor();
    }

    // 年数
    d += 1524.0;
    let mut _y = ((d - 122.1) / 365.25).floor() as i32;
    // 月数
    d -= (365.25 * (_y as f64)).floor();
    let mut _month = (d / 30.601).floor() as i32; // changed _m to _month
                                                  // 日数
    d -= (30.601 * (_month as f64)).floor(); // changed _m to _month
    let _d = d as i32;

    if _month > 13 {
        // changed _m to _month
        _month -= 13; // changed _m to _month
        _y -= 4715;
    } else {
        _month -= 1; // changed _m to _month
        _y -= 4716;
    }

    // 日的小数转为时分秒
    f *= 24.0;
    let _h = f.floor() as i32;
    f -= _h as f64;

    f *= 60.0;
    let _minute = f.floor() as i32; // changed _m to _minute
    f -= _minute as f64; // changed _m to _minute

    f *= 60.0;
    let _s = f as i32;

    (_y, _month, _d, _h, _minute, _s) // changed _m to _month and _minute
}
