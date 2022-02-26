use ndarray::{Array, Array3};
use covrus;

pub fn part_color(rgb_array: &Array3<u8>, color: u8) -> Array3<u8> {
    let mut part_array = vec![];
    let gray_array = covrus::cvt_rgb2gray(&rgb_array);
    let hsv_array = covrus::cvt_rgb2hsv(&rgb_array);
    let width = rgb_array.shape()[1] as usize;
    let height = rgb_array.shape()[0] as usize;
    match color {
        0 => {
            panic!("No meaning!");
        }
        1 => {
            for y in 0..height {
                for x in 0..width {
                    if hsv_array[[y,x,0]] <= 10. || hsv_array[[y,x,0]] >= 340. {
                        part_array.push(rgb_array[[y,x,0]] as u8);
                        part_array.push(rgb_array[[y,x,1]] as u8);
                        part_array.push(rgb_array[[y,x,2]] as u8);
                    } else {
                        part_array.push(gray_array[[y,x]] as u8);
                        part_array.push(gray_array[[y,x]] as u8);
                        part_array.push(gray_array[[y,x]] as u8);
                    }
                }
            }
        }
        2 => {
            for y in 0..height {
                for x in 0..width {
                    if hsv_array[[y,x,0]] >= 200. || hsv_array[[y,x,0]] <= 255. {
                        part_array.push(rgb_array[[y,x,0]] as u8);
                        part_array.push(rgb_array[[y,x,1]] as u8);
                        part_array.push(rgb_array[[y,x,2]] as u8);
                    } else {
                        part_array.push(gray_array[[y,x]] as u8);
                        part_array.push(gray_array[[y,x]] as u8);
                        part_array.push(gray_array[[y,x]] as u8);
                    }
                }
            }
        }
        3 => {
            for y in 0..height {
                for x in 0..width {
                    if hsv_array[[y,x,0]] >= 100. || hsv_array[[y,x,0]] <= 140. {
                        part_array.push(hsv_array[[y,x,0]] as u8);
                        part_array.push(hsv_array[[y,x,1]] as u8);
                        part_array.push(hsv_array[[y,x,2]] as u8);
                    } else {
                        part_array.push(gray_array[[y,x]] as u8);
                        part_array.push(gray_array[[y,x]] as u8);
                        part_array.push(gray_array[[y,x]] as u8);
                    }
                }
            }
        }
        _ => {
            panic!("Error.")
        }
    }
    Array::from_shape_vec((height, width, 3 as usize), part_array).unwrap()
}
