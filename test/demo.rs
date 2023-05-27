#[cfg(test)]
mod tests {
    use crate::common::util::time_util::get_current_millisecond;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn get_milliseconds(){
        let mini = get_current_millisecond();
        print!("{}",mini)
    }
}