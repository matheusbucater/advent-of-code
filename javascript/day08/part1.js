
const fs = require('node:fs');

let input = fs.readFileSync("./input.txt").toString();


const test1 = `RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)`;

const test2 = `LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)`;

const answer1 = 2;
const answer2 = 6;

class Node {
    constructor(main, left, right) {
        this.main = main;
        this.left = left;
        this.rigth = right;
    }
}

function createNodesArray(nodesInput) {
    let nodesArray = [];
    nodesInput.forEach((nodeInput) => {
        const main = nodeInput.slice(0,3);
        const left = nodeInput.slice(7,10);
        const rigth = nodeInput.slice(12,15);
        const nodeItem = new Node(main, left, rigth);
        nodesArray.push(nodeItem);
    });
    return nodesArray;
}


function main(input) {
    const [directions, _, ...nodesInput] = input.split("\n");
    const nodes = createNodesArray(nodesInput);

    let currentNode = nodes.filter((node) => node.main === 'AAA')[0];
    let directionsLength = directions.length;
    let index, count = 0;
    

    while (true) {
        if (currentNode.main === 'ZZZ') {
            break;
        }

        index = count % directionsLength;
        const direction = directions[index];

        if (direction == "R") {
            const nextNode = nodes.filter((node) => node.main === currentNode.rigth)[0];
            currentNode = nextNode;
        } else {
            const nextNode = nodes.filter((node) => node.main === currentNode.left)[0];
            currentNode = nextNode;
        }

        count++;
    }

    return count;
}

const result1 = main(test1);
const result2 = main(test2);
const result = main(input);

console.log(result1);
console.log(result2);
console.log(result);
