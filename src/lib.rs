#[cfg(test)]
mod tests {
    use yrs::{Array, Transact};

    #[test]
    fn yrs() {
        let doc = yrs::Doc::new();
        let array = doc.get_or_insert_array("array1");
        let mut trx = doc.transact_mut();
        array.insert(&mut trx, 0, "1").unwrap();
    }
}
