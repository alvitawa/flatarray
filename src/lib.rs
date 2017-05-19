
mod lrf;

#[cfg(test)]
mod tests {

    use lrf::Dimension;

    #[test]
    fn ptest() {
        let d = Dimension::new(Box::new([23,3,42,2]));
        println!("{:?}", d);
    }
}
