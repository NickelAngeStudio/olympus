#[cfg(test)]
mod tests {
    use crate::Hercules;


    #[test]
    #[should_panic]
    /// Test that Hercules CAN'T be created without workers
    fn create_hercules_no_worker() {

        let _hercules = Hercules::new(0);

    }


    #[test]
    /// Creating with only 1 thread and pushing 65536 labour
    /// 
    fn create_hercules() {

        let hercules = Hercules::new(1);

        for i in 0..65536 {
            hercules.push_labour(move || { println!("Executing labour #{}", i.clone()); });
        }
    }
}