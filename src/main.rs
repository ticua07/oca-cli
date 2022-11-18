extern crate reqwest;

use soup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut url = String::from("https://www9.oca.com.ar/OEPTrackingWeb/detalleenviore.asp?numero=");
    url.push_str(
        &std::env::args()
            .last()
            .expect("Provee el numero de seguimiento de pedido"),
    );
    let body = reqwest::get(url).await?.text().await?;

    let soup = Soup::new(&body);

    let mut cleaned_td = Vec::new();
    let mut all_td: Vec<_> = soup.tag("td").find_all().collect();
    all_td = all_td.to_vec();

    for el in &all_td {
        if el.get("class").unwrap_or("asdasd".to_string()) == "texto linea-tabla" {
            let mut trimmed_value = el.text();
            trimmed_value = trimmed_value.trim().to_string();
            cleaned_td.push(trimmed_value);
            // println!("{:?}", el.text());
        }
    }

    let longest_string = &cleaned_td
        .iter()
        .fold(&cleaned_td[6], |acc, item| {
            if item.len() > acc.len() {
                &item
            } else {
                acc
            }
        })
        .len();

    let strings_vector: &Vec<usize> = &cleaned_td[7..]
        .to_owned()
        .into_iter()
        .map(|el| el.len())
        // .map(|el| el)
        .collect();

    let mut top_menu_bar = String::from("┌");

    for _ in 0..longest_string + 14 {
        let _ = top_menu_bar.push('─');
    }
    let _ = top_menu_bar.push('┐');

    let mut bottom_menu_bar = String::from("└");

    for _ in 0..longest_string + 14 {
        let _ = bottom_menu_bar.push('─');
    }
    let _ = bottom_menu_bar.push('┘');

    println!("{top_menu_bar}");

    for el in (6..cleaned_td.len()).step_by(2) {
        let mut formatted_string =
            String::from(format!("│ {}: {}", &cleaned_td[el], &cleaned_td[el + 1]));

        for _ in 0..longest_string - strings_vector[el - 6] + 1 {
            let _ = &formatted_string.push(' ');
        }

        let _ = &formatted_string.push('│');

        println!("{}", formatted_string);
    }

    println!("{bottom_menu_bar}");

    Ok(())
}
