fn get_number_from_article(article: &str) -> Result<u16, String> {
    article.split_whitespace()
        .last()
        .ok_or("no whitespace".to_string())
        .map(|s| s.parse::<u16>().expect("correct parsing"))
}

pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        return "".to_owned();
    }
    let mut res = vec![];
    list_cat.iter()
        .for_each(|&cat| {
            let articles: u16 = list_art.iter()
                .filter(|&art| art.starts_with(cat))
                .map(|&art| get_number_from_article(art))
                .filter(|res_sum| res_sum.is_ok())
                .map(|res_sum| res_sum.expect("something went wrong"))
                .sum();
            res.push(format!("({} : {})", cat, articles));
        });
    res.join(" - ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

        b = vec![];
        c = vec!["A"];
        dotest(b, c, "");

        b = vec!["ABAR 200"];
        c = vec![];
        dotest(b, c, "");
    }
}
