use std::{ops::{Add, Sub, AddAssign, SubAssign}, mem, iter::Sum};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Price(u64);
impl From<u64> for Price {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Add for Price {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Price {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Price {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Price {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Sum for Price {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self(0);

        for item in iter {
            res += item;
        }

        res
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Coin {
    One,
    Two,
    Five,
    Ten,
    Twenty,
    Fifty
}

impl TryFrom<Price> for Coin {
    type Error = ();

    fn try_from(value: Price) -> Result<Self, Self::Error> {
        match value.0 {
            1 => Ok(Coin::One),
            2 => Ok(Coin::Two),
            5 => Ok(Coin::Five),
            10 => Ok(Coin::Ten),
            20 => Ok(Coin::Twenty),
            50 => Ok(Coin::Fifty),
            _ => Err(())
        }
    }
}

impl Coin {
    fn make_array<V: Into<Price> + Clone>(amount_value_pairs: &[(u64, V)]) -> Vec<Self> {
        let mut res = vec![];
        
        for pair in amount_value_pairs {
            let price: Price = pair.1.clone().into();

            for _ in 0..pair.0 {
                res.push(price.try_into().unwrap())
            }
        }

        res
    }

    pub fn value(&self) -> Price {
        match self {
            Coin::One => 1.into(),
            Coin::Two => 2.into(),
            Coin::Five => 5.into(),
            Coin::Ten => 10.into(),
            Coin::Twenty => 20.into(),
            Coin::Fifty => 50.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Product {
    name: String,
    price: Price
}

impl Product {
    fn new<S: AsRef<str>, P: Into<Price>>(name: S, price: P) -> Self {
        Self {
            name: name.as_ref().to_string(),
            price: price.into()
        }
    }
}

#[derive(Debug)]
struct VendingMachine {
    products: Vec<Product>,
    coins: Vec<Coin>
}

impl VendingMachine {
    fn new(available_coins: Vec<Coin>, products: Vec<Product>) -> Self {
        Self {
            products,
            coins: available_coins,
        }
    }

    fn get_change_values(&self, mut change: Price) -> Option<Vec<Price>> {
        let mut coin_values = vec![];
        
        //check we can give the change
        for coin in &self.coins {
            let coin_val = coin.value();

            if change >= coin_val {
                change -= coin_val;
                coin_values.push(coin_val);
            }

            if change.0 == 0 {
                return Some(coin_values)
            }
        }

        return None
    }

    pub fn buy<S: AsRef<str>, P: Into<Price>>(&mut self, product_name: S, payment: P) -> Result<(Product, Vec<Coin>), ()> {
        let payment = payment.into();

        // find the product
        let product_index = self.products.iter().position(
            |item| item.name == product_name.as_ref()
        ).ok_or(())?;

        // don't remove it yet
        let product = self.products.get(product_index).ok_or(())?;

        if payment < product.price {
            return Err(())
        }

        // find how much change we need in their value, the specific prices of coins
        let change_amount = payment - product.price;
        let mut change_values = self.get_change_values(change_amount).ok_or(())?;

        let coins = mem::take(&mut self.coins);

        let (giving_change, leftover_change) = coins
            .into_iter()
            .partition(|coin| !change_values.is_empty() && coin.value() == change_values.remove(0));

        self.coins = leftover_change;
        let product = self.products.swap_remove(product_index);

        return Ok((product, giving_change))
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vending_machine() {
        let mut machine = VendingMachine::new(
            Coin::make_array(&[
                (2, 50),    // 100
                (3, 20),    // 60
                (5, 10),    // 50
                (10, 5),    // 50
                (25, 2),    // 50
                (50, 1)     // 50
            ]),             // total: 360
            vec![
                Product::new("Fanta", 250),
                Product::new("Pepsi", 300),
                Product::new("Snickers", 430),
            ]
        );

        let fanta = machine.buy("Fanta", 300);
        assert!(
            matches!(
                fanta,
                Ok((
                    ref product,
                    ref change
                )) if *product == Product { name: "Fanta".to_string(), price: 250.into() } && *change == vec![Coin::Fifty]
            ),
            "The result of buying was: {:?}",
            fanta
        );
        assert_eq!(machine.coins.iter().map(|coin| coin.value()).sum::<Price>(), 310.into());

        let pepsi = machine.buy("Pepsi", 610);
        assert!(
            matches!(
                pepsi,
                Ok((
                    ref product,
                    _
                )) if *product == Product { name: "Pepsi".to_string(), price: 300.into() }
            ),
            "The result of buying was: {:?}",
            pepsi
        );
        assert_eq!(machine.coins.iter().map(|coin| coin.value()).sum::<Price>(), 0.into());

        let fail = machine.buy("Snickers", 431);
        assert!(
            matches!(
                fail,
                Err(())
            ),
            "The result of buying was: {:?}",
            fail
        );
    }
}