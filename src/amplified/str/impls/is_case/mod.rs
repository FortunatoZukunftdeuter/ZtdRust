#[inline]
fn is_all(string: &str, fnc: impl FnMut(char) -> bool) -> bool {
    return string.chars().all(fnc);
}

fn has_any(string: &str, fnc: impl FnMut(char) -> bool) -> bool {
    return string.chars().any(fnc);
}

pub trait ZtdIsCase {
    fn is_all(&self, fnc: impl FnMut(char) -> bool) -> bool;
    fn is_lowercase(&self) -> bool;
    fn is_uppercase(&self) -> bool;
    fn is_alphabetic(&self) -> bool;
    fn is_numeric(&self) -> bool;
    fn is_alphanumeric(&self) -> bool;
    fn is_digit(&self, radix: u32) -> bool;
    fn is_control(&self) -> bool;
    fn is_whitespace(&self) -> bool;

    fn is_ascii_lowercase(&self) -> bool;
    fn is_ascii_uppercase(&self) -> bool;
    fn is_ascii_alphabetic(&self) -> bool;
    fn is_ascii_numeric(&self) -> bool;
    fn is_ascii_alphanumeric(&self) -> bool;
    fn is_ascii_digit(&self) -> bool;
    fn is_ascii_control(&self) -> bool;
    fn is_ascii_whitespace(&self) -> bool;
    fn is_ascii_hexdigit(&self) -> bool;
    fn is_ascii_graphic(&self) -> bool;
    fn is_ascii_punctuation(&self) -> bool;

    fn has_any(&self, fnc: impl FnMut(char) -> bool) -> bool;
    fn has_lowercase(&self) -> bool;
    fn has_uppercase(&self) -> bool;
    fn has_alphabetic(&self) -> bool;
    fn has_numeric(&self) -> bool;
    fn has_alphanumeric(&self) -> bool;
    fn has_digit(&self, radix: u32) -> bool;
    fn has_control(&self) -> bool;
    fn has_whitespace(&self) -> bool;

    fn has_ascii_lowercase(&self) -> bool;
    fn has_ascii_uppercase(&self) -> bool;
    fn has_ascii_alphabetic(&self) -> bool;
    fn has_ascii_alphanumeric(&self) -> bool;
    fn has_ascii_digit(&self) -> bool;
    fn has_ascii_control(&self) -> bool;
    fn has_ascii_whitespace(&self) -> bool;
    fn has_ascii_hexdigit(&self) -> bool;
    fn has_ascii_graphic(&self) -> bool;
    fn has_ascii_punctuation(&self) -> bool;
}

impl ZtdIsCase for str {
    #[inline]
    fn is_all(&self, fnc: impl FnMut(char) -> bool) -> bool {
        return is_all(self, fnc);
    }

    #[inline]
    fn is_lowercase(&self) -> bool {
        return is_all(self, char::is_lowercase);
    }

    #[inline]
    fn is_uppercase(&self) -> bool {
        return is_all(self, char::is_uppercase);
    }

    #[inline]
    fn is_alphabetic(&self) -> bool {
        return is_all(self, char::is_alphabetic);
    }

    #[inline]
    fn is_numeric(&self) -> bool {
        return is_all(self, char::is_numeric);
    }

    #[inline]
    fn is_alphanumeric(&self) -> bool {
        return is_all(self, char::is_alphanumeric);
    }

    #[inline]
    fn is_digit(&self, radix: u32) -> bool {
        return is_all(self, |c| c.is_digit(radix));
    }

    #[inline]
    fn is_control(&self) -> bool {
        return is_all(self, char::is_control);
    }

    #[inline]
    fn is_whitespace(&self) -> bool {
        return is_all(self, char::is_whitespace);
    }

    #[inline]
    fn is_ascii_lowercase(&self) -> bool {
        return is_all(self, |c| c.is_ascii_lowercase());
    }

    #[inline]
    fn is_ascii_uppercase(&self) -> bool {
        return is_all(self, |c| c.is_ascii_uppercase());
    }

    #[inline]
    fn is_ascii_alphabetic(&self) -> bool {
        return is_all(self, |c| c.is_ascii_alphabetic());
    }

    #[inline]
    fn is_ascii_numeric(&self) -> bool {
        return is_all(self, |c| c.is_ascii_digit());
    }

    #[inline]
    fn is_ascii_alphanumeric(&self) -> bool {
        return is_all(self, |c| c.is_ascii_alphanumeric());
    }

    #[inline]
    fn is_ascii_digit(&self) -> bool {
        return is_all(self, |c| c.is_ascii_digit());
    }

    #[inline]
    fn is_ascii_control(&self) -> bool {
        return is_all(self, |c| c.is_ascii_control());
    }

    #[inline]
    fn is_ascii_whitespace(&self) -> bool {
        return is_all(self, |c| c.is_ascii_whitespace());
    }

    #[inline]
    fn is_ascii_hexdigit(&self) -> bool {
        return is_all(self, |c| c.is_ascii_hexdigit());
    }

    #[inline]
    fn is_ascii_graphic(&self) -> bool {
        return is_all(self, |c| c.is_ascii_graphic());
    }

    #[inline]
    fn is_ascii_punctuation(&self) -> bool {
        return is_all(self, |c| c.is_ascii_punctuation());
    }
    #[inline]
    fn has_any(&self, fnc: impl FnMut(char) -> bool) -> bool {
        return has_any(self, fnc);
    }
    #[inline]
    fn has_lowercase(&self) -> bool {
        return has_any(self, char::is_lowercase);
    }
    #[inline]
    fn has_uppercase(&self) -> bool {
        return has_any(self, char::is_uppercase);
    }
    #[inline]
    fn has_alphabetic(&self) -> bool {
        return has_any(self, char::is_alphabetic);
    }
    #[inline]
    fn has_numeric(&self) -> bool {
        return has_any(self, char::is_numeric);
    }
    #[inline]
    fn has_alphanumeric(&self) -> bool {
        return has_any(self, char::is_alphanumeric);
    }
    #[inline]
    fn has_digit(&self, radix: u32) -> bool {
        return has_any(self, |c| c.is_digit(radix));
    }
    #[inline]
    fn has_control(&self) -> bool {
        return has_any(self, char::is_control);
    }
    #[inline]
    fn has_whitespace(&self) -> bool {
        return has_any(self, char::is_whitespace);
    }

    #[inline]
    fn has_ascii_lowercase(&self) -> bool {
        return has_any(self, |c| c.is_ascii_lowercase());
    }
    #[inline]
    fn has_ascii_uppercase(&self) -> bool {
        return has_any(self, |c| c.is_ascii_uppercase());
    }
    #[inline]
    fn has_ascii_alphabetic(&self) -> bool {
        return has_any(self, |c| c.is_ascii_alphabetic());
    }
    #[inline]
    fn has_ascii_alphanumeric(&self) -> bool {
        return has_any(self, |c| c.is_ascii_alphanumeric());
    }
    #[inline]
    fn has_ascii_digit(&self) -> bool {
        return has_any(self, |c| c.is_ascii_digit());
    }
    #[inline]
    fn has_ascii_control(&self) -> bool {
        return has_any(self, |c| c.is_ascii_control());
    }
    #[inline]
    fn has_ascii_whitespace(&self) -> bool {
        return has_any(self, |c| c.is_ascii_whitespace());
    }
    #[inline]
    fn has_ascii_hexdigit(&self) -> bool {
        return has_any(self, |c| c.is_ascii_hexdigit());
    }
    #[inline]
    fn has_ascii_graphic(&self) -> bool {
        return has_any(self, |c| c.is_ascii_graphic());
    }
    #[inline]
    fn has_ascii_punctuation(&self) -> bool {
        return has_any(self, |c| c.is_ascii_punctuation());
    }
}
