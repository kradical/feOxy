use dom;

pub fn parse_html(html_contents: &str) {
    let mut inside_tag = false;
    let mut tag = String::new();
    let mut text = String::new();
    let mut current_element = dom::element_node("root".to_string(), dom::AttrMap::new(), Vec::new());
    let mut new_element = dom::element_node("child".to_string(), dom::AttrMap::new(), Vec::new());

    current_element.children.push(new_element);
    print!("{}\n\n", current_element.children[0]);

    for character in html_contents.chars() {
        if character == '>' {
            parse_tag(&tag);
            tag = String::new();
            inside_tag = false;
        } else if character == '<' {
            parse_text(text);
            text = String::new();
            inside_tag = true;
        } else if inside_tag {
            tag.push(character);
        } else {
            text.push(character);
        }
    }
}

fn parse_tag(tag: &str) {
    let mut first_word = true;
    let mut attr_name = true;
    let mut tagname = String::new();
    let mut name_str = String::new();
    let mut value_str = String::new();
    let mut attributes = dom::AttrMap::new();    

    for character in tag.chars() {
        if character.is_whitespace() {
            process_attr(&mut name_str, &mut value_str, &mut attributes);
            attr_name = true;
            first_word = false;
            continue;
        }

        if character == '=' {
            attr_name = false;
            continue;
        }

        if first_word {
            tagname.push(character);
        } else if attr_name {
            name_str.push(character);
        } else {
            value_str.push(character);
        }
    }
    process_attr(&mut name_str, &mut value_str, &mut attributes);

    let elem = dom::element_node(tagname, attributes, Vec::new());
    print!("{}", elem);
}

fn parse_text(text: String) {
    let mut all_space = true;
    for character in text.chars() {
        if !character.is_whitespace() {
            all_space = false;
            break;
        }
    }
    if !all_space {
        let text_node = dom::text_node(text);
        print!("{}", text_node);       
    }
}

fn process_attr(name: &mut String, value: &mut String, attr_map: &mut dom::AttrMap) {
    if value.len() > 0 {
        value.remove(0);
        value.pop();   
    }

    if name.len() > 0 {
        attr_map.insert(name.clone(), value.clone());
        *name = String::new();
        *value = String::new();
    }
}