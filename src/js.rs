use crate::*;

#[near_bindgen]
impl Contract {
    pub fn run_script(&self, script: String) -> String {
        return jslib::run_js(script).to_string();
    }

    pub fn run_bytecode(&self, bytecodebase64: String) -> String {
        let bytecode:Result<Vec<u8>, base64::DecodeError> = base64::decode(&bytecodebase64);
        return jslib::run_js_bytecode(bytecode.unwrap()).to_string();
    }

    // returns (all_tests, wrong_tests)
    pub fn test_code(&mut self, task_id: TaskId, code: String) -> (usize, usize) {
        let user = &env::predecessor_account_id();
        let mut complete_tasks = self.complete_tasks.get(user).unwrap_or_default();
        let task = self.tasks.get(&task_id).expect("ERR_TASK_NOT_FOUND");
        assert!(!complete_tasks.contains(&task_id), "ERR_TASK_ALREADY_COMPLETED");
        let points: usize = task.tests
            .iter()
            .map(| test |
                if self.internal_test_code(&code, &task.parameters, &test.input, &test.output) {
                    1
                }
                else {
                    0
                }
            )
            .sum();

        let all_tests = task.tests.len();
        let wrong_results = all_tests - points;
        let result = wrong_results == 0;

        let owner_id = user.to_string();

        if result {
            let points = self.points.get(user).unwrap_or_default();
            if points == 0 {
                log!( r#"EVENT_JSON:{{"standard":"nep171","version":"1.0.0","event":"nft_mint","data":[{{"owner_id":"{}","token_ids":["{}"]}}]}}"#,
                owner_id, owner_id);
            }

            self.points.insert(user, &(points + 1));
            complete_tasks.push(task_id);
            self.complete_tasks.insert(user, &complete_tasks);

        }

        (all_tests, wrong_results)
    }
}