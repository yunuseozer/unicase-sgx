use unicase::Unicode;

macro_rules! eq {
    ($left:expr, $right:expr) => ({
        assert_eq!(Unicode($left), Unicode($right));
    });
}

//#[test]
pub fn test_ascii_folding() {
    eq!("foo bar", "FoO BAR");
}

//#[test]
pub fn test_simple_case_folding() {
    eq!("στιγμας", "στιγμασ");
}

//#[test]
pub fn test_full_case_folding() {
    eq!("ﬂour", "flour");
    eq!("Maße", "MASSE");
    eq!("ᾲ στο διάολο", "ὰι στο διάολο");
}

