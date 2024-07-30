use approximately::ApproxEq;

/// Assume this is an image structure.
/// I need to ensure that at least 80% of the blocks in the images are the same to consider the two images identical.
#[derive(Debug, Clone)]
struct Image(Vec<u8>);

impl ApproxEq for Image {
    fn approx<T: std::borrow::Borrow<Self>>(&self, other: T) -> bool {
        self.0
            .iter()
            .zip(other.borrow().0.iter())
            .filter(|(a, b)| a == b)
            .count() as f32
            / self.0.len() as f32
            >= 0.8
    }
}

fn main() {
    let image1 = Image(vec![1, 2, 3, 4, 5]);
    let image2 = Image(vec![1, 2, 3, 4, 6]);

    let image3 = Image(vec![1, 2, 3, 4, 5]);
    let image4 = Image(vec![1, 2, 3, 5, 6]);
    println!("image1 approx image2:{:?}", image1.approx(&image2));
    println!("image3 approx image4:{:?}", image3.approx(&image4));
}
