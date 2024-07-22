pub fn landscape_type(mut nums: Vec<u8>) -> &'static str {
    let mut landscape = "neither";
    let mut previus:u8 = 0;
    let mut count:u8 = 0;
    // let size = nums.len();

    for current in nums {
        count += 1;
        if count == 1 {
            previus = current;
        } else {
            if current > previus && ( landscape == "neither" || landscape == "mountain") {
                previus = current;
                landscape = "mountain"
            }
            else if current < previus && landscape == "mountain" {
                previus = current;
                landscape = "mountain"
            }
            else if current < previus && ( landscape == "neither" || landscape == "valley") {
                previus = current;
                landscape = "valley"
            }
            else if current > previus && landscape == "valley" {
                previus = current;
                landscape = "valley"
            }
            else {
                landscape = "valley"
            }
        }
    }

    return landscape;
}

#[cfg(test)]
mod tests {
    use super::*;
        
    // #[test]
    // fn test01() {
    //     // LandscapeType([3, 4, 5, 4, 3]) ➞ "mountain"
    //     let result = landscape_type(vec![3, 4, 5, 4, 3]);
    //     assert_eq!(result, "mountain");
    // }

    // LandscapeType([9, 7, 3, 1, 2, 4]) ➞ "valley"
    
    // LandscapeType([9, 8, 9]) ➞ "valley"
    
    // LandscapeType([9, 8, 9, 8]) ➞ "neither"

    #[test]
    fn test05() {
        let result = landscape_type(vec![7, 6, 5, 4, 3]);
        assert_eq!(result, "neither");
    }
    // #[test]
    // fn test06() {
    //     let result = landscape_type(vec![7, 6, 5, 4, 5]);
    //     assert_eq!(result, "valley");
    // }

    // #[test]
    // fn test07() {
    //     let result = landscape_type(vec![2, 3, 3, 4, 5]);
    //     assert_eq!(result, "neither");
    // }
}
