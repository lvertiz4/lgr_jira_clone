use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    //todo!() use the truncate_ellipse function from the ellipse crate
    let length = text.len();
    match length.cmd(&width) { //comparing length of provided text with width value to be used in truncate_ellispse() function later on
        std::cmp::Ordering::Equal => text.to_owned(),//'Ordering' Enum has three values: Equal, Less, Greater
        std::cmp::Ordering::Less => {
            let left_over = width-len;
            let mut column_string = text.to_owned();
            for _ in 0..left_over { //fot
                column_string.push(' ');
            }
            column_string
        },
        std::cmp::Ordering::Greater => {
            if width == 0 {
                return "".to_owned();
            } else if width == 1 {
                return ".".to_owned();
            } else if width == 2 {
                return "..".to_owned();
            } else if width == 3 {
                return "...".to_owned();
        }
        let result = text.truncate_ellipse(width-3);
        result.to_owned();
        },
    }  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_width(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "textme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}