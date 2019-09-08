use crate::redisraw::bindings::RedisModuleCallReply;
use crate::{raw, RedisResult, RedisError};

#[derive(Debug, PartialEq)]
pub enum RedisValue {
    SimpleStringStatic(&'static str),
    SimpleString(String),
    BulkString(String),
    Integer(i64),
    Float(f64),
    Array(Vec<RedisValue>),
    None,
}

impl RedisValue {
    pub fn from_ptr(reply: *mut RedisModuleCallReply) -> RedisResult {
        match raw::call_reply_type(reply) {
            raw::ReplyType::String => {
                let string = raw::call_reply_string(reply);
                Ok(RedisValue::SimpleString(string))
            },
            raw::ReplyType::Integer => {
                let integer = raw::call_reply_integer(reply);
                Ok(RedisValue::Integer(integer))
            },
            raw::ReplyType::Array => {
                let len = raw::call_reply_length(reply);
                let mut array = Vec::with_capacity(len);

                for i in 0..len {
                    let reply = raw::call_reply_array_element(reply, i);
                    let elem = RedisValue::from_ptr(reply)?;
                    array.push(elem);
                }

                Ok(RedisValue::Array(array))
            },
            raw::ReplyType::Nil => Ok(RedisValue::None),
            raw::ReplyType::Unknown | raw::ReplyType::Error => {
                Err(RedisError::String(raw::call_reply_string(reply)))
            },
        }
    }
}

impl From<()> for RedisValue {
    fn from(_: ()) -> Self {
        RedisValue::None
    }
}

impl From<i64> for RedisValue {
    fn from(i: i64) -> Self {
        RedisValue::Integer(i)
    }
}

impl From<usize> for RedisValue {
    fn from(i: usize) -> Self {
        (i as i64).into()
    }
}

impl From<f64> for RedisValue {
    fn from(f: f64) -> Self {
        RedisValue::Float(f)
    }
}

impl From<String> for RedisValue {
    fn from(s: String) -> Self {
        RedisValue::BulkString(s)
    }
}

impl From<&str> for RedisValue {
    fn from(s: &str) -> Self {
        s.to_owned().into()
    }
}

impl From<&String> for RedisValue {
    fn from(s: &String) -> Self {
        s.to_owned().into()
    }
}

impl<T: Into<RedisValue>> From<Option<T>> for RedisValue {
    fn from(s: Option<T>) -> Self {
        match s {
            Some(v) => v.into(),
            None => RedisValue::None,
        }
    }
}

impl<T: Into<RedisValue>> From<Vec<T>> for RedisValue {
    fn from(items: Vec<T>) -> Self {
        RedisValue::Array(items.into_iter().map(|item| item.into()).collect())
    }
}

//////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::RedisValue;

    #[test]
    fn from_vec_string() {
        assert_eq!(
            RedisValue::from(vec!["foo".to_string()]),
            RedisValue::Array(vec![RedisValue::BulkString("foo".to_owned())])
        );
    }

    #[test]
    fn from_vec_str() {
        assert_eq!(
            RedisValue::from(vec!["foo"]),
            RedisValue::Array(vec![RedisValue::BulkString("foo".to_owned())])
        );
    }

    #[test]
    fn from_vec_string_ref() {
        assert_eq!(
            RedisValue::from(vec![&"foo".to_string()]),
            RedisValue::Array(vec![RedisValue::BulkString("foo".to_owned())])
        );
    }

    #[test]
    fn from_option_str() {
        assert_eq!(
            RedisValue::from(Some("foo")),
            RedisValue::BulkString("foo".to_owned())
        );
    }

    #[test]
    fn from_option_none() {
        assert_eq!(RedisValue::from(None::<()>), RedisValue::None,);
    }
}
