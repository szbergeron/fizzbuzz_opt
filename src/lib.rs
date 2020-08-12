pub fn fnaive1(upper: i64) -> i64 {
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

pub fn fopt1(upper: i64) -> i64 {
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

pub fn fopt2(upper: i64) -> i64 {
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
