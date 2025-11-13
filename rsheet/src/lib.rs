// 导入
use rsheet_lib::cell_expr::{CellArgument, CellExpr, CellExprEvalError}; // 1. 导入 CellExprEvalError
use rsheet_lib::cell_value::CellValue;
use rsheet_lib::command::Command;
use rsheet_lib::connect::{
    Connection, Manager, ReadMessageResult, Reader, WriteMessageResult, Writer,
};
use rsheet_lib::replies::Reply;

// 2. 导入我们需要的两个辅助函数
use rsheet_lib::cells::{column_name_to_number, column_number_to_name};

use std::collections::HashMap;
use std::error::Error;

use log::info;

// 状态枚举 (不变)
enum StoredCell {
    Value(CellValue),
    DependsOnError,
}

// 3. --- 新增辅助函数 ---
//    这个函数从我们的 `spreadsheet` 中安全地获取一个*标量* (scalar) 值。
//    它返回 (值, 是否有依赖错误)
fn get_scalar_value(
    sheet: &HashMap<String, StoredCell>,
    id_str: &str,
) -> (CellValue, bool) {
    match sheet.get(id_str) {
        // 如果单元格是空的，返回 None，没有错误
        None => (CellValue::None, false),
        // 如果单元格有值，克隆它并返回，没有错误
        // (注意: `CellValue::Error` 只是一个值，不是依赖错误)
        Some(StoredCell::Value(value)) => (value.clone(), false),
        // 如果单元格本身依赖于一个错误，这*是*一个依赖错误
        Some(StoredCell::DependsOnError) => (CellValue::None, true),
    }
}

// 4. --- 新增辅助函数 ---
//    这个函数将 "A1" 这样的字符串解析回 (col, row) 索引
//    "A1" -> (0, 0)
//    "B2" -> (1, 1)
fn parse_cell_id(id_str: &str) -> (u32, u32) {
    // 找到第一个数字字符的位置
    let split_point = id_str
        .find(|c: char| c.is_ascii_digit())
        .unwrap_or(id_str.len());

    // 分割列 (例如 "A") 和行 (例如 "1")
    let col_str = &id_str[..split_point];
    let row_str = &id_str[split_point..];

    // 使用库函数将 "A" 转换为 0
    let col_idx = column_name_to_number(col_str);
    // 将 "1" 解析为 0-indexed 的 0
    let row_idx = row_str.parse::<u32>().unwrap() - 1;

    (col_idx, row_idx)
}

pub fn start_server<M>(mut manager: M) -> Result<(), Box<dyn Error>>
where
    M: Manager,
{
    let mut spreadsheet: HashMap<String, StoredCell> = HashMap::new();

    let (mut recv, mut send) = match manager.accept_new_connection() {
        Connection::NewConnection { reader, writer } => (reader, writer),
        Connection::NoMoreConnections => {
            return Ok(());
        }
    };
    loop {
        info!("Just got message");
        match recv.read_message() {
            ReadMessageResult::Message(msg) => {
                let write_result = match msg.parse::<Command>() {
                    Ok(command) => match command {
                        // --- GET 命令 (与 Stage 1 完全相同) ---
                        Command::Get { cell_identifier } => {
                            let col_name = column_number_to_name(cell_identifier.col as u32);
                            let row_num = cell_identifier.row + 1;
                            let id_str = format!("{}{}", col_name, row_num);

                            let stored_val = spreadsheet.get(&id_str);

                            let reply = match stored_val {
                                None => Reply::Value(id_str, CellValue::None),
                                Some(StoredCell::Value(value)) => {
                                    Reply::Value(id_str, value.clone())
                                }
                                Some(StoredCell::DependsOnError) => Reply::Error(
                                    "Cell depends on another cell with an error".to_string(),
                                ),
                            };
                            send.write_message(reply)
                        }

                        // 5. --- SET 命令 (已修改) ---
                        Command::Set {
                            cell_identifier,
                            cell_expr,
                        } => {
                            // 5a. 转换我们要设置的单元格 ID (例如 "A2")
                            let col_name = column_number_to_name(cell_identifier.col as u32);
                            let row_num = cell_identifier.row + 1;
                            let id_str = format!("{}{}", col_name, row_num);

                            // 5b. 构造 CellExpr 并找到它需要的所有变量名
                            let expr = CellExpr::new(&cell_expr);
                            let vars_needed = expr.find_variable_names();

                            // 5c. 构建 `evaluate` 所需的 `deps` HashMap
                            let mut deps: HashMap<String, CellArgument> = HashMap::new();
                            let mut has_dependency_error = false;

                            for var_name in vars_needed {
                                // `var_name` 可能是 "A1" 或 "A1_B3"
                                let parts: Vec<&str> = var_name.split('_').collect();

                                match parts.as_slice() {
                                    // --- Case 1: 标量 (Scalar), "A1" ---
                                    [scalar_id] => {
                                        let (value, is_err) =
                                            get_scalar_value(&spreadsheet, scalar_id);
                                        if is_err {
                                            has_dependency_error = true;
                                            break; // 发现一个错误，停止收集依赖
                                        }
                                        deps.insert(var_name, CellArgument::Value(value));
                                    }

                                    // --- Case 2: 范围 (Range), "A1_B3" ---
                                    [start_id, end_id] => {
                                        let (c1, r1) = parse_cell_id(start_id);
                                        let (c2, r2) = parse_cell_id(end_id);

                                        let mut rows_data: Vec<Vec<CellValue>> = Vec::new();
                                        let mut range_has_error = false;

                                        // 遍历所有行 (r1 到 r2)
                                        for r_idx in r1..=r2 {
                                            let mut cols_data: Vec<CellValue> = Vec::new();
                                            // 遍历所有列 (c1 到 c2)
                                            for c_idx in c1..=c2 {
                                                // 重新构建单元格 ID (例如 "A1", "B1", "A2", "B2"...)
                                                let dep_col_name = column_number_to_name(c_idx);
                                                let dep_id_str =
                                                    format!("{}{}", dep_col_name, r_idx + 1);

                                                let (value, is_err) =
                                                    get_scalar_value(&spreadsheet, &dep_id_str);
                                                
                                                if is_err {
                                                    range_has_error = true;
                                                    break;
                                                }
                                                cols_data.push(value);
                                            }
                                            if range_has_error { break; }
                                            rows_data.push(cols_data);
                                        }

                                        if range_has_error {
                                            has_dependency_error = true;
                                            break;
                                        }
                                        
                                        // 5d. 区分 Vector 和 Matrix
                                        if c1 == c2 { // 垂直向量 (A1_A3)
                                            let vector = rows_data.into_iter().map(|mut row| row.pop().unwrap()).collect();
                                            deps.insert(var_name, CellArgument::Vector(vector));
                                        } else if r1 == r2 { // 水平向量 (A1_C1)
                                            let vector = rows_data.pop().unwrap();
                                            deps.insert(var_name, CellArgument::Vector(vector));
                                        } else { // 矩阵 (A1_B3)
                                            deps.insert(var_name, CellArgument::Matrix(rows_data));
                                        }
                                    }
                                    // 忽略无效的变量名格式 (例如 "A1_B2_C3")
                                    _ => {}
                                }
                            }

                            // 5e. 存储结果
                            if has_dependency_error {
                                // 如果任何依赖项是 `DependsOnError`，则此单元格也变为 `DependsOnError`
                                spreadsheet.insert(id_str, StoredCell::DependsOnError);
                            } else {
                                // 否则，我们调用 `evaluate`
                                match expr.evaluate(&deps) {
                                    Ok(value) => {
                                        // 存储计算出的值 (这也可能是 `CellValue::Error`)
                                        spreadsheet.insert(id_str, StoredCell::Value(value));
                                    }
                                    Err(_) => {
                                        // `evaluate` 失败意味着一个依赖项是 `CellValue::Error`
                                        spreadsheet.insert(id_str, StoredCell::DependsOnError);
                                    }
                                }
                            }
                            // `set` 命令不回复
                            WriteMessageResult::Ok
                        }
                    },
                    Err(e) => send.write_message(Reply::Error(e)),
                };

                // (循环的其余部分保持不变)
                match write_result {
                    WriteMessageResult::Ok => {}
                    WriteMessageResult::ConnectionClosed => {
                        break;
                    }
                    WriteMessageResult::Err(e) => {
                        return Err(Box::new(e));
                    }
                }
            }
            ReadMessageResult::ConnectionClosed => {
                break;
            }
            ReadMessageResult::Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    Ok(())
}