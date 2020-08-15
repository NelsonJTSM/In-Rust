use std::collections::LinkedList;
use std::env;

fn main() {
    let args: LinkedList<String> = env::args().collect();

    match last_duplicate(&args) {
        Some(last) => println!("{}", last),
        None => println!("no dupey dupe :("),
    }

    if is_token_repeated_consecutively(&args, "dupe") {
        println!("dupe dupe!");
    }
}

fn is_token_repeated_consecutively(list: &LinkedList<String>, token: &str) -> bool {
    let mut prev_iter = list.into_iter();
    for item in list.into_iter().skip(1) {
        // We are able to unwrap() because prev_iter should always be one String
        // behind the for loop's iterator, therefore it will never be None.
        let prev = prev_iter.next().unwrap();
        if item == token && item == prev {
            return true;
        }
    }
    false
}

fn last_duplicate<'a>(list: &'a LinkedList<String>) -> Option<&'a str> {
    for (i, item) in list.into_iter().rev().enumerate() {
        for other in list.into_iter().rev().skip(i + 1) {
            if item == other {
                return Some(item.as_str());
            }
        }
    }

    None
}

mod tests {
    #[allow(unused)]
    use crate::*;

    #[test]
    fn correct_last_duplicate() {
        let test_args = vec![
            ("chocolate lava cake lava cake", Some("cake")),
            ("chocolate lava cake cake lava", Some("lava")),
            ("chocolate lava cake cake cake", Some("cake")),
            ("chocolate lava lava cake cake", Some("cake")),
            ("chocolate lava dupe cake dupe cake cake", Some("cake")),
            ("be careful with this dupe", None),
            (
                "chocolate lava dupe dupe cake dupe cake cake chocolate",
                Some("chocolate"),
            ),
            ("cupcake cupcake dupe dupe dupe dupe dupe", Some("dupe")),
        ];

        for (arg_num, element) in test_args.iter().enumerate() {
            let arg_num = arg_num + 1;
            let (args, expected) = *element;
            let args_as_str: LinkedList<&str> = args.split_whitespace().collect();
            let args_as_string: LinkedList<String> =
                args_as_str.into_iter().map(|x| String::from(x)).collect();

            let duplicate = last_duplicate(&args_as_string);
            assert_eq!(duplicate, expected, "Error on arg_num = {}", arg_num);
        }
    }

    #[test]
    fn correct_multiple_dupe() {
        let test_args = vec![
            ("chocolate lava cake lava cake", false),
            ("chocolate lava cake cake lava", false),
            ("chocolate lava cake cake cake", false),
            ("chocolate lava lava cake cake", false),
            ("chocolate lava dupe cake dupe cake cake", false),
            ("be careful with this dupe", false),
            (
                "chocolate lava dupe dupe cake dupe cake cake chocolate",
                true,
            ),
            ("cupcake cupcake dupe dupe dupe dupe dupe", true),
        ];

        for (arg_num, element) in test_args.iter().enumerate() {
            let arg_num = arg_num + 1;
            let (args, expected) = *element;
            let args_as_str: LinkedList<&str> = args.split_whitespace().collect();
            let args_as_string: LinkedList<String> =
                args_as_str.into_iter().map(|x| String::from(x)).collect();

            let has_double_dupe = is_token_repeated_consecutively(&args_as_string, "dupe");
            assert_eq!(has_double_dupe, expected, "Error on arg_num = {}", arg_num);
        }
    }
}
