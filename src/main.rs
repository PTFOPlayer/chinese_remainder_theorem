use std::{io, process::exit};
fn main() {
    println!("Provide operation mode");
    println!("1: convert value into array of reminders");
    println!("2: convert array of reminders into value");
    println!("3: add two arrays of reminders and convert them into value");
    let mut profile= String::new();
    io::stdin().read_line(&mut profile).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let res:Result<i32, _> = profile[..(profile.len() -1)].parse();
    match res {
        Ok(res) => {
            match res {
                1 => {disassemble();}
                2 => {assemble();}
                3 => {add();}
                i32::MIN..=0 |3.. => {

                }
            }
        }
        Err(_) => {
            println!("wrong value provided");
            exit(0);
        }   
    }
}

fn disassemble() {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    println!("give value to convert");
    io::stdin().read_line(&mut buf1).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let source1: i128 = buf1[..(buf1.len() -1)].parse().unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    println!("give array of reminders sepparated by white spaces");
    io::stdin().read_line(&mut buf2).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let mut rems1: Vec<i32> = vec![];
    for element in  buf2.split_whitespace().collect::<Vec<&str>>() {
        let div:i32 = element.parse().unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
        rems1.append(&mut vec![(source1 % div as i128) as i32]);   
    }
    println!("{:?}\n", rems1);
}

fn assemble() {
    let mut buf1 = String::new();
    println!("give array of reminders sepparated by white spaces");
    io::stdin().read_line(&mut buf1).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let mut rems: Vec<i32> = vec![];
    for element in  buf1.split_whitespace().collect::<Vec<&str>>() {
        let rem:i32 = element.parse().unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
        rems.append(&mut vec![rem]); 
    }

    let mut buf2 = String::new();
    println!("give array of dividers sepparated by white spaces");
    io::stdin().read_line(&mut buf2).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let mut divs: Vec<i32> = vec![];
    for element in  buf2.split_whitespace().collect::<Vec<&str>>() {
        let div:i32 = element.parse().unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
        divs.append(&mut vec![div]); 
    }

    if rems.len() == divs.len() {
        println!("{}",conv_m2(rems, divs));
    } else {
        println!("dividers array is not the same length as array of reminders")
    }

}

fn add() {
    let mut buf1 = String::new();
    println!("give first array of reminders sepparated by white spaces");
    io::stdin().read_line(&mut buf1).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let mut rems1: Vec<i32> = vec![];
    for element in  buf1.split_whitespace().collect::<Vec<&str>>() {
        let rem:i32 = element.parse().unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
        rems1.append(&mut vec![rem]); 
    }
    let mut buf2 = String::new();
    println!("give second array of reminders sepparated by white spaces");
    io::stdin().read_line(&mut buf2).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let mut rems2: Vec<i32> = vec![];
    for element in  buf2.split_whitespace().collect::<Vec<&str>>() {
        let rem:i32 = element.parse().unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
        rems2.append(&mut vec![rem]); 
    }

    let mut buf3 = String::new();
    println!("give array of dividers sepparated by white spaces");
    io::stdin().read_line(&mut buf3).unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
    let mut divs: Vec<i32> = vec![];
    for element in  buf3.split_whitespace().collect::<Vec<&str>>() {
        let div:i32 = element.parse().unwrap_or_else(|_| {println!("wrogn value provided"); exit(0);});
        divs.append(&mut vec![div]); 
    }

    if rems1.len() == rems2.len() && rems1.len() == divs.len() {
        let mut rems: Vec<i32> = vec![];

        for i in  0..rems1.len() as usize {
            rems.append(&mut vec![rems1[i] + rems2[i]]);
        }
        println!("{}",conv_m2(rems, divs));
    } else {
        println!("dividers array is not the same length as array of reminders")
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

fn conv_m2(rems: Vec<i32>, divs: Vec<i32>) -> i128 {
    let mut m:i128 = 1;
    for i in 0..divs.len() as usize{
        m*= divs[i] as i128;
    }

    let mut res:i128 = 0;
    for i in 0..divs.len() as usize {
        let mi = m/divs[i] as i128;
        if euc(mi, divs[i] as i128)._s > 0 {
            res += rems[i] as i128 * euc(mi, divs[i] as i128)._s * mi;
        } else {
            res += rems[i] as i128 * (euc(mi, divs[i] as i128)._s + divs[i] as i128) * mi;
        }
    }
    return res % m
}
