use select::document::Document;
use select::predicate::{Class, Name};

fn get_data() -> Vec<(String, String)>{
    let site_url = "https://torrentsir50.com/bbs/";
    let document = Document::from(include_str!("./html/torrentsir/bbs.html"));
    let mut v = vec![];
    for node in document.find(Class("media-heading")){
        let title = node.text().trim().to_string();
        let bbs_link = 
            node.find(Name("a"))
                .next()
                .unwrap()
                .attr("href")
                .unwrap();
        let bbs_link = format!("{}{}", site_url, bbs_link.replace("./", ""));
        v.push((title, bbs_link));
    }
    v
}

fn get_magnet() -> String {
    let document = Document::from(include_str!("./html/torrentsir/magnet.html"));
    let  mut magnet = "";
    for node in document.find(Class("list-group-item")){
        if let Some(m) = node.find(Name("a")).next() {
            magnet = m.attr("href").unwrap();
        }
    }
    magnet.to_string()
}


#[cfg(test)]
mod torrentsir_tests {
    use super::*;
    #[test]
    fn get_data_func_test() {
        let data = get_data();
        let sample = &data[0];
        assert_eq!(
            "동상이몽2 너는 내 운명.E234.220221.720p-NEXT",
            sample.0,
        );
        assert_eq!(
            "https://torrentsir50.com/bbs/board.php?bo_table=entertain&wr_id=25446",
            sample.1,
        );
    }
    #[test]
    fn get_magnet_func_test(){
        let m = get_magnet();
        assert_eq!(
            "magnet:?xt=urn:btih:4fb8f765a74c5c41e4c78fe71b36780328d8512a",
            m
        );
    }
}
