use chrono::NaiveDate;

fn main() {
    println!("Implement me!");
}

const NOW: &str = "2019-06-26";

struct User {
    birth_date: NaiveDate
}

impl User {
    /// # Panics
    /// On out-of-range date, invalid month and/or day
    fn with_birthdate(year: i32, month: u32, day: u32) -> Self {
        Self { birth_date: NaiveDate::from_ymd_opt(year, month, day).expect("Invalid date passed")  }
    }

    /// Returns current age of [`User`] in years.
    fn age(&self) -> u32 {
        // NOW is known to be valid
        let date_now: NaiveDate = NOW.parse().unwrap();

        date_now.years_since(self.birth_date).unwrap_or(0)
    }

    /// Checks if [`User`] is 18 years old at the moment.
    fn is_adult(&self) -> bool {
        self.age() >= 18
    }
}

#[cfg(test)]
mod age_spec {
    use super::*;

    #[test]
    fn counts_age() {
        for ((y, m, d), expected) in vec![
            ((1990, 6, 4), 29),
            ((1990, 7, 4), 28),
            ((0, 1, 1), 2019),
            ((1970, 1, 1), 49),
            ((2019, 6, 25), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn zero_if_birthdate_in_future() {
        for ((y, m, d), expected) in vec![
            ((2032, 6, 25), 0),
            //((2016, 6, 27), 2),
            ((3000, 6, 27), 0),
            ((9999, 6, 27), 0),
        ] {
            let user = User::with_birthdate(y, m, d);
            assert_eq!(user.age(), expected);
        }
    }

    #[test]
    fn is_adult_false_if_less_than_18() {
        let not_18_date = (2001, 6, 27);

        let (y, m, d) = not_18_date;
        let user = User::with_birthdate(y, m, d);

        assert!(!user.is_adult(), "actual user age: {}", user.age());
    }

    #[test]
    fn is_adult_true_if_more_than_18() {
        let not_18_date = (2001, 6, 25);

        let (y, m, d) = not_18_date;
        let user = User::with_birthdate(y, m, d);

        assert!(user.is_adult(), "actual user age: {}", user.age());
    }

    #[test]
    fn is_adult_true_if_exactly_18() {
        let not_18_date = (2001, 6, 26);

        let (y, m, d) = not_18_date;
        let user = User::with_birthdate(y, m, d);

        assert!(user.is_adult(), "actual user age: {}", user.age());
    }

    #[test]
    #[should_panic(expected = "Invalid date passed")]
    fn constructor_year_out_of_range() {
        let _ = User::with_birthdate(300000, 1, 1);
    }

    #[test]
    #[should_panic(expected = "Invalid date passed")]
    fn constructor_month_is_wrong() {
        let _ = User::with_birthdate(2023, 13, 1);
    }

    #[test]
    #[should_panic(expected = "Invalid date passed")]
    fn constructor_date_not_in_month() {
        let _ = User::with_birthdate(2023, 2, 29);
    }
}
