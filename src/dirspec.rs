use std::cmp::{Eq, Ord, Ordering, PartialOrd, PartialEq};
use std::fmt::{Debug, Formatter, Result as FmtResult, Write};
use std::fs::{DirEntry, File, read_dir};
use std::io::Read;
use std::path::Path;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

struct Entry {
    de: DirEntry,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.file_name().eq(&other.file_name())
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.file_name().partial_cmp(&other.file_name()).map (|t| reverse_ordering(t))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        reverse_ordering(self.file_name().cmp(&other.file_name()))
    }
}

fn reverse_ordering(o1: Ordering) -> Ordering {
    match o1 {
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
    }
}

impl Entry {
    pub fn new(de: DirEntry) -> Self {
        Entry { de: de }
    }

    pub fn file_name(&self) -> String {
        self.de.file_name().into_string().unwrap()
    }

    pub fn hash(&self) -> String {
        trace!("For path: {}", self.de.path().display());
        if self.de.path().is_file() {
            return file_hash(self.de.path());
        }

        let ds = DirSpec::new(self.de.path()).unwrap();
        ds.hash()
    }
}

fn file_hash<T: AsRef<Path>>(path: T) -> String {
    let mut data = Vec::new();
    let mut f = File::open(path.as_ref()).expect("Unable to open file");
    f.read_to_end(&mut data).expect("Unable to read data");
    bits2shastr(data.as_slice())
}

fn bits2shastr(input: &[u8]) -> String {
    let mut sha = Sha256::new();
    sha.input(input);
    let mut bytes = [0u8; 32];
    sha.result(&mut bytes);
    bytes_to_hexstr(&bytes)
}

fn bytes_to_hexstr(input: &[u8]) -> String {
    let mut s = String::new();
    for b in input {
        write!(&mut s, "{:01$x}", b, 2).unwrap();
    }
    s
}

pub struct DirSpec {
    list: Vec<Entry>,
}

impl DirSpec {
    pub fn new<T: AsRef<Path>>(base_path: T) -> Result<Self> {
        let p1 = read_dir(base_path.as_ref())?;
        let mut v1 = vec![];
        for path in p1 {
            let p1 = try!(path);
            v1.push(Entry::new(p1));
        }
        v1.sort();

        Ok(DirSpec { list: v1 })
    }

    pub fn hash(&self) -> String {
        let spec = format!("{:?}", self);
        trace!("spec:{}", spec);
        bits2shastr(spec.as_bytes())
    }
}

impl Debug for DirSpec {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for entry in &self.list {
            writeln!(f, "{} {}", entry.hash(), entry.file_name())?;
        }
        Ok(())
    }
}

type Result<T> = ::std::result::Result<T, SpecError>;

quick_error! {
    #[derive(Debug)]
    pub enum SpecError {
        Io(err: ::std::io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
    }
}