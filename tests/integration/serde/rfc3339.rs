use serde::{Deserialize, Serialize};
use serde_test::{
    assert_de_tokens_error, assert_ser_tokens_error, assert_tokens, Configure, Token,
};
use time::macros::datetime;
use time::serde::rfc3339;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Test {
    #[serde(with = "rfc3339")]
    dt: OffsetDateTime,
    #[serde(with = "rfc3339::option")]
    option_dt: Option<OffsetDateTime>,
}

#[test]
fn serialize() {
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: Some(datetime!(2000-01-01 00:00:00 UTC)),
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("2000-01-01T00:00:00Z"),
            Token::Str("option_dt"),
            Token::Some,
            Token::BorrowedStr("2000-01-01T00:00:00Z"),
            Token::StructEnd,
        ],
    );
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 UTC),
        option_dt: None,
    };
    assert_tokens(
        &value.compact(),
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("2000-01-01T00:00:00Z"),
            Token::Str("option_dt"),
            Token::None,
            Token::StructEnd,
        ],
    );
    assert_de_tokens_error::<Test>(
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
            Token::BorrowedStr("bad"),
            Token::StructEnd,
        ],
        "the 'year' component could not be parsed",
    );
    let value = Test {
        dt: datetime!(2000-01-01 00:00:00 +00:00:01),
        option_dt: None,
    };
    assert_ser_tokens_error::<Test>(
        &value,
        &[
            Token::Struct {
                name: "Test",
                len: 2,
            },
            Token::Str("dt"),
        ],
        "The offset_second component cannot be formatted into the requested format.",
    );
}
