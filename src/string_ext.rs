pub trait StringExt {
    fn to_ref(&self) -> &String;
}

impl StringExt for String {
    fn to_ref(&self) -> &String { return self; }
}
