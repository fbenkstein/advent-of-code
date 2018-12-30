use std::collections::HashSet;

// Manual de-assembling
//
// 06: f = c | 0x10000
//     c = 16123384
// 08: d = f & 0xff
//     c = d + c
//     c = c & 0xffffff
//     c = c * 65899
//     c = c & 0xffffff
//     if f < 0x100
//       goto 28
//     else
//       goto 17
// 17: d = 0
//     b = d + 1
//     b = b * 0x100
//     if f < b
//       goto 24
//     else
//       goto 26
// 24: d = d + 1
//     goto 18
// 26: f = d
//     goto 08
// 28: if c == a
//       end
//     else
//       goto 06

fn deassembly() -> (Option<usize>, Option<usize>) {
    let mut min_halt_value = None;
    let mut last_halt_value = None;
    let mut halt_values = HashSet::new();

    let mut c: u64 = 0;
    let mut f;
    loop {
        f = c | 0x10000; // 06
        c = 16123384;
        loop {
            let d = f & 0xff; // 08
            c = d + c;
            c = c & 0xffffff;
            c = c * 65899;
            c = c & 0xffffff;
            if f < 0x100 {
                if min_halt_value.is_none() {
                    min_halt_value = Some(c as usize);
                }
                // It looks like halt values are coming peridoically,
                // so we just compute the last value in the period here.
                if halt_values.insert(c) {
                    last_halt_value = Some(c as usize);
                } else {
                    return (min_halt_value, last_halt_value);
                }
                break;
            } else {
                for d in 0.. {
                    if (d + 1) * 0x100 > f {
                        f = d;
                        break; // goto 08
                    }
                }
            }
        }
    }
}

pub fn solve(_: &str) -> (Option<usize>, Option<usize>) {
    deassembly()
}
