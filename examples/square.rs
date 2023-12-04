use image::{ImageBuffer, Rgb};
fn main() {
    let width = 100;
    let height = 100;
    let black = Rgb([0u8, 0u8, 0u8]);

    // Criando uma imagem 100x100 com um quadrado preto
    let mut img = ImageBuffer::from_fn(width, height, |x, y| {
        if x < 50 && y < 50 {
            black
        } else {
            Rgb([255u8, 255u8, 255u8])
        }
    });

    img.save("square.png").unwrap();
}
