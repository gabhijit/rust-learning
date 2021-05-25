use std::convert::TryInto as _;
use std::fmt;
use std::fmt::Write as StdWrite;

#[derive(Debug)]
struct ZeroGroup {
    start: u8,
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
    zero_segments: u8,
}

impl std::fmt::Debug for IPv6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut w = String::new();
        let mut zero_segment_count = 0;
        let mut zero_segment = false;
        for entry in self.segments.iter() {
            match *entry {
                IPv6Segment::NonZeroSegment(ref num) => {
                    zero_segment = false;
                    write!(&mut w, "{:x}", num)
                }
                IPv6Segment::ZeroSegment(ref z) => {
                    if zero_segment_count == 0 {
                        write!(&mut w, ":")?;
                    } else {
                        for _ in 0..z.count {
                            write!(&mut w, "0")?;
                        }
                    }
                    zero_segment_count += 1;
                    zero_segment = true;
                    write!(&mut w, "")
                }
            };
            write!(&mut w, ":x");
        }

        if !(zero_segment && zero_segment_count == 1) {
            let _ = w.pop();
        }

        write!(f, "{:?}", w)
    }
}

fn main() {
    let mut ip6addr: IPv6 = IPv6::default();

    // 2001:1234:0000:0000:1b12:0000:0000:1a13
    let ipv6: [u16; 8] = [
        0x2001u16, 0x1234u16, 0000, 0000, 0x1b12, 0000, 0000, 0x1a13u16,
    ];
    // let ipv6: [u16; 8] = [0x2u16, 0x3u16, 0, 0, 0x1bu16, 0, 0, 0x1u16];
    let ipv6: [u16; 8] = [1, 0, 0, 0, 0, 0, 0, 1];

    let mut in_zero_streak = true;
    let mut zero_streak_start = 0;
    let mut zero_streak_length = 0;
    let mut zero_group_idx: u8 = 0;
    let mut last_zgroup: Option<&ZeroGroup> = None;
    for (idx, entry) in ipv6.iter().enumerate() {
        if entry == &0 {
            if !in_zero_streak {
                zero_streak_start = idx.try_into().unwrap();
            }
            zero_streak_length += 1;
            in_zero_streak = true;
        } else {
            if in_zero_streak {
                if zero_streak_length > 1 {
                    ip6addr.segments.push(IPv6Segment::ZeroSegment(ZeroGroup {
                        start: zero_streak_start,
                        count: zero_streak_length,
                    }));
                    ip6addr.zero_segments += 1;
                }
                ip6addr.segments.push(IPv6Segment::NonZeroSegment(*entry));
            }
            in_zero_streak = false;
            zero_streak_length = 0;
        }
    }
    if in_zero_streak {
        ip6addr.segments.push(IPv6Segment::ZeroSegment(ZeroGroup {
            start: zero_streak_start,
            count: zero_streak_length,
        }));
        ip6addr.zero_segments += 1;
    }

    println!("{:#?}", ip6addr);
}
