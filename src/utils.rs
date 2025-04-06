use lazy_regex::regex;

pub fn is_url(v: &str) -> bool {
    let r = regex!(
        r"^https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)$"
    );
    r.is_match(&v)
}

pub fn is_email(v: &str) -> bool {
    let r = regex!(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$");
    r.is_match(&v)
}
