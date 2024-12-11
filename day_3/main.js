function part_1(content) {
    console.log(content.length)
    let sum = 0
    let enable = true
    content.split('').forEach((char, i) => {
        if (i + 8 >= content.length) {
            return
        }
        // console.log(i)
        if (char === 'm' && content[i + 1] === 'u' && content[i + 2] === 'l'
            && content[i + 3] === '(') {

            console.log('start here')
            let data = content.slice(i, i + 12)

            const closeFlag = data.includes(')')
            const indexClose = data.indexOf(')')
            const commaFlag = data.includes(',')
            if (!closeFlag || !commaFlag) {
                return
            }
            data = data.slice(0, indexClose + 1)
            // next 1 to 3 is number

            console.log(data)
            const numData = data.slice(4, -1)
            const twoNum = numData.split(',')
            console.log(twoNum)
            const flag = twoNum.every((curr) => {
                return curr.length <= 3 && !isNaN(curr) && !curr.includes(' ')
            }
            )
            if (flag && enable) {
                sum += Number(twoNum[0]) * Number(twoNum[1])



            }

        } else if (char === 'd') {
            const doFunc = content.slice(i, i + 4)
            const dontFunc = content.slice(i, i + 7)
            if (doFunc === 'do()') {
                enable = true

            } else if (dontFunc === "don't()") {
                enable = false

            }
            console.log(doFunc)
            console.log(dontFunc)

        }

    })
    return sum



}

function part_2(contnet) {

}

function read_file() {
    const fs = require('fs')
    try {
        const content = fs.readFileSync('sample.txt', 'utf8')

        return content
    } catch (err) {
        console.error(err)
    }
    return null



}

const string_file = read_file()
console.log(string_file)
console.log(part_1(string_file))
