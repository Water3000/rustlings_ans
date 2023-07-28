// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.


trait AppendBar {
    fn append_bar(self) -> Self;
    fn append_bar_flex(&mut self) -> Self;
    fn count(&self) -> usize;
}

impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut nv = Vec::from(self);
        nv.push("Bar".to_string());
        nv
    }

    fn append_bar_flex(&mut self) -> Self {
        self.push("Bar".to_string());
    
        let nv = self.clone();
        nv
    }

    fn count(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

    #[test]
    fn is_vec_pop_eq_bar_flex() {
        let mut foo = vec![String::from("Foo")];
        foo.append_bar_flex();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

    #[test]
    fn is_vec_pop_eq_count() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.count(), 2);
        assert_eq!(foo.count(), 2);
    }
}
