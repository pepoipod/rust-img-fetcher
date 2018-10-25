extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let img_url = "";

    make_directory("result");
    download_image(img_url, "img", "result");
}

/// 与えられた名前でディレクトリを作成する.
///
/// # Examples
///
/// ```
/// make_directory("test");
/// ```
///
fn make_directory(dir_name: &str) -> u8 {
    match fs::create_dir_all(dir_name) {
        Err(e) => panic!("{}: {}", dir_name, e),
        Ok(_) => 0,
    }
}

/// urlのページ内のselectorのsrcから画像をダウンロードする.
///
/// # Examples
///
/// ```
/// download_image("http://google.com", "img", "result");
/// ```
///
fn download_image(url: &str, selector: &str, base_dir_name: &str) -> u8 {
    let mut res = reqwest::get(&url).unwrap();

    if res.status().is_success() {
        println!("{} open is success!", url);
    } else {
        println!("{} open is error!", url);
        return 1;
    }

    let body = res.text().unwrap();
    let fragment = Html::parse_document(&body);

    let image_selector = Selector::parse(selector).unwrap();

    let mut cnt = 0;

    for image in fragment.select(&image_selector) {
        // もしカスタムデータ属性にurlがあれば、attrをdata-xxxなどにする.
        let url = image.value().attr("src").unwrap();

        println!("image_url: {}", url);

        let mut res = reqwest::get(url).unwrap();
        let mut body: Vec<u8> = vec![];

        res.read_to_end(&mut body).unwrap();

        let path = format!("{}/{}.jpg", base_dir_name, cnt);

        let mut f = File::create(&path).unwrap();

        match f.write_all(&body) {
            Ok(_) => println!("download success!"),
            Err(e) => println!("{}", e),
        }

        cnt = cnt + 1;
    }

    0
}
