pub mod structures;

mod input;
mod eh_url;
mod eh_config;
mod unescape;
mod test_helper;

// result

pub type EhResult<T> = Result<T, ParseError>;

pub trait Parser: Sized {
    fn parse(doc: &str) -> EhResult<Self>;
}

// error

pub type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum ParseError {
    RegexMatchFailed,
    OutOfRange,
    SignInRequired,
    AttributeNotFound,
    DomNotFound,
    FromServer(String),
    Other(BoxDynError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::RegexMatchFailed => write!(f, "regular expression matching failed"),
            ParseError::OutOfRange => write!(f, "input is out of range"),
            ParseError::SignInRequired => write!(f, "this page requires you to log on"),
            ParseError::AttributeNotFound => write!(f, "attribute cannot be found"),
            ParseError::DomNotFound => write!(f, "dom cannot be found"),
            ParseError::FromServer(s) => write!(f, "error from server: {}", s),
            ParseError::Other(e) => e.fmt(f),
        }
    }
}

const REGEX_MATCH_FAILED: ParseError = ParseError::RegexMatchFailed;
const OUT_OF_RANGE: ParseError = ParseError::OutOfRange;
const SIGN_IN_REQUIRED: ParseError = ParseError::SignInRequired;
const ATTRIBUTE_NOT_FOUND: ParseError = ParseError::AttributeNotFound;
const DOM_NOT_FOUND: ParseError = ParseError::DomNotFound;

impl std::error::Error for ParseError {}

impl From<BoxDynError> for ParseError {
    fn from(value: BoxDynError) -> Self {
        ParseError::Other(value)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(value: std::num::ParseIntError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(value: std::num::ParseFloatError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(value: chrono::ParseError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::Other(value.into())
    }
}

// const_concat

pub const unsafe fn transmute<From, To>(from: From) -> To {
    union Transmute<From, To> {
        from: std::mem::ManuallyDrop<From>,
        to: std::mem::ManuallyDrop<To>,
    }

    std::mem::ManuallyDrop::into_inner(Transmute { from: std::mem::ManuallyDrop::new(from) }.to)
}

pub const unsafe fn concat<First, Second, Out>(a: &[u8], b: &[u8]) -> Out
    where
        First: Copy,
        Second: Copy,
        Out: Copy,
{
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Both<A, B>(A, B);

    let arr: Both<First, Second> = Both(
        *transmute::<_, *const First>(a.as_ptr()),
        *transmute::<_, *const Second>(b.as_ptr()),
    );

    transmute(arr)
}

#[macro_export]
macro_rules! const_concat {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        let bytes: &'static [u8] = unsafe {
            &$crate::concat::<
                [u8; $a.len()],
                [u8; $b.len()],
                [u8; $a.len() + $b.len()],
            >($a.as_bytes(), $b.as_bytes())
        };

        unsafe { $crate::transmute::<_, &'static str>(bytes) }
    }};
    ($a:expr, $($rest:expr),*) => {{
        const TAIL: &str = const_concat!($($rest),*);
        const_concat!($a, TAIL)
    }};
    ($a:expr, $($rest:expr),*,) => {
        const_concat!($a, $($rest),*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn top_level_constants() {
        const SALUTATION: &str = "Hello";
        const TARGET: &str = "world";
        const GREETING: &str = const_concat!(SALUTATION, ", ", TARGET, "!");
        const GREETING_TRAILING_COMMA: &str = const_concat!(SALUTATION, ", ", TARGET, "!",);

        assert_eq!(GREETING, "Hello, world!");
        assert_eq!(GREETING_TRAILING_COMMA, "Hello, world!");
    }
}
