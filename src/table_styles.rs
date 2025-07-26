pub enum TableStyle {
    Thin,
    ThinRounded,
    ThinDouble,
    Thick,
    Basic,
}

impl TableStyle {
    pub fn get_chars(
        &self,
    ) -> (
        char,
        char,
        char,
        char,
        char,
        char,
        char,
        char,
        char,
        char,
        char,
    ) {
        match self {
            Self::Thin => (
                '\u{250C}', // UpperL
                '\u{252C}', // UpperM
                '\u{2510}', // UpperR
                '\u{251C}', // MediumL
                '\u{253C}', // MediumM
                '\u{2524}', // MediumR
                '\u{2514}', // LowerL
                '\u{2534}', // LowerM
                '\u{2518}', // LowerR
                '\u{2500}', // Horizontal
                '\u{2502}', // Vertical
            ),

            Self::Thick => (
                '\u{250F}', // UpperL
                '\u{2533}', // UpperM
                '\u{2513}', // UpperR
                '\u{2523}', // MediumL
                '\u{254B}', // MediumM
                '\u{252B}', // MediumR
                '\u{2517}', // LowerL
                '\u{253B}', // LowerM
                '\u{251B}', // LowerR
                '\u{2501}', // Horizontal
                '\u{2503}', // Vertical
            ),

            Self::ThinDouble => (
                '\u{2554}', // UpperL
                '\u{2566}', // UpperM
                '\u{2557}', // UpperR
                '\u{2560}', // MediumL
                '\u{256C}', // MediumM
                '\u{2563}', // MediumR
                '\u{255A}', // LowerL
                '\u{2569}', // LowerM
                '\u{255D}', // LowerR
                '\u{2550}', // Horizontal
                '\u{2551}', // Vertical
            ),

            Self::ThinRounded => (
                '\u{256D}', // UpperL (Rounded)
                '\u{252C}', // UpperM
                '\u{256E}', // UpperR (Rounded)
                '\u{251C}', // MediumL
                '\u{253C}', // MediumM
                '\u{2524}', // MediumR
                '\u{2570}', // LowerL (Rounded)
                '\u{2534}', // LowerM
                '\u{256F}', // LowerR (Rounded)
                '\u{2500}', // Horizontal
                '\u{2502}', // Vertical
            ),

            Self::Basic => ('+', '+', '+', '+', '+', '+', '+', '+', '+', '-', '|'),
        }
    }
}
