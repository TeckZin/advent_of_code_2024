function part1(lst1, lst2) {
    lst1.sort()
    lst2.sort()


    let diffList = []

    for (const i in lst1) {
        diffList[i] = Math.abs(lst1[i] - lst2[i])

    }

    return diffList.reduce((sum, curr) => sum += curr, 0)


}




function readFile() {

    const fs = require('fs');

    let left = []
    let right = []

    try {
        const content = fs.readFileSync('sample.txt', 'utf8');
        console.log(content);
        content.split('\n').forEach(line => {
            // Split by any number of spaces and filter out empty strings
            const [leftNum, rightNum] = line.trim().split(/\s+/);
            left.push(Number(leftNum));
            right.push(Number(rightNum));
        });
    } catch (err) {
        console.error('Error reading file:', err);
    }

    // return left, right

    return [left.slice(0, -1), right.slice(0, -1)]

}

const [lst1, lst2] = readFile()
console.log(lst1, lst2)

// const lst1 = [5, 3, 2]
// const lst2 = [4, 7, 8]

// console.log(part1(lst1, lst2))

function part2(lst1, lst2) {
    let sum = 0
    const mapCounts = lst2.reduce((counts, curr) => {
        counts[curr] = (counts[curr] || 0) + 1
        return counts
    }, {})

    console.log(mapCounts)


    for (const x of lst1) {
        if (mapCounts[x]) {
            sum += mapCounts[x] * x
        }



    }
    return sum


}

console.log(part2(lst1, lst2))
