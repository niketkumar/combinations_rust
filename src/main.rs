use std::io;
use std::collections::BTreeMap;

fn main() {
    println!("All Combinations!: (nC1) + (nC2) + .. + (nCn)");
    println!("Enter a word to generate all combinations:");

    let mut given = String::new();
    io::stdin().read_line(&mut given)
        .expect("Failed to read data");
    let data = given.trim();
    println!("Generating all combinations for {} ...", data);

    let n = data.len();
    let r = n;

    let all_indices_combos: AllCombos = combinations(n, r);
    let combos: Vec<String> = all_indices_combos
        .build(&data.as_bytes().to_vec())
        .iter()
        .map(|ref bytes| String::from_utf8(bytes.clone().to_vec()).unwrap())
        .collect();
    for combo in combos {
        println!("{:?}", combo);
    }
}

fn combinations(n: usize, r: usize) -> AllCombos {
    if r == 1 {
        let c_items = (0..n).map(|i| CItem { vs: vec![i] }).collect();
        let combos = Combos { c_items: c_items };
        let mut map = BTreeMap::new();
        map.insert(r, combos);
        AllCombos { map: map }
    } else {
        let all_combo_prev = combinations(n, r - 1);
        let combos_prev = all_combo_prev.map.get(&(r - 1)).unwrap();
        let combos = combos_prev.gen_combos(n);
        let mut map_prev = all_combo_prev.map.clone();
        map_prev.insert(r, combos);
        AllCombos { map: map_prev }
    }
}

#[derive(Clone, Debug)]
struct CItem {
    vs: Vec<usize>
}

#[derive(Clone, Debug)]
struct Combos {
    c_items: Vec<CItem>
}

#[derive(Debug)]
struct AllCombos {
    map: BTreeMap<usize, Combos>
}

impl AllCombos {
    fn build<T: Clone>(&self, data: &Vec<T>) -> Vec<Vec<T>> {
        let mut result: Vec<Vec<T>> = vec![];
        for (_, combos) in self.map.iter() {
            let mut combos_data = combos.build(data);
            result.append(&mut combos_data);
        }
        result
    }
}

impl Combos {
    fn gen_combos(&self, n: usize) -> Combos {
        let c_items = self.c_items.iter().flat_map(|ref c_item| c_item.gen_combos(n)).collect();
        Combos { c_items: c_items }
    }

    fn build<T: Clone>(&self, data: &Vec<T>) -> Vec<Vec<T>> {
        self.c_items.iter().map(|ref c_item| c_item.build(data)).collect()
    }
}

impl CItem {
    fn gen_combos(&self, n: usize) -> Vec<CItem> {
        let this = self;
        let max_index = this.vs.last().unwrap();
        (*max_index + 1..n).map(|i| this.append(i)).collect()
    }

    fn append(&self, i: usize) -> Self {
        let mut clone = self.vs.clone();
        clone.push(i);
        CItem { vs: clone }
    }

    fn build<T: Clone>(&self, data: &Vec<T>) -> Vec<T> {
        self.vs.iter().map(|i| data[*i].clone()).collect()
    }
}
