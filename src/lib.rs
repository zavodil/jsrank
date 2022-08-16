use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::collections::LazyOption;

use near_sdk::{env, near_bindgen, base64, AccountId, BorshStorageKey, PanicOnDefault, log};
use near_sdk::borsh::{self, BorshDeserialize,BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};

mod jslib;
mod wasimock;
mod nft;
mod js;

use nft::*;

type TaskId = u32;
type CategoryId = u32;
type Point = u16;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tasks: UnorderedMap<TaskId, Task>,
    task_categories: UnorderedMap<CategoryId, String>,
    complete_tasks: LookupMap<AccountId, Vec<TaskId>>,
    points: LookupMap<AccountId, Point>,
    next_task_id: TaskId,
    next_category_id: TaskId,
    owner_id: AccountId,

    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Task {
    title: String,
    category_id: CategoryId,
    description: String,
    parameters: String,
    tests: Vec<TaskTest>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskOutput {
    task_id: TaskId,
    category_id: CategoryId,
    title: String,
    description: String,
    parameters: String,
    category: String
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct TaskTest {
    input: String,
    output: String
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Tasks,
    TaskCategories,
    CompleteTasks,
    Points,

    Metadata
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            tasks: UnorderedMap::new(StorageKey::Tasks),
            task_categories: UnorderedMap::new(StorageKey::TaskCategories),
            complete_tasks: LookupMap::new(StorageKey::CompleteTasks),
            points: LookupMap::new(StorageKey::Points),
            next_task_id: 0,
            next_category_id: 0,
            owner_id,
            metadata: LazyOption::new(StorageKey::Metadata, Some(&get_default_meta())),
        }
    }

    pub fn get_available_tasks(&self, account_id: Option<AccountId>) -> Vec<TaskOutput> {
        let completed_tasks = if let Some(account_id) = account_id {
            self.complete_tasks.get(&account_id).unwrap_or_default()
        }
        else {
            vec![]
        };

        self.tasks.keys_as_vector()
            .iter()
            .filter(|task_id| !completed_tasks.contains(task_id))
            .map(|task_id| self.get_task_output(task_id, self.tasks.get(&task_id).unwrap()))
            .collect()
    }

    pub fn get_points(&self, account_id: AccountId) -> Point {
        self.points.get(&account_id).unwrap_or_default()
    }

    pub fn update_task(&mut self, task_id: TaskId,  title: Option<String>, description: Option<String>,
                       parameters: Option<String>){
        self.assert_owner();

        let mut task = self.tasks.get(&task_id).expect("ERR_TASK_NOT_FOUND");

        if let Some (title) = title {
            task.title = title;
        }
        if let Some (description) = description {
            task.description = description;
        }
        if let Some (parameters) = parameters {
            task.parameters = parameters;
        }

        self.tasks.insert(&task_id, &task);
    }

    pub fn add_task(&mut self, category_id: CategoryId, title: String, description: String, parameters: String, tests: Vec<(String, String)>) -> TaskId {
        self.assert_owner();
        let tests = tests
            .iter()
            .map(|(input, output)| TaskTest {input: input.clone(), output: output.clone()})
            .collect();

        self.tasks.insert(&self.next_task_id, &Task {
            title,
            category_id,
            description,
            parameters,
            tests
        });
        self.next_task_id += 1;
        self.next_task_id
    }

    pub fn get_task(&self, task_id: TaskId) -> TaskOutput {
        self.get_task_output(task_id, self.tasks.get(&task_id).expect("ERR_TASK_NOT_FOUND"))
    }

    pub fn get_category(&self, category_id: CategoryId) -> Option<String> {
        self.task_categories.get(&category_id)
    }

    pub fn add_category(&mut self, category: String) -> CategoryId {
        self.assert_owner();
        self.task_categories.insert(&self.next_category_id, &category);
        self.next_category_id += 1;

        self.next_category_id
    }
}


impl Contract {
    pub fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "ERR_NOT_OWNER");
    }

    pub fn get_task_output(&self, task_id: TaskId, task: Task) -> TaskOutput {
        let category = self.task_categories.get(&task.category_id).unwrap_or_default();
        TaskOutput {
            task_id,
            category_id: task.category_id,
            title: task.title,
            description: task.description,
            parameters: task.parameters,
            category
        }
    }

    pub fn internal_test_code(&self, script: &String, first_line: &String, input: &String, output: &String) -> bool {
        let user_function = format!("(function({}) {{ {} }})({})", first_line, script, input);

        let code: String = format!("(function() {{return ( {}  == {}); }})();", user_function, output);

        let res: String = self.run_script(code);

        res == "1"
    }
}

