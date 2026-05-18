use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;
use serde_json::Value;
use std::{fs::File, fs::write, io::Read};
use url::Url;
use zip::ZipArchive;

#[derive(Serialize, Debug)]
struct Verse {
    volume: String,
    book: String,
    chapter: i32,
    verse_num: i32,
    text: String,
    footnotes: Vec<String>,
}

const BASE_URL: &str = "https://openscriptureapi.org/api/scriptures/v1/lds/en/";

fn generate_url(book: &str, chapter: &i32) -> Url {
    let base = Url::parse(BASE_URL).unwrap();
    let (volume, book) = lookup_book(book).unwrap();
    return base
        .join(
            format!(
                "volume/{}/{}/{}?includeExtras.footnotes",
                volume, book, chapter
            )
            .as_str(),
        )
        .unwrap();
}

fn lookup_book(epub_id: &str) -> Option<(&'static str, &'static str)> {
    match epub_id {
        // Old Testament
        "gen" => Some(("oldtestament", "genesis")),
        "ex" => Some(("oldtestament", "exodus")),
        "lev" => Some(("oldtestament", "leviticus")),
        "num" => Some(("oldtestament", "numbers")),
        "deut" => Some(("oldtestament", "deuteronomy")),
        "josh" => Some(("oldtestament", "joshua")),
        "judg" => Some(("oldtestament", "judges")),
        "ruth" => Some(("oldtestament", "ruth")),
        "1-sam" => Some(("oldtestament", "1samuel")),
        "2-sam" => Some(("oldtestament", "2samuel")),
        "1-kgs" => Some(("oldtestament", "1kings")),
        "2-kgs" => Some(("oldtestament", "2kings")),
        "1-chr" => Some(("oldtestament", "1chronicles")),
        "2-chr" => Some(("oldtestament", "2chronicles")),
        "ezra" => Some(("oldtestament", "ezra")),
        "neh" => Some(("oldtestament", "nehemiah")),
        "esth" => Some(("oldtestament", "esther")),
        "job" => Some(("oldtestament", "job")),
        "ps" => Some(("oldtestament", "psalms")),
        "prov" => Some(("oldtestament", "proverbs")),
        "eccl" => Some(("oldtestament", "ecclesiastes")),
        "song" => Some(("oldtestament", "songofsolomon")),
        "isa" => Some(("oldtestament", "isaiah")),
        "jer" => Some(("oldtestament", "jeremiah")),
        "lam" => Some(("oldtestament", "lamentations")),
        "ezek" => Some(("oldtestament", "ezekiel")),
        "dan" => Some(("oldtestament", "daniel")),
        "hosea" => Some(("oldtestament", "hosea")),
        "joel" => Some(("oldtestament", "joel")),
        "amos" => Some(("oldtestament", "amos")),
        "obad" => Some(("oldtestament", "obadiah")),
        "jonah" => Some(("oldtestament", "jonah")),
        "micah" => Some(("oldtestament", "micah")),
        "nahum" => Some(("oldtestament", "nahum")),
        "hab" => Some(("oldtestament", "habakkuk")),
        "zeph" => Some(("oldtestament", "zephaniah")),
        "hag" => Some(("oldtestament", "haggai")),
        "zech" => Some(("oldtestament", "zechariah")),
        "mal" => Some(("oldtestament", "malachi")),
        // New Testament
        "matt" => Some(("newtestament", "matthew")),
        "mark" => Some(("newtestament", "mark")),
        "luke" => Some(("newtestament", "luke")),
        "john" => Some(("newtestament", "john")),
        "acts" => Some(("newtestament", "acts")),
        "rom" => Some(("newtestament", "romans")),
        "1-cor" => Some(("newtestament", "1corinthians")),
        "2-cor" => Some(("newtestament", "2corinthians")),
        "gal" => Some(("newtestament", "galatians")),
        "eph" => Some(("newtestament", "ephesians")),
        "philip" => Some(("newtestament", "philippians")),
        "col" => Some(("newtestament", "colossians")),
        "1-thes" => Some(("newtestament", "1thessalonians")),
        "2-thes" => Some(("newtestament", "2thessalonians")),
        "1-tim" => Some(("newtestament", "1timothy")),
        "2-tim" => Some(("newtestament", "2timothy")),
        "titus" => Some(("newtestament", "titus")),
        "philem" => Some(("newtestament", "philemon")),
        "heb" => Some(("newtestament", "hebrews")),
        "james" => Some(("newtestament", "james")),
        "1-pet" => Some(("newtestament", "1peter")),
        "2-pet" => Some(("newtestament", "2peter")),
        "1-jn" => Some(("newtestament", "1john")),
        "2-jn" => Some(("newtestament", "2john")),
        "3-jn" => Some(("newtestament", "3john")),
        "jude" => Some(("newtestament", "jude")),
        "rev" => Some(("newtestament", "revelation")),
        // Book of Mormon
        "1-ne" => Some(("bookofmormon", "1nephi")),
        "2-ne" => Some(("bookofmormon", "2nephi")),
        "jacob" => Some(("bookofmormon", "jacob")),
        "enos" => Some(("bookofmormon", "enos")),
        "jarom" => Some(("bookofmormon", "jarom")),
        "omni" => Some(("bookofmormon", "omni")),
        "w-of-m" => Some(("bookofmormon", "wordsofmormon")),
        "mosiah" => Some(("bookofmormon", "mosiah")),
        "alma" => Some(("bookofmormon", "alma")),
        "hel" => Some(("bookofmormon", "helaman")),
        "3-ne" => Some(("bookofmormon", "3nephi")),
        "4-ne" => Some(("bookofmormon", "4nephi")),
        "morm" => Some(("bookofmormon", "mormon")),
        "ether" => Some(("bookofmormon", "ether")),
        "moro" => Some(("bookofmormon", "moroni")),
        // Doctrine and Covenants
        "dc" => Some(("doctrineandcovenants", "doctrineandcovenants")),
        // Pearl of Great Price
        "moses" => Some(("pearlofgreatprice", "moses")),
        "abr" => Some(("pearlofgreatprice", "abraham")),
        "js-m" => Some(("pearlofgreatprice", "josephsmithmatthew")),
        "a-of-f" => Some(("pearlofgreatprice", "articlesoffaith")),
        _ => None,
    }
}

fn extract_veres(
    volume: &str,
    book: &str,
    chapter: &i32,
    text: &String,
    api_respsone: &Value,
) -> Vec<Verse> {
    let text = Html::parse_document(text);
    let selector = Selector::parse("p").unwrap();
    let verse_re = Regex::new(r"(\d*)(.*)").unwrap();

    let mut verses: Vec<Verse> = Vec::new();

    for element in text.select(&selector) {
        if let Some(caps) = verse_re.captures(&element.text().collect::<String>()) {
            let verse = Verse {
                volume: volume.to_string(),
                book: book.to_string(),
                chapter: chapter.to_owned(),
                verse_num: caps[1].parse::<i32>().unwrap(),
                text: caps[2].split_whitespace().collect::<Vec<_>>().join(" "),
                footnotes: api_respsone["chapter"]["verses"]
                    [(caps[1].parse::<i32>().unwrap() - 1) as usize]["footNotes"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v["text"].as_str().unwrap().to_string())
                    .collect(),
            };
            verses.push(verse);
        }
    }

    return verses;
}

fn main() {
    let all_text_re = Regex::new(r"OEBPS/Text/.*\d\.xhtml\b").unwrap();
    let book_and_chap_re = Regex::new(r"/(\w*|\w*-\w*)/\d*_000_(.*)_(\d*)").unwrap(); //TODO refactor to combine these.^^^^

    let file = File::open("standard_works.epub").unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let mut verses: Vec<Verse> = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let name = entry.name().to_owned();

        if all_text_re.is_match(&name) {
            if let Some(caps) = book_and_chap_re.captures(&name) {
                if &caps[2] != "od" {
                    let volume = &caps[1];
                    let book = &caps[2];
                    let chapter = &caps[3].parse::<i32>().unwrap();

                    // extract and parse the text
                    let mut raw = Vec::new();
                    entry.read_to_end(&mut raw).unwrap();
                    let u16s: Vec<u16> = raw[2..]
                        .chunks_exact(2)
                        .map(|c| u16::from_le_bytes([c[0], c[1]]))
                        .collect();
                    let content = String::from_utf16_lossy(&u16s);

                    let target_url = generate_url(book, chapter);
                    let response = reqwest::blocking::get(target_url)
                        .unwrap()
                        .json::<serde_json::Value>()
                        .unwrap();

                    verses.extend(extract_veres(volume, book, chapter, &content, &response));
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&verses).unwrap();
    write("verses.json", json).unwrap();
}

// unzip the epub
// for each verse, make a json object and add the scriptures that are in the refrences to the object
// write the json objects to a file
