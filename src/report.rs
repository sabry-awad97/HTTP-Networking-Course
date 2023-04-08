use prettytable::{color, format::consts::FORMAT_BOX_CHARS, Attr, Cell, Row, Table};
use std::{collections::HashMap, error::Error, io::Write};

pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn sort_pages(
    pages: &HashMap<String, usize>,
    order: SortOrder,
    filter: impl Fn(&String, &usize) -> bool,
) -> Vec<(String, usize)> {
    let mut filtered_pages: Vec<(String, usize)> = pages
        .iter()
        .filter(|(k, v)| filter(k, v))
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    match order {
        SortOrder::Ascending => filtered_pages.sort_by(|a, b| a.1.cmp(&b.1)),
        SortOrder::Descending => filtered_pages.sort_by(|a, b| b.1.cmp(&a.1)),
    }

    filtered_pages
}

pub fn print_report(
    pages: &HashMap<String, usize>,
    writer: &mut impl Write,
) -> Result<(), Box<dyn Error>> {
    if pages.is_empty() {
        return Err("No pages found".into());
    }

    writeln!(writer, "==========")?;
    writeln!(writer, "REPORT")?;
    writeln!(writer, "==========")?;

    let sorted_pages = sort_pages(pages, SortOrder::Ascending, |_, _| true);

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

    table.print(writer)?;

    Ok(())
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
        let actual = sort_pages(&input, SortOrder::Descending, |_, _| true);
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
        let actual = sort_pages(&input, SortOrder::Descending, |_, _| true);
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
