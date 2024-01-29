const fs = require('node:fs');

const input = fs.readFileSync("./input1.txt").toString();

const test = `0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45`;

function subTwobyTwo(listArray) {
    let subArray = [];
    for (let i = 1; i < listArray.length; i++) {
        const curr = listArray[i];
        const prev = listArray[i-1];
        subArray.push(curr-prev);
    }
    return subArray;
}

function recordHistory(record) {
    let history = [];
    history.push(record.map((x) => parseInt(x)));
    let subArray = subTwobyTwo(record);
    history.push(subArray);
    while(true) {
        if (subArray.every((s) => s ==  0)) {
            break;
        }
        subArray = subTwobyTwo(subArray);
        history.push(subArray);
    }
    console.log(history);
    return history;
}

function populateHistory(history) {
    let nextValue = 0;
    history[0] = history[0].concat([0]);
    for (let i = 1; i < history.length; i++) { 
        const curr_record = history[i];
        const prev_record = history[i-1];
        curr_record.push(+curr_record.slice(-1) + +prev_record.slice(-1));
        history[i] = curr_record;
        console.log("curr_record", curr_record);
        if (i == history.length - 1) {
            nextValue = curr_record.slice(-1);
            break;
        }
    }
    return nextValue;
}
function main(input) {

    let sum = 0;
    const records = input.split("\n");
    for (const record of records) {
        let history = recordHistory(record.split(" ")).reverse();
        sum += populateHistory(history)[0];
    }
    return sum;
}

const result1 = main(test);
console.log(result1);
const result2 = main(input);
console.log(result2);
