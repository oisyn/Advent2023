pub struct Parser<'a> {
    buf: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { buf: s.as_bytes() }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn at_end(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn skip(&mut self, num: usize) -> &mut Self {
        self.buf = &self.buf[num.min(self.buf.len())..];
        self
    }

    pub fn peek_char(&self) -> Option<u8> {
        if self.buf.is_empty() {
            None
        } else {
            Some(self.buf[0])
        }
    }

    pub fn take_char(&mut self) -> Option<u8> {
        if self.buf.is_empty() {
            None
        } else {
            let r = Some(self.buf[0]);
            self.buf = &self.buf[1..];
            r
        }
    }

    pub fn peek(&mut self, len: usize) -> &'a str {
        unsafe { std::str::from_utf8_unchecked(&self.buf[..len]) }
    }

    pub fn take(&mut self, len: usize) -> &'a str {
        let str = unsafe { std::str::from_utf8_unchecked(&self.buf[..len]) };
        self.buf = &self.buf[len.min(self.buf.len())..];
        str
    }

    pub fn take_while(&mut self, mut f: impl FnMut(u8) -> bool) -> &'a str {
        let len = 'len: {
            for i in 0..self.buf.len() {
                if !f(self.buf[i]) {
                    break 'len i;
                }
            }
            self.buf.len()
        };
        self.take(len)
    }

    pub fn parse<T: FromParser>(&mut self) -> Option<T> {
        <T as FromParser>::parse_from(self)
    }
}

pub trait FromParser: Sized {
    fn parse_from<'a>(parser: &mut Parser<'a>) -> Option<Self>;
}

macro_rules! impl_int_parser {
    ($t:ty) => {
        impl FromParser for $t {
            fn parse_from<'a>(parser: &mut Parser<'a>) -> Option<Self> {
                parser.take_while(|c| c.is_ascii_digit()).parse().ok()
            }
        }
    };
}

impl_int_parser!(i8);
impl_int_parser!(u8);
impl_int_parser!(i16);
impl_int_parser!(u16);
impl_int_parser!(i32);
impl_int_parser!(u32);
impl_int_parser!(i64);
impl_int_parser!(u64);
impl_int_parser!(isize);
impl_int_parser!(usize);
