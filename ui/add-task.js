const near = require("./near");
const {getConfig} = require("./frontend/near-config");
const fs = require("fs");

const owner_id = "jsrank.near";
const nearConfig = getConfig(process.env.NODE_ENV || "mainnet");

async function loadTask(filename) {
    return fs.readFileSync(`./tasks/${filename}.txt`, 'utf8');
}

function addTask(category_id, taskName, parameters, tests) {
    loadTask(taskName)
        .then(async content => {
            const resp = await near.NearCall(
                owner_id,
                nearConfig.contractName,
                "add_task",
                {
                    category_id,
                    title: taskName,
                    description: content,
                    parameters,
                    tests

                }
            );
        });
}

addTask(0, "Repeated String", "s, n", [["\"aba\", 10", "7"], ["\"a\", 1000000000000", "1000000000000"], ["\"abba\", 88", "44"], ["\"nevergiveup\", 100", "0"], ["\"tryagain\", 25", "6"], ["\"aaaaaaaaaaaa\", 5", "5"]]);

addTask(0, "Sales by Match", "n, ar", [["9, [10, 20, 20, 10, 10, 30, 50, 10, 20]", "3"], ["10, [1, 1, 3, 1, 2, 1, 3, 3, 3, 3]", "4"], ["6, [1, 1, 1, 1, 1, 1]", "3"], ["17, [1, 1, 5, 5, 5, 3, 5, 1, 2, 5, 1, 3, 3, 3, 3, 4, 5]", "7"], ["8, [1, 2, 3, 4, 5, 6, 7, 8]", "0"]]);

addTask(0, "Counting Valleys", "steps, path", [["8, \"UDDDUDUU\"", "1"], ["12, \"DDUUDDUDUUUD\"", "2"], ["9, \"DDDDDDDDD\"", "1"], ["9, \"UUUUUUUUU\"", "0"],  ["25, \"DDUUUDDDDUUUUDDDDDDUUUUUU\"", "3"],  ["28, \"DDUUDDUUDDDUUUDDDUUUDDUUDDUU\"", "6"]]);