use rand::random;

const EMOJIS: &[&[u8]] = &[
    include_bytes!("../assets/emoji_pngs/1f004.png"),
    include_bytes!("../assets/emoji_pngs/1f0cf.png"),
    include_bytes!("../assets/emoji_pngs/1f170.png"),
    include_bytes!("../assets/emoji_pngs/1f171.png"),
    include_bytes!("../assets/emoji_pngs/1f17e.png"),
    include_bytes!("../assets/emoji_pngs/1f17f.png"),
];

const EMOJIS_LEN: usize = EMOJIS.len();

pub fn get_emoji() -> &'static [u8] {
    unsafe { *EMOJIS.get_unchecked(random::<usize>() % EMOJIS_LEN) }
}
