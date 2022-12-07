const fs = require('fs');
const data = fs.readFileSync('my.txt', 'utf8');

class Directory {
    constructor(name, parent) {
        this.name = name;
        this.files = []
        this.dirs = []
        this.parent = parent
    }

    get size() {
        return this.files.map(v => v.size).reduce((a, b) => a+b, 0) + this.dirs.map(v => v.size).reduce((a, b) => a+b, 0)
    }
}

class XFile {
    constructor(name, size) {
        this.name = name;
        this.size = size;
    }
}

const root = new Directory("/", null)
let curr = root;
let nodes = [];
function mkdir(dirName) {
    const maybe = curr.dirs.find(d => d.name == dirName)
    if (maybe) return maybe;

    const next = new Directory(dirName, curr);
    nodes.push(next)
    curr.dirs.push(next);
    return next
}
data.split('\r\n').forEach((cmd, i) => {
    console.log("Processing... ", cmd)
    if (cmd.indexOf("$ cd") == 0) {
        dir = cmd.split(" ")[2]
        console.log("CD", dir)
        if (dir == "/") {
            curr = root;
        } else if (dir == "..") {
            curr = curr.parent;
        } else {
            curr= mkdir(dir);
        }
    } else if (cmd.indexOf("$ ls") == 0) {
        console.log("LS", dir)
    } else {
        const parts = cmd.split(" ");
        if (parts[0] == "dir") {
            mkdir(parts[1])
        } else {
            const file = new XFile(parts[1], parseInt(parts[0]));
            if (file.size) {
                curr.files.push(file)
                nodes.push(file)
            }
        }
    }
})

function find_below(cap) {
    return nodes.map(n => n.size <= cap ? n.size : 0).reduce((a, b) => a+b)
}

function find_node_above(cap) {
    return nodes.map(node => node.size).sort((a, b) => a-b).filter(size => size >= cap)[0]
}

const total_disk_space = 70000000;
const free_required = 30000000;
const unused = total_disk_space-root.size;
const del = free_required - unused;
console.log(root.size, find_node_above(del), find_below(100000))