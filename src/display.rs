use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

use crate::index_set::IndexSet;

pub fn print_entry(domain: &str, username: &str, password: &str) {
    let table = vec![vec![
        domain.cell(),
        username.cell(),
        password.cell().justify(Justify::Right),
    ]]
    .table()
    .title(vec![
        "Domain".cell(),
        "Username".cell(),
        "Password".cell().justify(Justify::Right),
    ])
    .bold(true);

    let _ = print_stdout(table);
}

pub fn print_index(set: &IndexSet) {
    let mut result = Vec::new();

    for (index, item) in set.data.iter().enumerate() {
        let vec = vec![index.cell(), item.key.clone().cell()];
        result.push(vec);
    }

    let _ = print_stdout(result);
}
