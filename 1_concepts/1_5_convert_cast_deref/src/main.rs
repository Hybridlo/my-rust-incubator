mod email_string;
mod random_t;

use std::{borrow::Borrow, fmt::Display};

use email_string::EmailString;
use random_t::Random;

fn print_as_ref<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref());
}

fn print_borrow<T: Borrow<str> + Display>(input: T) {
    println!("{}", input);
}

fn _print_as_ref2<T: AsRef<EmailString>>(input: T) {
    println!("{}", AsRef::<EmailString>::as_ref(&input));
}

fn print_borrow2<T: Borrow<EmailString> + Display>(input: T) {
    println!("{}", input);
}

fn print_as_ref3<T: AsRef<i32>>(input: T) {
    println!("{}", input.as_ref());
}

fn print_borrow3<T: Borrow<i32> + Display>(input: T) {
    println!("{}", input);
}


fn main() {
    let mut email: EmailString = "my@email.com".try_into().unwrap();   // using TryInto (which in this case depends on FromStr)
    let not_email: Result<EmailString, _> = "not_an_email".try_into();
    assert!(not_email.is_err());

    print_borrow("hey");
    print_borrow("hey".to_string());
    print_borrow(email.clone());    // using Borrow
    
    print_as_ref("hey");    // can be used because str manually implements AsRef<str>
    print_as_ref("hey".to_string());
    print_as_ref(&email);    // using AsRef<str>
    print_as_ref(&mut email);    // using AsRef<str>

    // using blanket impl on T, &T, &mut T
    print_borrow2(email.clone());
    print_borrow2(&email);
    print_borrow2(&mut email);

    // can't use these since AsRef doesn't have a blanket impl
    // _print_as_ref2(email.clone());
    // _print_as_ref2(&email);
    // _print_as_ref2(&mut email);
    println!("{}", email);      // using Display (also provides blanket impl of ToString)
    let _email_string: String = email.into();   // using From<EmailString> for String

    let random = Random::new(1, 2, 3);

    print_as_ref3(random.clone());
    print_as_ref3(random.clone());
    print_as_ref3(random.clone());

    print_borrow3(random.clone());
    print_borrow3(random.clone());
    print_borrow3(random.clone());

    println!("{}", random);
    println!("{}", random);
    println!("{}", random);

    println!("{}", random);
    println!("{}", random);
    println!("{}", random);
}
