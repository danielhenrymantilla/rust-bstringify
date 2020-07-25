use ::bstringify::bstringify;

#[test]
fn basic ()
{
    assert_eq!(
        bstringify!(Hello, World),
        stringify!(Hello, World).as_bytes(),
    )
}

#[test]
fn within_a_match ()
{
    assert!(matches!(&[][..], bstringify!()));
}
