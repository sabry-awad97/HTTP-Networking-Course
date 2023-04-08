use prettytable::{color, format::consts::FORMAT_BOX_CHARS, Attr, Cell, Row, Table};
use std::collections::HashMap;

pub fn sort_pages(pages: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut pages_vec: Vec<(String, usize)> = pages.iter().map(|(k, v)| (k.clone(), *v)).collect();
    pages_vec.sort_by_key(|&(_, v)| std::cmp::Reverse(v));
    pages_vec
}

pub fn print_report(pages: &HashMap<String, usize>) {
    println!("==========");
    println!("REPORT");
    println!("==========");

    let sorted_pages = sort_pages(pages);

    let mut table = Table::new();
    table.set_format(*FORMAT_BOX_CHARS);
    table.add_row(Row::new(vec![
        Cell::new("URL").with_style(Attr::ForegroundColor(color::CYAN)),
        Cell::new("Internal Links").with_style(Attr::ForegroundColor(color::CYAN)),
    ]));

    for (url, count) in sorted_pages {
        table.add_row(Row::new(vec![
            Cell::new(&url).with_style(Attr::ForegroundColor(color::YELLOW)),
            Cell::new(&count.to_string()).with_style(Attr::ForegroundColor(color::GREEN)),
        ]));
    }

    table.printstd();
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_sort_pages() {
        let input = hashmap! {
            "url1".to_string() => 5,
            "url2".to_string() => 1,
            "url3".to_string() => 3,
            "url4".to_string() => 10,
            "url5".to_string() => 7,
        };
        let actual = sort_pages(&input);
        let expected = vec![
            ("url4".to_string(), 10),
            ("url5".to_string(), 7),
            ("url1".to_string(), 5),
            ("url3".to_string(), 3),
            ("url2".to_string(), 1),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sort_pages_null_case() {
        let input = hashmap! {};
        let actual = sort_pages(&input);
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
