/*
    --- Day 8: Space Image Format ---
    The Elves' spirits are lifted when they realize you have an opportunity to reboot one of their Mars rovers, and so they are curious if you would spend a brief sojourn on Mars. You land your ship near the rover.

    When you reach the rover, you discover that it's already in the process of rebooting! It's just waiting for someone to enter a BIOS password. The Elf responsible for the rover takes a picture of the password (your puzzle input) and sends it to you via the Digital Sending Network.

    Unfortunately, images sent via the Digital Sending Network aren't encoded with any normal encoding; instead, they're encoded in a special Space Image Format. None of the Elves seem to remember why this is the case. They send you the instructions to decode it.

    Images are sent as a series of digits that each represent the color of a single pixel. The digits fill each row of the image left-to-right, then move downward to the next row, filling rows top-to-bottom until every pixel of the image is filled.

    Each image actually consists of a series of identically-sized layers that are filled in this way. So, the first digit corresponds to the top-left pixel of the first layer, the second digit corresponds to the pixel to the right of that on the same layer, and so on until the last digit, which corresponds to the bottom-right pixel of the last layer.

    For example, given an image 3 pixels wide and 2 pixels tall, the image data 123456789012 corresponds to the following image layers:

    Layer 1: 123
             456

    Layer 2: 789
             012
    The image you received is 25 pixels wide and 6 pixels tall.

    To make sure the image wasn't corrupted during transmission, the Elves would like you to find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
*/

use std::fs;

struct Layer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Layer {
    fn from_slice(input_data: &[u32], width: usize, height: usize) -> Layer {
        Layer {
            data: input_data.to_vec(),
            width: width,
            height: height,
        }
    }

    fn pixel(&self, row: usize, col: usize) -> u32 {
        self.data[row * self.width + col]
    }

    fn display(&self) {
        for row in self.data.chunks(self.width as usize) {
            println!("{}", row
                            .into_iter()
                            .map(|i| i.to_string())
                            .collect::<String>());
        }
        println!("");
    }

    fn count_digits(&self, value: u32) -> u32 {
        let count = self.data
                        .iter()
                        .filter(|&&d| d == value)
                        .map(|&x| x)
                        .collect::<Vec<u32>>()
                        .len();
        count as u32
    }

    fn checksum(&self, digit1: u32, digit2: u32) -> u32 {
        let count1 = self.count_digits(digit1);
        let count2 = self.count_digits(digit2);
        count1 * count2
    }
}

struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}

impl Image {
    fn from_slice(input_vec: &[u32], image_width: usize, image_height: usize) -> Image {
        let mut layers = Vec::new();
        let layer_size = image_width * image_height;
        for chunk in input_vec.chunks(layer_size) {
            let layer = Layer::from_slice(chunk, image_width, image_height);
            layers.push(layer);
        }

        Image {
            layers: layers,
            width: image_width,
            height: image_height,
        }
    }

    fn display(&self, layer_num: u32) {
        self.layers[layer_num as usize].display();
    }

    fn display_all(&self) {
         self.layers
            .iter()
            .for_each(|layer| layer.display());
    }
}

fn image_layer_with_fewest_zeros(image: &Image) -> usize {
    let mut layer_id = 0;
    let mut lowest_count = std::u32::MAX;

    for i in 0..image.layers.len() {
        let count = image.layers[i].count_digits(0);
        if count < lowest_count {
            layer_id = i;
            lowest_count = count;
        }
    }

    layer_id
}

fn checksum_fewest_zeros(image: &Image, digit1: u32, digit2: u32) -> u32 {
    let layer_num = image_layer_with_fewest_zeros(image);
    let layer = &image.layers[layer_num];
    layer.checksum(digit1, digit2)
}

pub fn solve() {
    let input = fs::read_to_string("src/day_08_input.txt")
                    .expect("Something went wrong reading the file");
    let input_vec: Vec<u32> = input
                            .trim()
                            .chars()
                            .map(|c| c.to_digit(10).unwrap())
                            .collect();
    let image = Image::from_slice(&input_vec, 25, 6);
    // image.display_all();

    let chk = checksum_fewest_zeros(&image, 1, 2);
    println!("Checksum(1, 2): {}", chk);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image() {
        let input_vec = [1,2,3,4,5,6,7,8,9,0,1,2];
        let image = Image::from_slice(&input_vec, 3, 2);
        assert_eq!(image.layers[0].data, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(image.layers[1].data, vec![7, 8, 9, 0, 1, 2]);
    }
}