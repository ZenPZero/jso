use crate::{Val::*, parse, parse::Error::*};

macro_rules! pass {
  (
    $($in:literal => $out:expr)*
  ) => {
    $(
      assert_eq!(parse($in), Ok($out));
    )*
  };
}

macro_rules! fail {
  (
    $($in:literal => $err:expr)*
  ) => {
    $(
      assert_eq!(parse($in), Err($err));
    )*
  };
}

#[test]
fn empty_input() {
  fail!(
    ""       => UnexpectedEof
    " "      => UnexpectedEof
    "      " => UnexpectedEof
  );
}

#[test]
fn illegal_char() {
  fail!(
    "!"           => UnexpectedChar(0, '!')
    "   !"        => UnexpectedChar(3, '!')
    r#"  {"h"!"#  => UnexpectedChar(6, '!')
    r"[true,a]"   => UnexpectedChar(6, 'a')
  );
}

#[test]
fn null() {
  pass!(
    "null"          => Null
    "     null"     => Null
    "     null    " => Null
    "nullanything can go here and it will not matter" => Null
  );

  fail!(
    "nul"   => UnexpectedEof
    "nul "  => UnexpectedChar(3, ' ')
    "nul l" => UnexpectedChar(3, ' ')
    "nill"  => UnexpectedChar(1, 'i')
  );
}

#[test]
fn bool() {
  pass!(
    "true"          => Bool(true)
    "     true"     => Bool(true)
    "     true    " => Bool(true)
    "trueanything can go here and it will not matter" => Bool(true)

    "false"          => Bool(false)
    "     false"     => Bool(false)
    "     false    " => Bool(false)
    "falseanything can go here and it will not matter" => Bool(false)
  );

  fail!(
    "tru"   => UnexpectedEof
    "tru "  => UnexpectedChar(3, ' ')
    "tru e" => UnexpectedChar(3, ' ')
    "tsue"  => UnexpectedChar(1, 's')

    "fals"   => UnexpectedEof
    "fals "  => UnexpectedChar(4, ' ')
    "fals e" => UnexpectedChar(4, ' ')
    "fblse"  => UnexpectedChar(1, 'b')
  );
}

#[test]
fn num() {
  pass!(
    "     0       " => Num(    0.     )
    "     0.      " => Num(    0.     )
    "     0.0     " => Num(    0.     )
    "     0.000000" => Num(    0.     )
    "000000.000000" => Num(    0.     )
    "    -0       " => Num(    0.     )
    "    -1       " => Num(   -1.     )
    "     1.10    " => Num(    1.1    )
    "   547.75    " => Num(  547.75   )
    "     2.5e10  " => Num(    2.5e10 )
    "     2.5E10  " => Num(    2.5E10 )
    "     0.33    " => Num(    0.33   )
    " -3237.12345 " => Num(-3237.12345)
  );

  fail!(
    "1.2.3" => InvalidNum("1.2.3".parse::<f64>().unwrap_err())
    "   .0" => UnexpectedChar(3, '.')
  );
}

#[test]
fn str() {
  pass!(
    "\"\"" => Str("".into())
    "\" \"" => Str(" ".into())
    "\"abc\"" => Str("abc".into())
    "\"This.    is @ string! \n\"" => Str("This.    is @ string! \n".into())
    "\"hello\"\"anything can go here and it will not matter\"" => Str("hello".into())
  );

  fail!(
    "\"" => UnexpectedEof
  );

  pass!(
    "\" test \\\" \"" => Str(" test \" ".into())
    "\" test \\\\ \"" => Str(" test \\ ".into())
    "\" test \\b \"" => Str(" test \x08 ".into())
    "\" test \\f \"" => Str(" test \x0c ".into())
    "\" test \\n \"" => Str(" test \n ".into())
    "\" test \\r \"" => Str(" test \r ".into())
    "\" test \\t \"" => Str(" test \t ".into())
  );

  // @todo(unicode-escapes)
  fail!(
    "\" \\u1234 \"" => UnexpectedChar(3, 'u')
  );
}

// @todo Arr Obj
