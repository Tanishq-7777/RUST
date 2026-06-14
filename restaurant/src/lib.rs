//Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
mod front_of_house {
    pub mod hosting {
        use crate::eat_at_restaurant;

        pub fn add_to_waitlist() {
        }
        pub fn seat_at_table() {}
    }

    
    mod serving {
        fn take_order() {}

        fn serve_order() {
            crate::front_of_house::hosting::seat_at_table();
            //or
            super::hosting::seat_at_table();
            
        }

        fn take_payment() {}
    }

}

use crate::front_of_house::hosting as host;
fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();
    //using use keyword
    host::add_to_waitlist();
}