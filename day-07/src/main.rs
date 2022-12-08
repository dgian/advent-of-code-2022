use std::{
    fs, 
    path::PathBuf, 
    collections::BTreeMap
};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input file");
    let mut disk_drive = DiskDrive::from_path("/");

    input.lines()
        .for_each(|line| {
            let cd = Regex::new(r"^\$ cd ([\w|/]+)$").unwrap();
            let cd_up = Regex::new(r"^\$ cd \.\.$").unwrap();
            let file_entry = Regex::new(r"^(\d+) ([^\s]+)$").unwrap();

            cd.captures(line).map(
                |cmd| {
                    let dir = cmd.get(1).expect("Could not parse cd command").as_str();
                    disk_drive.change_dir(dir);
                }
            )
            .or_else(|| cd_up.find(line).map(
                |_| disk_drive.change_dir_up()
            ))
            .or_else(|| file_entry.captures(line).map(
                |cmd| {
                    let size = cmd.get(1)
                        .expect("Could not parse file cmd output")
                        .as_str().parse()
                        .expect("Could not parse file size");
                    disk_drive.add_file_size_to_dir(size);
                }
            ));
        });

    // Part1
    println!("Part 1 total size: {}", disk_drive.sum_of_less_than(100_000));
    
    // Part2
    println!("Part 2 minimum size: {}", disk_drive.min_amount_to_free_capacity(30_000_000));
}

struct DiskDrive {
    dirs: BTreeMap<PathBuf, u64>,
    pwd: PathBuf,
}

impl DiskDrive {
    const MAX_CAPACITY: u64 = 70_000_000;

    fn from_path(path: &str) -> Self {
        DiskDrive { dirs: BTreeMap::new(), pwd: PathBuf::from(path) }
    }

    fn change_dir(&mut self, dir: &str) {
        self.pwd.push(dir);
        println!("pwd \n{}", self.pwd.display());

        self.dirs.insert(self.pwd.clone(), 0);
    }

    fn change_dir_up(&mut self) {
        let dirs_clone = self.dirs.clone();
        let pwd_clone = self.pwd.clone();

        let size = dirs_clone.get(&pwd_clone).unwrap();
        self.pwd.push(pwd_clone.parent().unwrap());
        self.dirs.entry(pwd_clone).and_modify(|e| *e += size);

        println!("Adding directory size {} to pwd {}", size, self.pwd.display());
    }

    fn add_file_size_to_dir(&mut self, size: u64) {
        println!("Adding file size {} to pwd {}", size.to_string(), self.pwd.display());

        self.dirs.entry(self.pwd.clone()).and_modify(|e| *e += size);
    }

    fn sum_of_less_than(&self, threshold: u64) -> u64 {
        self.dirs.clone()
            .into_iter()
            .filter(|(k, v)| *v < threshold)
            .map(|(k, v)| v).
            sum()
    }

    fn min_amount_to_free_capacity(&self, free_capacity: u64) -> u64 {
        let root_size = self.dirs
            .get(&PathBuf::from("/"))
            .expect("No / directory found in this disk drive");

        self.dirs.clone()
            .into_iter()
            .filter(|(k, v)| *v > free_capacity - (DiskDrive::MAX_CAPACITY - root_size))
            .map(|(k, v)| v)
            .min()
            .expect("No suitable directory size found for removal")
    }
}
