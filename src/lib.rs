pub fn nothing(upper: u64) -> u64 {
    42
}

pub fn fnaive1(upper: u64) -> u64 {
    let mut count = 0;

    for i in 1..upper {
        if i % 3 == 0 {
            count += 1;
        }
        if i % 5 == 0 {
            count += 1;
        }
    }

    count
}

pub fn fopt1(upper: u64) -> u64 {
    let mut factor3 = 0;
    let mut factor5 = 0;

    let mut count = 0;

    for i in 1..upper {
        if i - 3 == factor3 {
            factor3 = i;
            count += 1;
        }
        if i - 5 == factor5 {
            factor5 = i;
            count += 1;
        }
    }

    count
}

pub fn fopt2(upper: u64) -> u64 {
    let mut factor3 = 0;
    let mut factor5 = 0;

    let mut count = 0;

    for i in 1..upper {
        if i - 3 == factor3 {
            factor3 += 3;
            count += 1;
        }
        if i - 5 == factor5 {
            factor5 += 5;
            count += 1;
        }
    }

    count
}

pub fn fopt3(upper: u64) -> u64 {
    let upper = upper as i64;

    let mut factor3 = 0;
    let mut factor5 = 0;

    let mut count = 0;

    for i in 1..upper {
        let first = (i - 3 == factor3) as i64;
        factor3 += first * 3;
        count += first;
        let second = (i - 5 == factor5) as i64;
        factor5 += second * 5;
        count += second;
    }

    count as u64
}

pub fn fzbz2(up: u64)  -> u64 {
    let mut cnt = 0;
    for i in 1..up {
        cnt += (i % 3 == 0) as u64 + (i % 5 == 0) as u64;
    } 
    cnt
} 

pub fn fzbz1(up: u64) -> u64 {
    (1..up)
        .map(|i| {
            let mut cnt = 0;
            if i % 3 == 0 {
                cnt += 1;
            }
            if i % 5 == 0 {
                cnt += 1;
            }
            cnt
        })
        .sum()
}

pub fn fbf(u: u64) -> u64 {
    u/3 + u/5 - (u % 3 == 0) as u64 - (u % 5 == 0) as u64
}

pub fn test_cast(up: i64) -> u64 {
    up as u64
}
