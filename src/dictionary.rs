pub struct Wordle {
    pub word: String,
    pub seed: usize,
}

impl Wordle {
    pub fn new(word: String, seed: usize) -> Self {
        assert_eq!(5, word.len(), "word '{}' must be 5 character long", word);
        Self { word, seed }
    }

    pub fn split(&mut self) -> [char; 5] {
        let mut chars = self.word.chars();
        [
            chars.next().expect("cannot get character at position 0"),
            chars.next().expect("cannot get character at position 1"),
            chars.next().expect("cannot get character at position 2"),
            chars.next().expect("cannot get character at position 3"),
            chars.next().expect("cannot get character at position 4"),
        ]
    }
}

pub fn pick_word(seed: usize) -> Wordle {
    let index = seed % ENGLISH_WORDS.len();
    Wordle::new(ENGLISH_WORDS[index].to_string(), seed)
}

const ENGLISH_WORDS: [&str; 150] = [
    "ABOUT", "ADIEU", "ADMIN", "ADMIT", "ADOPT", "AFTER", "ALBUM", "ALTER", "AMBER", "ANGEL",
    "ANGER", "ANGLE", "APART", "ARGUE", "ARISE", "AUDIO", "AVOID", "BACON", "BADGE", "BASIC",
    "BEACH", "BEARD", "BEAST", "BEGIN", "BEING", "BELOW", "BIBLE", "BLACK", "BOARD", "BORED",
    "BRAIN", "BRAVE", "BREAD", "CAROL", "CHAIR", "CHAOS", "CLEAN", "CLEAR", "CLONE", "CLOUD",
    "COACH", "COUGH", "CREAM", "CUPID", "DAILY", "DAISY", "DANCE", "DEATH", "DEATH", "DOUGH",
    "DRAMA", "DREAM", "DRIVE", "EARLY", "EARTH", "EIGHT", "EQUAL", "FAITH", "FIELD", "FORCE",
    "FORUM", "FRUIT", "GIVEN", "GLOBE", "GLOVE", "GREAT", "GUARD", "HEART", "HONEY", "HUMAN",
    "IRATE", "JUDGE", "KNIFE", "LATER", "LATER", "LAUGH", "LAYER", "LEMON", "LIGHT", "LIVER",
    "LOCAL", "LOVER", "MAGIC", "MAJOR", "MANGO", "METAL", "METER", "MONEY", "MOUSE", "MOUTH",
    "MUSIC", "NINJA", "NOISY", "OFTEN", "ORDER", "ORGAN", "OTHER", "PASTA", "PEACH", "PHONE",
    "PILOT", "PITCH", "PLACE", "PLAIN", "PLANT", "PLATE", "POINT", "POWER", "QURAN", "RANGE",
    "RIVER", "ROYAL", "SCARE", "SCARF", "SHINE", "SHOUT", "SIGHT", "SMILE", "SMOKE", "SOLID",
    "SOUND", "SOUTH", "SPACE", "SPADE", "STONE", "SUGAR", "SUPER", "SUSHI", "TABLE", "TIGER",
    "TOADS", "TODAY", "TOUCH", "TRAIN", "TREND", "TULIP", "UNCLE", "UNDER", "VAGUE", "VEGAN",
    "WATCH", "WATER", "WEARY", "WHALE", "WHITE", "WOMAN", "YOUNG", "YOUTH", "ZEBRA", "ZESTY",
];
