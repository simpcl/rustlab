use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Debug)]
enum Role {
    PRIMARY,
    SECONDARY,
}

#[derive(Debug)]
struct Slot {
    id: u64,
    role: Role,
}

#[derive(Debug)]
struct VNode {
    id: u64,
    node_name: Option<String>,
    slots: Vec<Slot>,
}

#[derive(Debug)]
struct Node {
    name: String,
    vnode_set: HashSet<u64>,
}

#[derive(Debug)]
struct ClusterManager {
    max_vnode_id: u64,
    max_slot_id: u64,
    nodes: HashMap<String, Node>,
    vnodes: Vec<VNode>,
}

impl VNode {
    fn new(id: u64) -> VNode {
        VNode {
            id: id,
            node_name: None,
            slots: Vec::new(),
        }
    }

    fn assign(&mut self, node_name: Option<String>) {
        self.node_name = node_name;
    }
}

impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: String::from(name),
            vnode_set: HashSet::new(),
        }
    }

    fn pickup_vnode(&mut self, vnode_id: u64) {
        self.vnode_set.insert(vnode_id);
    }

    fn drop_vnode(&mut self, vnode_id: u64) {
        self.vnode_set.remove(&vnode_id);
    }
}

impl ClusterManager {
    fn new() -> ClusterManager {
        ClusterManager {
            max_vnode_id: 1024,
            max_slot_id: 16,
            nodes: HashMap::new(),
            vnodes: Vec::new(),
        }
    }

    fn init_vnodes(&mut self, vnode_num: u64) {
        self.max_vnode_id = vnode_num;
        for vnode_id in (0..self.max_vnode_id) {
            let vnode = VNode::new(vnode_id);
            self.vnodes.push(vnode);
        }
    }

    fn init_slots(&mut self, slot_num: u64) -> bool {
        if self.vnodes.len() <= 0 {
            return false;
        }
        self.max_slot_id = slot_num;

        let mut dh = DefaultHasher::new();

        for slot_id in (0..self.max_slot_id) {
            slot_id.hash(&mut dh);
            //let vnode_id = dh.finish() % self.max_vnode_id;
            let hc = dh.finish() % 2u64.pow(32);
            let vnode_id = hc / (2u64.pow(32) / self.max_vnode_id);

            let slot = Slot {
                id: slot_id,
                role: Role::PRIMARY,
            };
            self.vnodes[vnode_id as usize].slots.push(slot);
            let slot = Slot {
                id: slot_id,
                role: Role::SECONDARY,
            };
            let vnode_id = (vnode_id + 1) % self.max_vnode_id;
            self.vnodes[vnode_id as usize].slots.push(slot);
            let slot = Slot {
                id: slot_id,
                role: Role::SECONDARY,
            };
            let vnode_id = (vnode_id + 1) % self.max_vnode_id;
            self.vnodes[vnode_id as usize].slots.push(slot);
        }
        return true;
    }

    fn add_nodes(&mut self, names: &[&str]) {
        for name in names {
            self.nodes.insert(String::from(*name), Node::new(*name));
        }
    }

    pub fn allocate(&mut self, names: &[&str]) {
        self.add_nodes(names);
        let mut rnd = rand::thread_rng();
        let mut ns: Vec<String> = self.nodes.keys().cloned().collect();

        for i in (0..self.max_vnode_id) {
            let ri = rnd.gen_range(0, ns.len() - 2) as usize;
            let key = ns.remove(ri);
            let name = key.clone();
            ns.push(key);
            if let Some(node) = self.nodes.get_mut(&name) {
                //println!("node {} pick up vnode {} ==> ns {:?}", index, vnode_id, ns);
                node.pickup_vnode(i);
                self.vnodes[i as usize].node_name = Some(name);
            }
        }
    }

    pub fn scale(&mut self, name: &str) {
        let old_nodes_num = self.nodes.len() as u64;
        self.add_nodes(&[name]);
        let new_nodes_num = self.nodes.len() as u64;
        let vnodes_num_per_node = self.max_vnode_id / new_nodes_num;

        let mut rnd = rand::thread_rng();
        let vnodes_num = self.max_vnode_id;
        let mut indexes = Vec::new();

        // select vnodes_num_per_node vnodes and remove old node name
        for i in (0..vnodes_num_per_node) {
            let index = rnd.gen_range(0, vnodes_num) as usize;
            let node_name_src = self.vnodes[index]
                .node_name
                .as_ref()
                .map_or("", String::deref);
            if node_name_src.len() > 0 {
                if let Some(node_src) = self.nodes.get_mut(node_name_src) {
                    node_src.drop_vnode(index as u64);
                }
            }
            indexes.push(index);
        }

        println!(
            "scale => vnodes_num_per_node: {}, indexes: {}",
            vnodes_num_per_node,
            indexes.len()
        );
        // reassign vnodes_num_per_node vnodes to the new node
        if let Some(node_dst) = self.nodes.get_mut(&String::from(name)) {
            for index in &indexes {
                node_dst.pickup_vnode(*index as u64);
                //println!("scale: pickup vnode {}", index);
                self.vnodes[*index].node_name = Some(String::from(name));
            }
            println!(
                "scale => dst node vnode set size: {}",
                node_dst.vnode_set.len()
            );
        }
    }

    pub fn show_vnodes(&self) {
        for vnode_id in (0..self.vnodes.len()) {
            if self.vnodes[vnode_id].slots.len() > 0 {
                println!("vnode id {}: {:?}", vnode_id, self.vnodes[vnode_id].slots);
            }
        }
    }

    pub fn show_nodes(&self) {
        let mut i = 0;
        let mut total_vnodes = 0;
        let mut total_slots = 0;
        for (_, node) in &self.nodes {
            i += 1;
            let mut primary_slots = 0;
            let mut secondary_slots = 0;
            total_vnodes += node.vnode_set.len();
            println!(
                "============= Node {} => name: {}, vnodes count: {}",
                i,
                node.name,
                node.vnode_set.len()
            );
            for vnode_id in &node.vnode_set {
                let vnode_index = *vnode_id as usize;
                if self.vnodes[vnode_index].slots.len() > 0 {
                    for slot in &self.vnodes[vnode_index].slots {
                        match slot.role {
                            Role::PRIMARY => primary_slots += 1,
                            Role::SECONDARY => secondary_slots += 1,
                        }
                    }
                    //println!("{:?}", self.vnodes[vnode_index].slots);
                }
            }
            println!(
                "Slots: primary {}, secondary {}",
                primary_slots, secondary_slots
            );
            total_slots += primary_slots + secondary_slots;
        }

        println!(
            "total_vnodes: {}, total_slots {}",
            total_vnodes, total_slots
        );
    }
}

fn main() {
    let mut cm = ClusterManager::new();

    cm.init_vnodes(65536);
    cm.init_slots(128);
    cm.show_vnodes();

    cm.allocate(&["a", "b", "c", "d", "e", "f"]);
    cm.show_nodes();

    cm.scale("ggg");
    cm.scale("hhh");
    cm.show_nodes();
}
