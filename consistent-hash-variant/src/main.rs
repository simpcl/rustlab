use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct ReplicaSet {
    slot_id: u64,
    node_map: HashMap<String, u32>,
}

#[derive(Debug)]
struct Node {
    name: String,
    slot_set: HashSet<(u64, u32)>,
}

#[derive(Debug)]
struct ClusterManager {
    max_slot_id: u64,
    replicaset_map: HashMap<u64, ReplicaSet>,
    node_map: HashMap<String, Node>,
}

impl ReplicaSet {
    fn new(id: u64) -> ReplicaSet {
        ReplicaSet {
            slot_id: id,
            node_map: HashMap::new(),
        }
    }

    fn reset(&mut self) {
        self.node_map.clear();
    }

    fn get_primary(&self) -> Option<&str> {
        for (name, role) in &self.node_map {
            if *role == 0 {
                return Some(name.as_str());
            }
        }
        None
    }

    fn assign(&mut self, node_name: &str, role: u32) {
        self.node_map.insert(String::from(node_name), role);
    }

    fn migrate(&mut self, node_name: &str) {
        self.node_map.remove(&String::from(node_name));
    }
}

impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: String::from(name),
            slot_set: HashSet::new(),
        }
    }

    fn pickup_slot(&mut self, slot_id: u64, role: u32) {
        self.slot_set.insert((slot_id, role));
    }
}

impl ClusterManager {
    pub fn new(slot_num: u64) -> ClusterManager {
        let mut cm = ClusterManager {
            max_slot_id: slot_num,
            replicaset_map: HashMap::new(),
            node_map: HashMap::new(),
        };

        for i in 0..cm.max_slot_id {
            cm.replicaset_map.insert(i, ReplicaSet::new(i));
        }
        cm
    }

    fn add_nodes(&mut self, names: &[&str]) {
        for name in names {
            self.node_map.insert(String::from(*name), Node::new(*name));
        }
    }

    pub fn allocate(&mut self, names: &[&str]) {
        self.add_nodes(names);
        let mut ns: Vec<String> = self.node_map.keys().cloned().collect();

        for i in 0..self.max_slot_id {
            for r in 0..3 {
                let name = ns[r].clone();
                if let Some(node) = self.node_map.get_mut(&name) {
                    node.pickup_slot(i, r as u32);
                    if let Some(replset) = self.replicaset_map.get_mut(&i) {
                        replset.assign(name.as_str(), r as u32);
                    }
                }
            }
            let key = ns.remove(0);
            ns.push(key);
        }
    }

    pub fn scale(&mut self, name: &str) {
        let old_nodes_num = self.node_map.len() as u64;
        let old_slots_num = self.max_slot_id / old_nodes_num;
        let new_nodes_num = old_nodes_num + 1;
        let new_slots_num = self.max_slot_id / new_nodes_num;

        let mut total_migrate_slot_set: HashSet<(u64, u32)> = HashSet::new();
        let mut total_primary_slot_num = 0;
        let mut total_secondary_slot_num = 0;

        for (_, node) in &mut self.node_map {
            let mut migrate_slot_set: HashSet<(u64, u32)> = HashSet::new();
            // migrate slots according the role
            for r in 0..3 {
                let mut i = new_slots_num;
                for (slot_id, role) in &node.slot_set {
                    if *role != r as u32 {
                        continue;
                    }
                    if total_migrate_slot_set.insert((*slot_id, *role)) {
                        if let Some(replset) = self.replicaset_map.get_mut(slot_id) {
                            replset.migrate(node.name.as_str());
                            migrate_slot_set.insert((*slot_id, *role));
                            i += 1;
                            if r == 0 {
                                total_primary_slot_num += 1;
                            } else {
                                total_secondary_slot_num += 1;
                            }
                        }
                    }
                    if i >= old_slots_num {
                        break;
                    }
                }
            }
            for (slot_id, role) in migrate_slot_set {
                node.slot_set.remove(&(slot_id, role));
            }
        }
        println!(
            "scale => migrate primary slots num: {}",
            total_primary_slot_num
        );
        println!(
            "scale => migrate secondary slots num: {}",
            total_secondary_slot_num
        );

        self.add_nodes(&[name]);
        if let Some(node_dst) = self.node_map.get_mut(name) {
            for (slot_id, role) in total_migrate_slot_set {
                if let Some(replset) = self.replicaset_map.get_mut(&slot_id) {
                    replset.assign(node_dst.name.as_str(), role);
                    node_dst.pickup_slot(slot_id, role);
                }
            }
        }
    }

    pub fn show_nodes(&self) {
        let mut total_primary_slots = 0;
        let mut total_secondary_slots = 0;
        let mut i = 0;
        for (_, node) in &self.node_map {
            println!("============= {} Node name: {}", i, node.name);
            i += 1;
            let mut primary_slots = 0;
            let mut secondary_slots = 0;
            for (slot_id, role) in &node.slot_set {
                //println!("slot: {}, role: {}", *slot_id, role);
                if *role == 0 {
                    primary_slots += 1;
                } else {
                    secondary_slots += 1;
                }
            }
            println!(
                "primary slots count: {}, secondary slots count: {}",
                primary_slots, secondary_slots,
            );
            total_primary_slots += primary_slots;
            total_secondary_slots += secondary_slots;
        }
        println!(
            "total primary slots count: {}, total secondary slots count: {}",
            total_primary_slots, total_secondary_slots
        );
    }
}

fn main() {
    let mut cm = ClusterManager::new(128);

    cm.allocate(&["aaa", "bbb", "ccc", "ddd", "eee", "fff"]);
    cm.show_nodes();

    cm.scale("ggg");
    cm.scale("hhh");
    cm.scale("iii");
    cm.show_nodes();
}
