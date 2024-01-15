
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

use ic_cdk_macros::{export_candid, query, update};

use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Deserialize, Serialize)]
struct Todo {
    text: String,
    completed: bool,
}

type TodoMap = BTreeMap<usize, Todo>;

thread_local! {
    static COUNTER: Cell<usize> = Cell::default();
    static TODOS: RefCell<TodoMap> = RefCell::default();
}

#[query]
fn get_todos(page: usize, offset: usize) -> Vec<Todo> {
    TODOS.with_borrow(|t| {
        t.values()
            .into_iter()
            .skip(page * offset)
            .take(offset)
            .map(Clone::clone)
            .collect::<Vec<_>>()
    })
}

#[query]
fn get_todo(id: usize) -> Option<Todo> {
    TODOS.with_borrow(|t| t.get(&id).cloned())
}

#[update]
fn update_todo(id: usize, todo: Todo) {
    TODOS.with_borrow_mut(|t| t.insert(id, todo));
}

#[update]
fn delete_todo(id: usize) {
    TODOS.with_borrow_mut(|t| t.remove(&id));
}

#[update]
fn add_todo(text: String) -> usize {
    let id = COUNTER.with(|c| {
        let next_id = c.get() + 1;
        c.set(next_id);
        next_id
    });
    TODOS.with_borrow_mut(|t| {
        t.insert(
            id,
            Todo {
                text,
                completed: false,
            },
        )
    });

    return id;
}

// For Generating Candid Interface
export_candid!();
