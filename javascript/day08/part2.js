2
const fs = require('node:fs');

let input = fs.readFileSync("./input.txt").toString();

const test = `LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)`;

const answer = 2;

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

    let currentNodes = nodes.filter((node) => node.main.endsWith('A'));
    let directionsLength = directions.length;
    let index, count = 0;

    while (true) {
        if (currentNodes.every((node) => node.main.endsWith('Z'))) {
            break;
        }

        index = count % directionsLength;
        const direction = directions[index];

        if (direction == "R") {
            currentNodes = currentNodes.flatMap((currentNode) => nodes.filter((node) => node.main === currentNode.rigth));
        } else {
            currentNodes = currentNodes.flatMap((currentnode) => nodes.filter((node) => node.main === currentnode.left));
        }

        count++;
    }

    return count;
}

const result1 = main(test);
const result = main(input);

console.log(result1);
console.log(result);
