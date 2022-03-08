#!/usr/bin/env node

console.log("Olá Mundo!")

function multiplier(x, y) {
    let value_one = Number(x);
    let value_two = Number(y);
    let result_one = isNaN(value_one)
    let result_two = isNaN(value_two)

    if (result_one || result_two) {
        console.error(`You provided a ${typeof x && typeof y}, must be a number`);
        process.exit(1);
    }
    let result = (x * y)
    return result
}

function print_string_to_upper(str) {
    if (typeof str != "string") {
        console.log(`Must be String, you provided ${typeof str}`)
        process.exit(1)
    }
    let cast = String(str)
    console.log(`${cast.toUpperCase()}`)
}

console.log(multiplier(2, 2))
print_string_to_upper("thiago")