/*
    --- Part Two ---
    Now you're ready to decode the image. The image is rendered by stacking the layers and aligning the pixels with the same positions in each layer. The digits indicate the color of the corresponding pixel: 0 is black, 1 is white, and 2 is transparent.

    The layers are rendered with the first layer in front and the last layer in back. So, if a given position has a transparent pixel in the first and second layers, a black pixel in the third layer, and a white pixel in the fourth layer, the final image would have a black pixel at that position.

    For example, given an image 2 pixels wide and 2 pixels tall, the image data 0222112222120000 corresponds to the following image layers:

    Layer 1: 02
             22

    Layer 2: 11
             22

    Layer 3: 22
             12

    Layer 4: 00
             00
    Then, the full image can be found by determining the top visible pixel in each position:

    The top-left pixel is black because the top layer is 0.
    The top-right pixel is white because the top layer is 2 (transparent), but the second layer is 1.
    The bottom-left pixel is white because the top two layers are 2, but the third layer is 1.
    The bottom-right pixel is black because the only visible pixel in that position is 0 (from layer 4).
    So, the final image looks like this:

    01
    10
    What message is produced after decoding your image?
*/

use std::fmt;

pub struct Layer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Layer {
    fn from_slice(input_data: &[u32], width: usize, height: usize) -> Layer {
        Layer {
            data: input_data.to_vec(),
            width,
            height,
        }
    }

    fn pixel(&self, row: usize, col: usize) -> u32 {
        self.data[row * self.width + col]
    }

    fn count_digits(&self, value: u32) -> u32 {
        let count = self.data.iter().filter(|&&d| d == value).count();
        count as u32
    }

    fn checksum(&self, digit1: u32, digit2: u32) -> u32 {
        let count1 = self.count_digits(digit1);
        let count2 = self.count_digits(digit2);
        count1 * count2
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in self.data.chunks(self.width as usize) {
            writeln!(
                f,
                "{}",
                row.iter().map(|i| i.to_string()).collect::<String>()
            )?;
        }
        Ok(())
    }
}

struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}

impl Image {
    fn from_slice(input_vec: &[u32], image_width: usize, image_height: usize) -> Image {
        let layer_size = image_width * image_height;
        let layers: Vec<Layer> = input_vec
            .chunks(layer_size)
            .map(|chunk| Layer::from_slice(chunk, image_width, image_height))
            .collect();
        Image {
            layers,
            width: image_width,
            height: image_height,
        }
    }

    fn display(&self, layer_num: u32) {
        println!("{}", self.layers[layer_num as usize]);
    }

    fn display_all(&self) {
        self.layers.iter().for_each(|layer| println!("{}", layer));
    }

    fn stack_layers(&self) -> Layer {
        let mut pixel_vec = Vec::new();
        for row in 0..self.layers[0].height {
            for col in 0..self.layers[0].width {
                let pixel = self
                    .layers
                    .iter()
                    .map(|layer| layer.pixel(row, col)) // Get the pixel value at this layer
                    .find(|&pixel| pixel != 2) // Find the first non-transparent pixel
                    .unwrap_or(2); // If all were transparent, default to the transparent value
                pixel_vec.push(pixel);
            }
        }
        // There's probably a better way to do this, by flattening each layer and zipping all layers together.
        // But, I can't figure out how to do it.

        Layer::from_slice(&pixel_vec, self.width, self.height)
    }
}

#[aoc(day8, part2)]
pub fn solve(input: &str) -> Layer {
    let input_vec: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let image = Image::from_slice(&input_vec, 25, 6);
    // image.display_all();

    image.stack_layers()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image() {
        let input_vec = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let image = Image::from_slice(&input_vec, 3, 2);
        assert_eq!(image.layers[0].data, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(image.layers[1].data, vec![7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_image_stack() {
        let input_vec = [0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        let image = Image::from_slice(&input_vec, 2, 2);
        let stacked_image: Layer = image.stack_layers();
        assert_eq!(stacked_image.data, vec![0, 1, 1, 0]);
    }
}
