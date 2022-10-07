use std::io::Write;

fn usage() -> ! {
    eprintln!("fromhex: usage: fromhex <hex-digits>");
    std::panic::resume_unwind(Box::new(()));
}

fn main() {
    let hex = std::env::args().nth(1).unwrap_or_else(|| usage());
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        usage();
    }
    let nhex = hex.len();
    let mut hexchars = Vec::with_capacity(nhex + 1);
    if nhex % 2 == 1 {
        hexchars.push('0');
    }
    hexchars.extend(hex.chars());
    let mut out = Vec::with_capacity((nhex + 1) / 2);
    for pair in hexchars.chunks(2) {
        let p1 = char::to_digit(pair[0], 16).unwrap();
        let p2 = char::to_digit(pair[1], 16).unwrap();
        out.push((p1 * 16 + p2) as u8);
    }
    std::io::stdout().write_all(&out).unwrap_or_else(|e| {
        eprintln!("fromhex: write failed: {e}");
        std::panic::resume_unwind(Box::new(()));
    });
}
