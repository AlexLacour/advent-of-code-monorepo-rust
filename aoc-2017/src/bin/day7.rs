use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use aoc_utils::{console, file_io};
use itertools::Itertools;
use prse::{parse, try_parse};

#[derive(Debug)]
struct Disk {
    name: String,
    size: i32,
    children: Vec<String>,
}

fn parse_disks_string(disks_string: &String) -> Disk {
    let res: (String, i32, Vec<String>) = match try_parse!(disks_string, "{} ({}) -> {:, :}") {
        Ok(res) => res,
        Err(_) => {
            let partial_res: (String, i32) = parse!(disks_string, "{} ({})");
            (partial_res.0, partial_res.1, Vec::new())
        }
    };

    // return
    Disk {
        name: res.0,
        size: res.1,
        children: res.2,
    }
}

fn get_disk_size(
    from_disk: &String,
    disks_graph: &HashMap<String, Disk>,
    sizes_cache: &mut HashMap<String, i32>,
) -> i32 {
    let mut children_size = 0;
    for child_name in &disks_graph[from_disk].children {
        if !sizes_cache.contains_key(child_name) {
            let child_size = get_disk_size(child_name, disks_graph, sizes_cache);
            sizes_cache.insert(child_name.to_string(), child_size);
        }
        children_size += sizes_cache[child_name];
    }
    let from_disk_size = disks_graph[from_disk].size + children_size;

    // return
    from_disk_size
}

fn main() {
    let input_raw_graph = file_io::read_input_file(Path::new(file!()));
    let input_raw_graph = file_io::parse_input_string::<String>(input_raw_graph, None);

    let disks: HashMap<String, Disk> = input_raw_graph
        .iter()
        .map(|disk_line| {
            let disk = parse_disks_string(&disk_line[0].to_string());
            (disk.name.clone(), disk)
        })
        .collect();

    let root_disk: String = {
        let children_disks: HashSet<&String> = disks
            .values()
            .map(|disk| &disk.children)
            .flatten()
            .collect();
        let all_disks: HashSet<&String> = HashSet::from_iter(disks.keys());

        // return
        all_disks
            .difference(&children_disks)
            .into_iter()
            .last()
            .expect("No root disk ?")
            .to_string()
    };

    console::display(root_disk.clone(), "Part 1");

    let mut disk_sizes: HashMap<String, i32> = HashMap::new();
    get_disk_size(&root_disk, &disks, &mut disk_sizes);

    let mut used_disk = &root_disk;
    let mut target_size: i32 = 0;

    loop {
        // get children sizes
        let children_size: Vec<i32> = disks[used_disk]
            .children
            .iter()
            .map(|name| disk_sizes[name])
            .collect();

        if !children_size.iter().all_equal() {
            // Get problematic child id by counting occurences
            for (child_id, size) in children_size.iter().enumerate() {
                let count = children_size
                    .clone()
                    .into_iter()
                    .filter(|x| x == size)
                    .count();
                if count <= 1 {
                    used_disk = &disks[used_disk].children[child_id];
                    target_size = children_size[(child_id + 1) % children_size.len()];
                    break;
                }
            }
        } else {
            let problematic_size = disks[used_disk].size + children_size.iter().sum::<i32>();
            let size_difference = target_size - problematic_size;
            let ideal_size = disks[used_disk].size + size_difference;

            console::display(ideal_size, "Part 2");
            break;
        }
    }
}
