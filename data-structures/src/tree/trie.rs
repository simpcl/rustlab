struct Trie {
    state: Vec<[usize; 27]>,
    capacity: usize,
}

fn new_trie(_n: usize) -> Trie {
    Trie {
        state: vec![[0; 27]],
        capacity: 0,
    }
}

impl Trie {
    fn extend(&mut self) {
        if self.capacity < self.state.len() {
            return;
        }
        for _i in self.state.len()..(self.capacity + 1) {
            self.state.push([0; 27]);
        }
    }

    fn insert(&mut self, s: &str) {
        let mut root: usize = 0;
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            if self.state[root][index] == 0 {
                self.capacity += 1;
                self.extend();
                self.state[root][index] = self.capacity;
            }
            //println!(
            //    "==================== {} {} {}",
            //    c, root, self.state[root][index]
            //);
            root = self.state[root][index];
            self.state[root][26] += 1;
        }
    }

    fn search(&self, s: &str) -> usize {
        let mut root: usize = 0;
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            if self.state[root][index] == 0 {
                return 0;
            }
            root = self.state[root][index];
        }
        self.state[root][26]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut t = new_trie(1024);
        t.insert("abcd");
        assert_eq!(t.capacity, 4);
        assert_eq!(t.state[1][26], 1);
        t.insert("abcef");
        assert_eq!(t.capacity, 6);
        assert_eq!(t.state[1][26], 2);
    }

    #[test]
    fn test_search() {
        let mut t = new_trie(1024 * 1024);
        t.insert("abcd");
        t.insert("abcef");
        t.insert("abczzg");

        let word = "abce";
        println!("search {}: {}", word, t.search(word));
        let word = "abczf";
        println!("search {}: {}", word, t.search(word));
    }
}
