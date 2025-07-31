use crate::TableStyle;

use serde_json::{Map, Value};

pub fn get_table_formatted(data: &Map<String, Value>, style: TableStyle) -> String {
    if data.is_empty() {
        return String::new();
    }

    let (mut max_key_len, mut max_value_len, size) = get_max_len(&data);
    max_key_len += 2;
    max_value_len += 2;

    let mut output = String::new();

    let chars = style.get_chars();

    // upper line
    output.push(chars.upper_left);
    for _ in 0..max_key_len {
        output.push(chars.horizontal);
    }
    output.push(chars.upper_medium);
    for _ in 0..max_value_len {
        output.push(chars.horizontal);
    }
    output.push(chars.upper_right);
    output.push('\n');

    for (pos, (key, value)) in data.iter().enumerate() {
        let value = match value {
            serde_json::Value::String(s) => s,
            other => &other.to_string(),
        };

        // row
        output.push(chars.vertical);
        output.push_str(&format!("{:^max_key_len$}", key));
        output.push(chars.vertical);
        output.push_str(&format!("{:^max_value_len$}", value));
        output.push(chars.vertical);
        output.push('\n');

        // medium line
        if pos < size - 1 {
            output.push(chars.medium_left);
            for _ in 0..max_key_len {
                output.push(chars.horizontal);
            }
            output.push(chars.medium_medium);
            for _ in 0..max_value_len {
                output.push(chars.horizontal);
            }
            output.push(chars.medium_right);
            output.push('\n');
        }
    }

    // lower line
    output.push(chars.lower_left);
    for _ in 0..max_key_len {
        output.push(chars.horizontal);
    }
    output.push(chars.lower_medium);
    for _ in 0..max_value_len {
        output.push(chars.horizontal);
    }
    output.push(chars.lower_right);
    output.push('\n');

    output
}

fn get_max_len(json_obj: &serde_json::Map<String, serde_json::Value>) -> (usize, usize, usize) {
    let mut max_key_len: usize = 0;
    let mut max_value_len: usize = 0;

    let size = json_obj
        .iter()
        .map(|(key, value)| {
            let key_len = key.len();
            let value_len = match value {
                serde_json::Value::String(s) => s.len(),
                other => other.to_string().len(),
            };
            if key_len > max_key_len {
                max_key_len = key_len;
            };

            if value_len > max_value_len {
                max_value_len = value_len;
            };
        })
        .count();

    (max_key_len, max_value_len, size)
}

#[cfg(test)]
mod tests {
    use super::{get_max_len, get_table_formatted};
    use crate::TableStyle;

    use serde_json::json;

    #[test]
    fn test_get_max_len() {
        let object = json!({
            "name": "Frank Thomas",
            "age": 23,
            "income_f": 23405.50,
            "income_s": "23405.50",
            "code": 10026,
            "cid_i": 012345678,
            "cid_s": "012345678",
        });

        let object = object.as_object().unwrap();

        assert_eq!(get_max_len(object), (8, 12, 7));
    }

    #[test]
    fn test_get_table_formatted() {
        let object = json!({
            "name": "Frank Thomas",
            "age": 23,
            "income_f": 23405.50,
            "income_s": "23405.50",
            "code": 10026,
            "cid_i": 012345678,
            "cid_s": "012345678",
        });

        let object = object.as_object().unwrap();

        let expected_table_ascii = "\
+----------+--------------+
|   name   | Frank Thomas |
+----------+--------------+
|   age    |      23      |
+----------+--------------+
| income_f |   23405.5    |
+----------+--------------+
| income_s |   23405.50   |
+----------+--------------+
|   code   |    10026     |
+----------+--------------+
|  cid_i   |   12345678   |
+----------+--------------+
|  cid_s   |  012345678   |
+----------+--------------+
";

        let expected_table_thin = "\
┌──────────┬──────────────┐
│   name   │ Frank Thomas │
├──────────┼──────────────┤
│   age    │      23      │
├──────────┼──────────────┤
│ income_f │   23405.5    │
├──────────┼──────────────┤
│ income_s │   23405.50   │
├──────────┼──────────────┤
│   code   │    10026     │
├──────────┼──────────────┤
│  cid_i   │   12345678   │
├──────────┼──────────────┤
│  cid_s   │  012345678   │
└──────────┴──────────────┘
";

        let expected_table_thin_rounded = "\
╭──────────┬──────────────╮
│   name   │ Frank Thomas │
├──────────┼──────────────┤
│   age    │      23      │
├──────────┼──────────────┤
│ income_f │   23405.5    │
├──────────┼──────────────┤
│ income_s │   23405.50   │
├──────────┼──────────────┤
│   code   │    10026     │
├──────────┼──────────────┤
│  cid_i   │   12345678   │
├──────────┼──────────────┤
│  cid_s   │  012345678   │
╰──────────┴──────────────╯
";

        let expected_table_thick = "\
┏━━━━━━━━━━┳━━━━━━━━━━━━━━┓
┃   name   ┃ Frank Thomas ┃
┣━━━━━━━━━━╋━━━━━━━━━━━━━━┫
┃   age    ┃      23      ┃
┣━━━━━━━━━━╋━━━━━━━━━━━━━━┫
┃ income_f ┃   23405.5    ┃
┣━━━━━━━━━━╋━━━━━━━━━━━━━━┫
┃ income_s ┃   23405.50   ┃
┣━━━━━━━━━━╋━━━━━━━━━━━━━━┫
┃   code   ┃    10026     ┃
┣━━━━━━━━━━╋━━━━━━━━━━━━━━┫
┃  cid_i   ┃   12345678   ┃
┣━━━━━━━━━━╋━━━━━━━━━━━━━━┫
┃  cid_s   ┃  012345678   ┃
┗━━━━━━━━━━┻━━━━━━━━━━━━━━┛
";

        let expected_table_double_thin = "\
╔══════════╦══════════════╗
║   name   ║ Frank Thomas ║
╠══════════╬══════════════╣
║   age    ║      23      ║
╠══════════╬══════════════╣
║ income_f ║   23405.5    ║
╠══════════╬══════════════╣
║ income_s ║   23405.50   ║
╠══════════╬══════════════╣
║   code   ║    10026     ║
╠══════════╬══════════════╣
║  cid_i   ║   12345678   ║
╠══════════╬══════════════╣
║  cid_s   ║  012345678   ║
╚══════════╩══════════════╝
";

        assert_eq!(
            get_table_formatted(object, TableStyle::Basic),
            expected_table_ascii
        );
        assert_eq!(
            get_table_formatted(object, TableStyle::Thin),
            expected_table_thin
        );
        assert_eq!(
            get_table_formatted(object, TableStyle::ThinRounded),
            expected_table_thin_rounded
        );
        assert_eq!(
            get_table_formatted(object, TableStyle::Thick),
            expected_table_thick
        );
        assert_eq!(
            get_table_formatted(object, TableStyle::ThinDouble),
            expected_table_double_thin
        );
    }
}
