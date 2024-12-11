function safe(report) {
    const asc = report.every((num, i) => {
        if (i === 0) return true;
        const diff = num - report[i - 1];
        return diff >= 1 && diff <= 3;
    });

    const desc = report.every((num, i) => {
        if (i === 0) return true;
        const diff = report[i - 1] - num;
        return diff >= 1 && diff <= 3;
    });

    return asc || desc

}
function part_1(matrix) {
    let total = 0
    matrix.forEach((report) => {
        if (sefe(report)) {
            total += 1;
        }
    });
    return total;
}

function part_2(matrix) {
    let total = 0
    matrix.forEach((report) => {
        if (safe(report)) {
            total += 1
            return
        }
        for (let i = 0; i < report.length; i++) {
            const newReport = [...report.slice(0, i), ...report.slice(i + 1)]

            if (safe(newReport)) {
                total += 1
                return

            }


        }
    });
    return total;
}

/**
    * @returns {number[][]}
    *
    *
    */
function read_file() {
    const fs = require('fs')
    let matrix = [[]]

    try {
        const content = fs.readFileSync('sample.txt', 'utf8');
        // console.log(content)
        const reports = content.trim('\n').split('\n')
        reports.forEach((report, i) => {
            matrix[i] = report.split(' ')
        })



        return matrix


    } catch (error) {
        console.error(error)

    }

}


const matrix = read_file()
console.log(part_2(matrix))



