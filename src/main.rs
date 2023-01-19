const SIZE:i32 = 11;
const PRIMES: [i32;23] = [5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97 ];
fn main() {
                        
    let source1: i128 = 999999999;
    let mut rems1 = [0;SIZE as usize];
    for i in 0..SIZE as usize{
        rems1[i] = (source1 % PRIMES[i] as i128) as i32;
    }

    //println!("l1:{}\n{:?}\n", conv_m1(rems1), rems1);
    println!("l1:{}\n {:?}\n", conv_m2(rems1), rems1);
}

fn conv_m1(rems: [i32; SIZE as usize]) -> i128{
    let mut x:i128 = 0;
    loop {
        let mut j = 0;
        for _ in 0..SIZE as usize {
            if x % PRIMES[j] as i128 != rems[j] as i128  {break;}
            j+=1;
        }
        if j == SIZE as usize{
            return x;
        }
        x+=1;
    }
}

#[derive(Debug)]
struct EucRes {
    _d: i128,
    _s: i128,
    _t: i128
}

fn euc(d1:i128, d2:i128) -> EucRes {
    let mut d = vec![d1, d2];
    let mut q = vec![0, (d1 - d1%d2) / d2];
    let mut s = vec![1,0];
    let mut t = vec![0,1];

    let mut i = 2;
    while d[i-1] != 0 {
        d.append(&mut vec![ d[i-2] % d[i-1] ]);
        if d[i] == 0 {break;}
        q.append(&mut vec![ (d[i-1] - (d[i-1]%d[i])) / d[i] ]);
        s.append(&mut vec![ s[i-2] - (q[i-1] * s[i-1]) ]);
        t.append(&mut vec![ t[i-2] - (q[i-1] * t[i-1]) ]);
        i += 1;
    }
    i-=1;
    return EucRes{
        _d: d[i],
        _s: s[i],
        _t: t[i]
    };
}

fn conv_m2(rems: [i32; SIZE as usize]) -> i128 {
    let mut m:i128 = 1;
    for i in 0..SIZE as usize{
        m*= PRIMES[i] as i128;
    }

    let mut res:i128 = 0;
    for i in 0..SIZE as usize {
        let mi = m/PRIMES[i] as i128;
        res += rems[i]  as i128 * euc(mi, PRIMES[i] as i128)._s * mi;
    }
    return res % m
}