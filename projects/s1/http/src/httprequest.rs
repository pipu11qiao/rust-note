#[derive(Debug, PartialEq)]

pub enum Method {
    Get,
    Post,
    Unitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Unitialized,
        }
    }
}
#[cfg(text)]

mod tests {
    use super::*;
    fn test_method_inro() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get)
    }
}