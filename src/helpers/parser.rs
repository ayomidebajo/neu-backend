use unicode_segmentation::UnicodeSegmentation;

use crate::models::{FilteredUser, GetCustomer};

pub fn name_parser(s: String) -> bool {
    // `.trim()` returns a view over the input `s` without trailing
    // whitespace-like characters.
    // `.is_empty` checks if the view contains any character.
    let is_empty_or_whitespace = s.trim().is_empty();
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters // (`a` and ``).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set, // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;
    // Iterate over all characters in the input `s` to check if any of them matches
    // one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

pub fn user_parser(u: GetCustomer) -> FilteredUser {
    FilteredUser {
        id: u.id,
        fname: u.fname.to_owned(),
        lname: u.lname.to_owned(),
        email: u.email.to_owned(),
        is_verified: u.is_verified_user.to_owned(),
        created_at: u.created_at.to_owned(),
    }
}
