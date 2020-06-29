use intspector::min_bits;
use intspector::bin_string;
use intspector::twos_complement;
use intspector::parse_int;

#[test]
fn min_bits_pos_input() {
    assert_eq!(min_bits(0), 1);
    assert_eq!(min_bits(1), 1);
    assert_eq!(min_bits(2), 2);
    assert_eq!(min_bits(3), 2);
    assert_eq!(min_bits(4), 3);
    assert_eq!(min_bits(5), 3);
    assert_eq!(min_bits(254), 8);
    assert_eq!(min_bits(255), 8);
    assert_eq!(min_bits(256), 9);
    assert_eq!(min_bits(257), 9);
}

#[test]
fn min_bits_neg_input() {
    assert_eq!(min_bits(-1), 1);
    assert_eq!(min_bits(-2), 2);
    assert_eq!(min_bits(-3), 3);
    assert_eq!(min_bits(-4), 3);
    assert_eq!(min_bits(-5), 4);
    assert_eq!(min_bits(-127), 8);
    assert_eq!(min_bits(-128), 8);
    assert_eq!(min_bits(-129), 9);
    assert_eq!(min_bits(-130), 9);
}

#[test]
fn twos_complement_3bit() {
    assert_eq!(twos_complement(0, 3), 0);
    assert_eq!(twos_complement(1, 3), 7);
    assert_eq!(twos_complement(2, 3), 6);
    assert_eq!(twos_complement(3, 3), 5);
    assert_eq!(twos_complement(4, 3), 4);
    assert_eq!(twos_complement(5, 3), 3);
    assert_eq!(twos_complement(6, 3), 2);
    assert_eq!(twos_complement(7, 3), 1);
}

#[test]
fn twos_complement_64bit() {
    assert_eq!(twos_complement(0, 64), 0);
    assert_eq!(twos_complement(1, 64), 0xFFFFFFFFFFFFFFFF);
    assert_eq!(twos_complement(2, 64), 0xFFFFFFFFFFFFFFFE);
    assert_eq!(twos_complement(0xFFFFFFFFFFFFFFFF, 64), 1);
    assert_eq!(twos_complement(0xFFFFFFFFFFFFFFFE, 64), 2);
}

#[test]
fn bin_string_0b() {
    assert_eq!(bin_string(0, 0), "");
    assert_eq!(bin_string(1, 0), "");
}

#[test]
fn bin_string_1b() {
    assert_eq!(bin_string(0, 1), "0");
    assert_eq!(bin_string(1, 1), "1");
    assert_eq!(bin_string(2, 1), "0");
    assert_eq!(bin_string(3, 1), "1");
}

#[test]
fn bin_string_8b() {
    assert_eq!(bin_string(  0, 8), "0000_0000");
    assert_eq!(bin_string(  1, 8), "0000_0001");
    assert_eq!(bin_string(  2, 8), "0000_0010");
    assert_eq!(bin_string(  3, 8), "0000_0011");
    assert_eq!(bin_string( 15, 8), "0000_1111");
    assert_eq!(bin_string( 16, 8), "0001_0000");
    assert_eq!(bin_string(255, 8), "1111_1111");
    assert_eq!(bin_string(256, 8), "0000_0000");
}

#[test]
fn bin_string_12b() {
    assert_eq!(bin_string(  0, 12), "0000 0000_0000");
    assert_eq!(bin_string(  1, 12), "0000 0000_0001");
    assert_eq!(bin_string(255, 12), "0000 1111_1111");
    assert_eq!(bin_string(256, 12), "0001 0000_0000");
}

#[test]
fn bin_string_16b() {
    assert_eq!(bin_string(  0, 16), "0000_0000 0000_0000");
    assert_eq!(bin_string(  1, 16), "0000_0000 0000_0001");
    assert_eq!(bin_string(255, 16), "0000_0000 1111_1111");
    assert_eq!(bin_string(256, 16), "0000_0001 0000_0000");
}

#[test]
fn parse_int_no_prefix() {
    assert_eq!(parse_int("0"), Some(0));
    assert_eq!(parse_int("00"), Some(0));
    assert_eq!(parse_int("1"), Some(1));
    assert_eq!(parse_int("01"), Some(1));
    assert_eq!(parse_int("101"), Some(101));
}

#[test]
fn parse_int_binary() {
    assert_eq!(parse_int("b0"), Some(0));
    assert_eq!(parse_int("b1"), Some(1));
    assert_eq!(parse_int("b01"), Some(1));
    assert_eq!(parse_int("b101"), Some(5));
    assert_eq!(parse_int("0b101"), Some(5));
}

#[test]
fn parse_int_octal() {
    assert_eq!(parse_int("o0"), Some(0));
    assert_eq!(parse_int("o1"), Some(1));
    assert_eq!(parse_int("o01"), Some(1));
    assert_eq!(parse_int("o101"), Some(65));
    assert_eq!(parse_int("0o101"), Some(65));
}

#[test]
fn parse_int_decimal() {
    assert_eq!(parse_int("d0"), Some(0));
    assert_eq!(parse_int("d1"), Some(1));
    assert_eq!(parse_int("d01"), Some(1));
    assert_eq!(parse_int("d101"), Some(101));
    assert_eq!(parse_int("0d101"), Some(101));
}

#[test]
fn parse_int_hex() {
    assert_eq!(parse_int("x0"), Some(0));
    assert_eq!(parse_int("x1"), Some(1));
    assert_eq!(parse_int("x01"), Some(1));
    assert_eq!(parse_int("x101"), Some(257));
    assert_eq!(parse_int("0x101"), Some(257));
}

