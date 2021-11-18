// generate by command: cargo new --lib restaurant

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
