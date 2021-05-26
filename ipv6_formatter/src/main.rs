use core::fmt;
use core::fmt::Write as CoreWrite;

use std::net::Ipv6Addr;

#[derive(Debug)]
struct ZeroGroup {
    count: u8,
}

#[derive(Debug)]
enum IPv6Segment {
    NonZeroSegment(u16),
    ZeroSegment(ZeroGroup),
}

#[derive(Default)]
struct IPv6 {
    segments: Vec<IPv6Segment>,
}

impl core::fmt::Debug for IPv6 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
        //let mut segments: Vec<String> = Vec::new();
        let mut w = String::new();
        let mut long_zero_segments = 0;
        let mut zero_streak = false;
        for entry in self.segments.iter() {
            match *entry {
                IPv6Segment::NonZeroSegment(ref num) => {
                    zero_streak = false;
                    //segments.push(format!("{:x}", num).to_string());
                    write!(&mut w, "{:x}", num)?;
                }
                IPv6Segment::ZeroSegment(ref z) => {
                    if z.count == 1 {
                        let _ = write!(&mut w, "0");
                    } else {
                        long_zero_segments += 1;
                        if long_zero_segments == 1 {
                            zero_streak = true;
                            let _ = w.pop();
                            write!(&mut w, ":")?;
                        } else {
                            for _ in 0..z.count {
                                //segments.push("0".to_string());
                                write!(&mut w, "0:")?;
                            }
                            let _ = w.pop();
                        }
                    }
                }
            };
            //segments.push(":".to_string())
            write!(&mut w, ":")?;
        }

        if !(long_zero_segments == 1 && zero_streak) {
            let _ = w.pop();
            //segments.push("".to_string());
            //if allzero_segment {
            //segments.push("".to_string());
            //}
        }

        write!(f, "{:?}", w)
    }
}

fn main() {
    let mut ip6addr: IPv6 = IPv6::default();

    let ipv6_a = "2a03:2880:f12f:183:face:b00c:0:25de";
    //let ipv6_a = "1:0:1:0:1:0:1:0";
    //let ipv6_a = "fe0::1";
    let ipv6 = ipv6_a.parse::<Ipv6Addr>().unwrap().segments();

    let mut in_zero_streak = true;
    let mut zero_streak_length = 0;
    for entry in ipv6.iter() {
        if entry == &0 {
            zero_streak_length += 1;
            in_zero_streak = true;
        } else {
            if in_zero_streak {
                if zero_streak_length >= 1 {
                    ip6addr.segments.push(IPv6Segment::ZeroSegment(ZeroGroup {
                        count: zero_streak_length,
                    }));
                }
                ip6addr.segments.push(IPv6Segment::NonZeroSegment(*entry));
            } else {
                ip6addr.segments.push(IPv6Segment::NonZeroSegment(*entry));
            }
            in_zero_streak = false;
            zero_streak_length = 0;
        }
    }
    if in_zero_streak {
        ip6addr.segments.push(IPv6Segment::ZeroSegment(ZeroGroup {
            count: zero_streak_length,
        }));
    }

    println!("{:?} : {:#?}", ipv6_a, ip6addr);
}
