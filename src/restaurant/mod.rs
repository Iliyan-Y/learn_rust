mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: "white".to_string(),
                cheese: "mozzarella".to_string(),
                topping: topping.to_string(),
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table")
        }
        pub fn take_order() {
            seat_at_table();
            let customer_pizza: super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(customer_pizza);
        }
        fn serve_customer(customer_pizza: super::Pizza) {
            println!(
                "Customer is served a pizza with: {}, {}, {}",
                customer_pizza.dough, customer_pizza.cheese, customer_pizza.topping
            )
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
