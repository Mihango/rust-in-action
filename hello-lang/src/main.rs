fn main() {
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";

    let regions = [southern_germany, japan];
    greet_world(regions);
}

fn greet_world(regions: [&str; 2]) -> u32 {
    println!("Hello World!");

    regions
        .iter()
        .for_each(|region| println!("Region: {}", region));

    regions.len() as u32
}

mod test {
    
    #[test]
    fn test_greet_world() {
        let southern_germany = "Grüß Gott!";
        let japan = "ハロー・ワールド";

        let regions = [southern_germany, japan];
        let size = crate::greet_world(regions);
        assert_eq!(size, 2);
    }
}
