use std::io::Read;

pub struct PacketBuffer {
    pub buf: [u8; 512],
    pub pos: usize,
}

impl PacketBuffer {
    fn new() -> Self {
        Self {
            buf: [0; 512],
            pos: 0,
        }
    }
}

pub struct DnsPacket {
    header: DnsHeader,
    question: Vec<DnsQuestion>,
    answer: Vec<DnsRecord>,
    authority: Vec<DnsRecord>,
    addtional: Vec<DnsRecord>,
}

pub struct DnsHeader;
pub struct DnsQuestion;
pub struct DnsRecord;

fn main() -> std::io::Result<()> {
    let mut pb = PacketBuffer::new();
    let mut f = std::fs::File::open("data/response_packet_google.com.txt")?;
    // let mut f = std::fs::File::open("README.md")?;
    if let Ok(size) = f.read(&mut pb.buf)
        && size <= 512
    {
        println!("Success! Read {size} byte.");
        println!("-----");
        println!(
            "{}",
            std::str::from_utf8(&pb.buf).expect("failed to convert utf8")
        );
        println!("-----");
    } else {
        println!("failed... maybe bigger file size rather than 512byte.");
    }

    Ok(())
}
