mod front_of_house {
    pub mod hosting {
        use std::io;
        pub fn add_to_waitlist() {
        }

        fn seat_to_table() {}
        pub mod robot {
            fn startup() {
                super::seat_to_table();
            }
        }
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }

    fn door() {

    }
}

mod back_of_house {
    pub mod hosting {

    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}


pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
