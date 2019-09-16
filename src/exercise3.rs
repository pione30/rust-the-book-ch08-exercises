use std::collections::HashMap;

pub fn add_an_employee(payroll: &mut HashMap<String, Vec<String>>, query: String) -> Result<&mut HashMap<String, Vec<String>>, String> {
    let split_query: Vec<&str> = query.split(' ').collect();

    if split_query.len() < 4 || split_query[0] != "Add" || split_query[2] != "to" {
        return Err("Query must be like 'Add <Name> to <Department>'".to_string());
    }

    let employee_list = payroll.entry(split_query[3].to_string()).or_insert(Vec::new());
    employee_list.push(split_query[1].to_string());
    employee_list.sort();

    Ok(payroll)
}
