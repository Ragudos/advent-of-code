const fs_promises = require("node:fs/promises");
const path = require("node:path");

const path_to_txt = path.join(__dirname, "day1.txt");

fs_promises.readFile(path_to_txt, { encoding: "utf-8" })
	.then(parse_puzzle_input_first_part)
	.catch((err) => {
		console.error(err);
	});

/**
 * @param {string} contents
 */
function parse_puzzle_input_first_part(contents) {
	const array_of_input = contents.split("\r\n");
	
	// first_part_parser(array_of_input);
	 second_part_parser(array_of_input);
}

/**
 * @param {string[]} array_of_input
 */
function second_part_parser(array_of_input) {
	const word_digit_map = {
		"one": "1",
		"two": "2",
		"three": "3",
		"four": "4",
		"five": "5",
		"six": "6",
		"seven": "7",
		"eight": "8",
		"nine": "9",
	};

	let sum = 0;
	const first_ten = "one|two|three|four|five|six|seven|eight|nine";
	const regex = new RegExp(`${first_ten}`, "g");

	console.log(array_of_input[0].split(regex));

	 console.log(sum);
}

/**
 * @param {string[]} array_of_input
 */
function first_part_parser(array_of_input) {
	let sum = 0;

	for (const current_input of array_of_input) {
		let stopping_point_of_first_loop = 0;
		let first_digit;
		let second_digit;

		for (let idx = 0; idx < current_input.length; ++idx) {
			const current_char = current_input[idx];

			if (!isNaN(current_char)) {
				first_digit = current_char;
				stopping_point_of_first_loop = idx;
				break;
			}
		}

		// No digit was found
		if (first_digit == undefined) {
			continue;
		}

		for (let idx = current_input.length - 1; idx >= stopping_point_of_first_loop; --idx) {
			const current_char = current_input[idx];

			// only one digit exists
			if (idx == stopping_point_of_first_loop) {
				second_digit = first_digit;
				break;
			} else if (!isNaN(current_char)) {
				second_digit = current_char;
				break;
			}
		}

		sum += parseInt(first_digit + second_digit);
	}
	
	// console.log(sum);
}
