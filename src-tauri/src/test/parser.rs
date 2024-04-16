use crate::modules::riscv::basic::interface::parser::{Parser, RISCVParser};

pub fn test_parser() {
    let mut p = RISCVParser::new();
    let rope = ropey::Rope::from_str(
        "
        a:
        addi a1, a2, 0x1\n \
        jal a\n \
        .data\n \
        bb:.align 2\n \
        .byte 0x1\n \
        .text\n \
        beq a1, a2, bb\n \
        ",
    );
    let res = p.parse(rope.to_string());
    println!("{:?}", res);
}
