// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unite_price: u32,
}

impl Order {
    fn check_name(_name: &str) {
        if _name.is_empty() || _name.len() > 300 {
            panic!("The product name should be not empty and less then 300 bytes");
        }
    }
    fn check_quantity(_quant: u32) {
        if _quant <= 0 {
            panic!("The quantity shold be greater than 0");
        }
    }
    fn check_unite_price(_price: u32) {
        if _price <= 0 {
            panic!("The unit price must be gearter then 0")
        }
    }
    pub fn new(_name: String, _quant: u32, _price: u32) -> Self {
        Self::check_name(_name.as_str());
        Self::check_unite_price(_price);
        Self::check_quantity(_quant);
        Self {
            product_name: _name,
            quantity: _quant,
            unite_price: _price,
        }
    }

    pub fn total(&self) -> u32 {
        self.unite_price * self.quantity
    }

    pub fn set_product_name(&mut self, _name: String) {
        Self::check_name(_name.as_str());
        self.product_name = _name;
    }

    pub fn set_unit_price(&mut self, _price: u32) {
        Self::check_unite_price(_price);
        self.unite_price = _price;
    }

    pub fn set_quantity(&mut self, _quant: u32) {
        Self::check_quantity(_quant);
        self.quantity = _quant;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unite_price
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
}
