use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};
use std::path::PathBuf;

type Err = Box<dyn std::error::Error>;

pub struct TorrentFile {
    path: PathBuf,
    buffer: Vec<u8>,
    str_buf: String
}

impl TorrentFile {
    pub fn new(f_name: &PathBuf) -> Result<Self, std::io::Error> {
        let f = std::fs::File::open(f_name)?;
        let mut reader = BufReader::new(f);
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;
        let str_buf = String::from_utf8_lossy(&buffer[..]).to_string();
        Ok(Self { path: f_name.clone(), buffer, str_buf })
    }

    pub fn get_num_field_value(&self, fld_name: &str) -> u32 {
        let mut value = 0u32;
        if let Some(xs) = self.str_buf.find(fld_name) {
            let (_, length) = get_pos_and_length(
                self.buffer.as_slice(),
                xs + fld_name.len() + 1, 'e'
            );
            value = length as u32;
        }
        value
    }

    pub fn get_pieces(&self) -> Vec<u8> {
        let pat = "pieces";
        return if let Some(xs) = self.str_buf.find(pat) {
            let (start_pos, length) = get_pos_and_length(
                self.buffer.as_slice(),
                xs + pat.len(), ':'
            );
            // println!("pieces: pos={start_pos}, length={length}");
            // увы, это копирование, но тут не должно быть много
            self.buffer[start_pos..start_pos + length].to_vec()
        } else {
            vec![]
        }
    }

    pub fn get_announces(&self) -> Vec<String> {
        let mut v = vec![];
        let pat = "announce-listl";
        if let Some(xs) = self.str_buf.find(pat) {
            // в этом случае будет список url в формате [l<length>:<url>e]
            // если следующий символ будет е, то список заканчивается
            let mut beg_pos = xs + pat.len();
            while self.buffer[beg_pos] == b'l' {
                let (start_pos, length) = get_pos_and_length(
                    self.buffer.as_slice(),
                    beg_pos + 1, ':'
                );
                v.push(String::from(
                    self.str_buf.get(start_pos..start_pos + length).unwrap())
                );
                beg_pos = start_pos + length + 1;
            }
        }
        v
    }

    // info в стиле lostfilm
    // в торрент файле lostfim блок info в целом имеет
    // структуру 4:info[содержимое]e
    pub fn get_light_info(&self) -> Vec<u8> {
        // поступаем просто - в торрент файле lostfim
        // блок info в целом имеет структуру 4:info[содержимое]e
        // нам нужно [содержимое]
        let pat = "4:info";
        return if let Some(xs) = self.str_buf.find(pat) {
            self.buffer[xs + pat.len()..self.buffer.len() - 1].to_vec()
        } else {
            vec![]
        }
    }
}

pub struct TorrentMetadata<'a> {
    tor_file: &'a TorrentFile,
    announces: Vec<String>,
    length: u32,
    piece_length: u32,
    pieces: Vec<u8>,
    hashinfo: Vec<u8>
}

impl<'a> TorrentMetadata<'a> {
    pub fn new(tf: &'a TorrentFile) -> Self {
        Self {
            tor_file: tf,
            announces: tf.get_announces(),
            length: tf.get_num_field_value("length"),
            piece_length: tf.get_num_field_value("piece length"),
            pieces: tf.get_pieces(),
            hashinfo: vec![]
        }
    }
}

impl Display for TorrentMetadata<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "File: {:?}", self.tor_file.path).unwrap();
        writeln!(f, "List of announces:").unwrap();
        for v in self.announces.iter() {
            writeln!(f, "{v}").unwrap();
        }
        writeln!(f, "length: {}", self.length).unwrap();
        writeln!(f, "piece_length: {}", self.piece_length).unwrap();
        writeln!(f, "pieces: {:?}", self.pieces)
    }
}

pub fn get_files(path: &PathBuf, pat: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut res = vec![];
    for f in std::fs::read_dir(path)? {
        let entry = f?.path();
        let f_ext = entry.extension();
        if f_ext.is_some() && f_ext.unwrap() == pat { res.push(entry); }
    }
    Ok(res)
}

// работаем с числом в виде [<num>suffix]
fn get_pos_and_length(buf: &[u8], index: usize, suffix: char) -> (usize, usize) {
    let xs: Vec<u8> = buf.iter().skip(index)
        .take_while(|x| **x != (suffix as u8))
        .map(|x| *x)
        .collect();
    // println!("get_pos: {xs:?}");
    (
        index + xs.len() + 1,
        String::from_utf8(xs).unwrap().parse::<usize>().unwrap()
    )
}

/*
fn main() -> Result<(), Err> {


    /*
    let path = std::path::PathBuf::from("c:/users/alexa/downloads");
    let tor_files = get_files(&path, "torrent")?;

    if tor_files.len() > 0 {
        for tf_path in tor_files.iter() {
            // println!("{:?}", tf.file_name());
            let tf = TorrentFile::new(&tf_path)?;
            let tf_metadata = TorrentMetadata::new(&tf);
            println!("{tf_metadata}");
        }
    } else {
        println!("Not found the *.torrent files in the location {path:?}");
    }
    */

    Ok(())
}
 */