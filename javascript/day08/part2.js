
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

    let currentNodes = nodes.filter((node) => node.main.slice(-1) === 'A');
    // console.log(currentNodes);
    let directionsLength = directions.length;
    let index, count = 0;

    while (true) {
        // console.log("currentNodes", currentNodes);
        if (currentNodes.every((node) => node.main.slice(-1) === 'Z')) {
            break;
        }

        index = count % directionsLength;
        const direction = directions[index];

        let nextNodes = [];
        if (direction == "R") {
            for (const currentNode of currentNodes) {
                // console.log("currentNode", currentNode);
                const nextNode = nodes.filter((node) => node.main === currentNode.rigth)[0];
                // console.log("nextNode", nextNode);
                nextNodes.push(nextNode);
            }
            currentNodes = nextNodes;
        } else {
            for (const currentNode of currentNodes) {
                // console.log("currentNode", currentNode);
                const nextNode = nodes.filter((node) => node.main === currentNode.left)[0];
                // console.log("nextNode", nextNode);
                nextNodes.push(nextNode);
            }
            // console.log("nextNodes", nextNodes);
            currentNodes = nextNodes;
        }

        count++;
    }

    return count;
}

const result1 = main(test);
const result = main(input);

console.log(result1);
console.log(result);
