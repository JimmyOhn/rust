//@no-rustfix

#![warn(clippy::unconditional_recursion)]
#![allow(clippy::partialeq_ne_impl, clippy::default_constructed_unit_structs)]

enum Foo {
    A,
    B,
}

impl PartialEq for Foo {
    fn ne(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        self != other
    }
    fn eq(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        self == other
    }
}

enum Foo2 {
    A,
    B,
}

impl PartialEq for Foo2 {
    fn ne(&self, other: &Self) -> bool {
        self != &Foo2::B // no error here
    }
    fn eq(&self, other: &Self) -> bool {
        self == &Foo2::B // no error here
    }
}

enum Foo3 {
    A,
    B,
}

impl PartialEq for Foo3 {
    fn ne(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        self.ne(other)
    }
    fn eq(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        self.eq(other)
    }
}

enum Foo4 {
    A,
    B,
}

impl PartialEq for Foo4 {
    fn ne(&self, other: &Self) -> bool {
        self.eq(other) // no error
    }
    fn eq(&self, other: &Self) -> bool {
        self.ne(other) // no error
    }
}

enum Foo5 {
    A,
    B,
}

impl Foo5 {
    fn a(&self) -> bool {
        true
    }
}

impl PartialEq for Foo5 {
    fn ne(&self, other: &Self) -> bool {
        self.a() // no error
    }
    fn eq(&self, other: &Self) -> bool {
        self.a() // no error
    }
}

struct S;

// Check the order doesn't matter.
impl PartialEq for S {
    fn ne(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        other != self
    }
    fn eq(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        other == self
    }
}

struct S2;

// Check that if the same element is compared, it's also triggering the lint.
impl PartialEq for S2 {
    fn ne(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        other != other
    }
    fn eq(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        other == other
    }
}

struct S3;

impl PartialEq for S3 {
    fn ne(&self, _other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        self != self
    }
    fn eq(&self, _other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        self == self
    }
}

// There should be no warning here!
#[derive(PartialEq)]
enum E {
    A,
    B,
}

#[derive(PartialEq)]
struct Bar<T: PartialEq>(T);

struct S4;

impl PartialEq for S4 {
    fn eq(&self, other: &Self) -> bool {
        // No warning here.
        Bar(self) == Bar(other)
    }
}

macro_rules! impl_partial_eq {
    ($ty:ident) => {
        impl PartialEq for $ty {
            fn eq(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

struct S5;

impl_partial_eq!(S5);
//~^ ERROR: function cannot return without recursing

struct S6 {
    field: String,
}

impl PartialEq for S6 {
    fn eq(&self, other: &Self) -> bool {
        let mine = &self.field;
        let theirs = &other.field;
        mine == theirs // Should not warn!
    }
}

struct S7<'a> {
    field: &'a S7<'a>,
}

impl<'a> PartialEq for S7<'a> {
    fn eq(&self, other: &Self) -> bool {
        //~^ ERROR: function cannot return without recursing
        let mine = &self.field;
        let theirs = &other.field;
        mine == theirs
    }
}

struct S8 {
    num: i32,
    field: Option<Box<S8>>,
}

impl PartialEq for S8 {
    fn eq(&self, other: &Self) -> bool {
        if self.num != other.num {
            return false;
        }

        let (this, other) = match (self.field.as_deref(), other.field.as_deref()) {
            (Some(x1), Some(x2)) => (x1, x2),
            (None, None) => return true,
            _ => return false,
        };

        this == other
    }
}

struct S9;

impl std::string::ToString for S9 {
    fn to_string(&self) -> String {
        //~^ ERROR: function cannot return without recursing
        self.to_string()
    }
}

struct S10;

impl std::string::ToString for S10 {
    fn to_string(&self) -> String {
        //~^ ERROR: function cannot return without recursing
        let x = self;
        x.to_string()
    }
}

struct S11;

impl std::string::ToString for S11 {
    fn to_string(&self) -> String {
        //~^ ERROR: function cannot return without recursing
        (self as &Self).to_string()
    }
}

struct S12;

impl std::default::Default for S12 {
    fn default() -> Self {
        Self::new()
    }
}

impl S12 {
    fn new() -> Self {
        //~^ ERROR: function cannot return without recursing
        Self::default()
    }

    fn bar() -> Self {
        // Should not warn!
        Self::default()
    }
}

#[derive(Default)]
struct S13 {
    f: u32,
}

impl S13 {
    fn new() -> Self {
        // Shoud not warn!
        Self::default()
    }
}

fn main() {
    // test code goes here
}
