use std::collections::BTreeMap;
// #[warn(dead_code)]
#[allow(dead_code)]

use std::collections::HashMap;
use std::iter::Peekable; 
use std::str::Chars;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
 RFC 
 * What's a json parser 
 * - It translates a json String to an class object, but it does through an intermediate representation
 * 
 * How should the intermediate representation look like 
 * 
 * JsonIR { // meaning Json Intermediate representation 
 *   Number, // for simplicity let's only assume i64  
 *   String, // for simplicity let's assume String consists of alphabets
 *   Object, // for simplicity let's assume it's a HashMap<Value, Value>, 
 *   [Object], // array of objects
 *   NULL, 
 * }
 * 
 * Our task in this is to convert a String into JsonIR 
 * 
 * How would sample Json Strings look 
 * 
 * Eg1: "23", "Hello", " { a: 1, b: "hello", c: {a: 1, b: 2 } }", "[4,1,2,5]"
 *
 */

#[allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
 enum JsonIR{
    Bool(bool),
    Number(i64), 
    String(String), // strings needs to be encapsulated by "" 
    Array(Vec<JsonIR>), // array needs to be encapsulated by []
    Object(HashMap<String, JsonIR>), // object needs to be encapsulated by {}
    NULL, 
 }

/**
 * Peekable Iterator provides the option to peek at the next element without consuming it
 * Peekable is not implemented for char type need to think why 
 */

 #[allow(dead_code)]
 impl JsonIR{

    fn new(&mut self, s: String) -> Result<JsonIR, ()> {

        let mut s = s.chars().peekable(); 
        self.build(&mut s) 
    }

    fn build(&mut self, s: &mut Peekable<Chars>) -> Result<JsonIR, ()>{ // why can't Peekable be implemented for char

        Self::skip_whitespace(s);

        match s.peek(){
            // Note only the start type of tokens are mentioned here,
            // this is important as we will handle the closing tokens inside the build functions
            Some(&'{') => Self::build_object(self, s), 
            Some(&'"') => Self::build_string(self, s), 
            Some(&'[') => Self::build_array(self, s), 
            Some(&c) if c.is_numeric() => Self::build_number(self, s), 
            _ => Ok(JsonIR::NULL),
        }
    }

    fn build_object(&mut self, s: &mut Peekable<Chars>) -> Result<JsonIR, ()> {
        Self::skip_whitespace(s);
        Self::consume(s, '{')?; 
        Self::skip_whitespace(s);

        let mut hm = HashMap::<String, JsonIR>::new();

        loop{
            Self::skip_whitespace(s);
            let key = Self::build_string(self, s)? ; 
            Self::consume(s, ':')?;

            Self::skip_whitespace(s);
            let value = Self::build(self, s)? ; 

            if let JsonIR::String(key) = key{
                hm.insert(key, value); 
            }

            if let Some(_) = s.next_if_eq(&',') {
                continue;
            } else {
                break;
            }
        }

        Self::skip_whitespace(s);
        Self::consume(s, '}')?; 
        Self::skip_whitespace(s);
        Ok(JsonIR::Object(hm))
    }

    fn build_array(&mut self, s: &mut Peekable<Chars>) -> Result<JsonIR, ()> {
        Self::skip_whitespace(s);
        Self::consume(s, '[')?; 
        Self::skip_whitespace(s);

        let mut arr = Vec::<JsonIR>::new(); 

        loop {
            Self::skip_whitespace(s); 
            let elem = Self::build(self, s)?;
            arr.push(elem); 
            Self::skip_whitespace(s); 
            if let Some(_) = s.next_if_eq(& ',') {
                continue; 
            } else {
                break;
            }
        }

        Self::skip_whitespace(s);
        Self::consume(s, ']')?; 
        Ok(JsonIR::Array(arr))
    }

    fn build_string(&mut self, s: &mut Peekable<Chars>) -> Result<JsonIR, ()> {
        let mut res = String::new(); 

        // i really liked the API. it's easy to read the code aka fluent
        while let Some(ch) = s.next_if(|&ch| ch != '"' ){
            // only do next if the lambda function is true 
            res.push(ch);
        }

        Self::consume(s, '"')?; 
        Ok(JsonIR::String(res))
    }

    fn build_number(&mut self, s: &mut Peekable<Chars>) -> Result<JsonIR, ()> {

        let mut res = String::new(); 

        while let Some(ch) = s.next_if(|&ch| ch.is_numeric()){
            // only do next if the lambda function is true 
            res.push(ch);
        }

        Ok(JsonIR::Number(res.parse().expect("Fail to parse the given number")))
    }

    fn consume(s: &mut Peekable<Chars>, ch: char) -> Result<(), ()> {
        
        if let Some(&c) = s.peek() {
            if c == ch {
                s.next();
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        } 
    }

    fn skip_whitespace(s: &mut Peekable<Chars>)  {
        while let Some(&ch) = s.peek(){
            if ch.is_whitespace(){
                s.next();
            } else {
                break;
            }
        }
    }
 }





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test1(){
        let a = r#"{"code": 200,
                    "payload": {
                        "features": [
                            "recursive",
                            "easy",
                            "fun"
                        ]
                    }
                }"#;

        let mut b = HashMap::<String, JsonIR>::new(); 
        b.insert("code".to_string(), JsonIR::Number(200));

        let mut features = vec![JsonIR::String("recursive".to_string()), JsonIR::String("easy".to_string()), JsonIR::String("fun".to_string())]; 
        let mut features_map = HashMap::<String, JsonIR>::new(); 
        features_map.insert("features".to_string(), JsonIR::Array(features));
        let mut payload = HashMap::<String, JsonIR>::new();
        payload.insert("payload".to_string(), JsonIR::Object(features_map)); 
        b.insert("payload".to_string(), JsonIR::Object(payload));

        let a = JsonIR::new(a.to_string()); 
        assert_eq!(a, b); 
    }
}
