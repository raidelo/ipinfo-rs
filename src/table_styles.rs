pub struct TableChars {
    pub upper_left: char,
    pub upper_medium: char,
    pub upper_right: char,
    pub medium_left: char,
    pub medium_medium: char,
    pub medium_right: char,
    pub lower_left: char,
    pub lower_medium: char,
    pub lower_right: char,
    pub horizontal: char,
    pub vertical: char,
}

impl TableChars {
    pub fn new(style: TableStyle) -> TableChars {
        style.get_chars()
    }
}

pub enum TableStyle {
    Thin,
    ThinRounded,
    ThinDouble,
    Thick,
    Basic,
}

impl TableStyle {
    pub fn get_chars(&self) -> TableChars {
        match self {
            Self::Thin => TableChars {
                upper_left: '\u{250C}',
                upper_medium: '\u{252C}',
                upper_right: '\u{2510}',
                medium_left: '\u{251C}',
                medium_medium: '\u{253C}',
                medium_right: '\u{2524}',
                lower_left: '\u{2514}',
                lower_medium: '\u{2534}',
                lower_right: '\u{2518}',
                horizontal: '\u{2500}',
                vertical: '\u{2502}',
            },

            Self::ThinRounded => TableChars {
                upper_left: '\u{256D}',
                upper_medium: '\u{252C}',
                upper_right: '\u{256E}',
                medium_left: '\u{251C}',
                medium_medium: '\u{253C}',
                medium_right: '\u{2524}',
                lower_left: '\u{2570}',
                lower_medium: '\u{2534}',
                lower_right: '\u{256F}',
                horizontal: '\u{2500}',
                vertical: '\u{2502}',
            },

            Self::ThinDouble => TableChars {
                upper_left: '\u{2554}',
                upper_medium: '\u{2566}',
                upper_right: '\u{2557}',
                medium_left: '\u{2560}',
                medium_medium: '\u{256C}',
                medium_right: '\u{2563}',
                lower_left: '\u{255A}',
                lower_medium: '\u{2569}',
                lower_right: '\u{255D}',
                horizontal: '\u{2550}',
                vertical: '\u{2551}',
            },

            Self::Thick => TableChars {
                upper_left: '\u{250F}',
                upper_medium: '\u{2533}',
                upper_right: '\u{2513}',
                medium_left: '\u{2523}',
                medium_medium: '\u{254B}',
                medium_right: '\u{252B}',
                lower_left: '\u{2517}',
                lower_medium: '\u{253B}',
                lower_right: '\u{251B}',
                horizontal: '\u{2501}',
                vertical: '\u{2503}',
            },
            Self::Basic => TableChars {
                upper_left: '+',
                upper_medium: '+',
                upper_right: '+',
                medium_left: '+',
                medium_medium: '+',
                medium_right: '+',
                lower_left: '+',
                lower_medium: '+',
                lower_right: '+',
                horizontal: '-',
                vertical: '|',
            },
        }
    }
}
