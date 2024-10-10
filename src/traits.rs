use super::prelude::*;

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for &str {
    fn to_string(&self) -> String {
        String::from(*self)
    }
}

impl ToString for u8 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 3];
        let mut i = 0;
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for u16 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 5];
        let mut i = 0;
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for u32 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 10];
        let mut i = 0;
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for u64 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 20];
        let mut i = 0;
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for usize {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 20];
        let mut i = 0;
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for i8 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 4];
        let mut i = 0;
        if n < 0 {
            result.push('-');
            n = -n;
        }
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for i16 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 6];
        let mut i = 0;
        if n < 0 {
            result.push('-');
            n = -n;
        }
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for i32 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 11];
        let mut i = 0;
        if n < 0 {
            result.push('-');
            n = -n;
        }
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for i64 {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 20];
        let mut i = 0;
        if n < 0 {
            result.push('-');
            n = -n;
        }
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}

impl ToString for isize {
    fn to_string(&self) -> String {
        let mut result = String::new();
        let mut n = *self;
        let mut buf = [0u8; 20];
        let mut i = 0;
        if n < 0 {
            result.push('-');
            n = -n;
        }
        while n > 0 {
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            i += 1;
        }
        while i > 0 {
            i -= 1;
            result.push(buf[i] as char);
        }
        result
    }
}
