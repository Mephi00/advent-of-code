use self::element::Element;

mod element;

pub fn main(input_str: &String) {
    let input_split = input_str.split("\n");

    let mut pairs = Vec::new();

    let mut temp = ("", "");

    for split in input_split {
        if temp.0.is_empty() {
            temp.0 = split;
        } else if temp.1.is_empty() {
            temp.1 = split;
        } else if !temp.1.is_empty() {
            pairs.push(temp);
            temp = ("", "");
        }
    }
}

fn parse_pair(pair: (&str, &str)) -> (Element, Element) {
    let mut temp_vec: Vec<Element> = Vec::new();
    let mut temp_return = (Element::NUM(0), Element::NUM(0));
    let mut layer = 0;

    for split in pair.0.split(",").map(|x| x.trim()) {
        if split.starts_with("[") {
            let mut curr_temp_list = Vec::new();
            for start_list in split.split("") {
                if let Ok(number) = start_list.parse() {
                    curr_temp_list.push(Element::NUM(number));
                } else {
                }
            }

            let mut curr_list;

            for _ in 0..layer {
                if let Element::LIST(ref mut curr_list_temp) = temp_return.0 {
                    curr_list = curr_list_temp;
                }
            }
        }
    }

    temp_return
}
